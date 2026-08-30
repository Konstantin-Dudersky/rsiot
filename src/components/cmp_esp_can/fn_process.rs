use std::time::Duration;

use enumset::EnumSet;
use esp_idf_svc::{
    hal::can::{self, Alert, AsyncCanDriver, CanDriver},
    sys::{esp, twai_initiate_recovery},
};
use tokio::{
    sync::mpsc::{Receiver, Sender, error::TryRecvError},
    task::JoinSet,
    time::timeout,
};
use tracing::{info, warn};

use crate::{
    components::shared_tasks::cmp_can_general::CanGeneralTasks,
    components_config::can_general::{BufferBound, CanFrame},
    executor::{Instant, MsgBusLinker},
    message::MsgDataBound,
};

use super::{Config, Error, can_filter::can_filter_convert};

pub async fn fn_process<TMsg, TBuffer>(
    config: Config<TMsg, TBuffer>,
    msgbus_linker: MsgBusLinker<TMsg>,
) -> super::Result<()>
where
    TMsg: 'static + MsgDataBound,
    TBuffer: 'static + BufferBound,
{
    let mut task_set: JoinSet<Result<(), Error>> = JoinSet::new();

    // Общие задачи обмена по шине CAN
    let (mut ch_rx_send_to_can, ch_tx_recv_from_can) = CanGeneralTasks {
        msgbus_linker,
        buffer_default: config.buffer_default,
        task_set: &mut task_set,
        fn_input: config.fn_input,
        period: config.period,
        fn_periodic: config.fn_periodic,
        fn_output: config.fn_output,
        error_task_end_input: || Error::TaskEndInput,
        error_task_end_output: || Error::TaskEndOutput,
        error_tokio_mpsc_send: || Error::TokioSyncMpscSend,
    }
    .spawn();

    // Фильтр
    let filter = can_filter_convert(&config.filters)?;

    // Скорость
    let timing = config.can_settings.bitrate.into();

    // Выбираем ошибки, которые драйвер будет выдавать при работе
    //
    // Из полного перечня исключены:
    // - Alert::AboveErrorWarning
    // - Alert::BelowErrorWarning
    // - Alert::PeripheralReset - создаётся очень много сообщений
    // - Alert::Success - возникает при успешной отправке - нет необходимости
    // - Alert::TransmitIdle - возникает после отправки - нет необходимости
    //
    // Описание ошибок в документации:
    // https://docs.espressif.com/projects/esp-idf/en/v4.2/esp32/api-reference/peripherals/twai.html
    let mut alerts = EnumSet::new();
    alerts.insert(Alert::ActiveError); // Состояние ERROR ACTIVE
    alerts.insert(Alert::AlertAndLog);
    alerts.insert(Alert::ArbLost);
    alerts.insert(Alert::BusError);
    alerts.insert(Alert::BusOffline); // Состояние BUS OFF
    alerts.insert(Alert::BusRecovered);
    alerts.insert(Alert::ErrorPass); // Состояние ERROR PASSIVE
    alerts.insert(Alert::ReceiveFifoOverflow);
    alerts.insert(Alert::ReceiveQueueFull);
    alerts.insert(Alert::Received);
    alerts.insert(Alert::RecoveryInProgress);
    alerts.insert(Alert::TransmitFailed);
    alerts.insert(Alert::TransmitRetried);

    // Настройка драйвера CAN
    let can_driver_config = can::config::Config::new()
        .filter(filter)
        .timing(timing)
        .rx_queue_len(100)
        .tx_queue_len(100)
        .alerts(alerts);

    let mut can_driver =
        AsyncCanDriver::new(config.can, config.pin_tx, config.pin_rx, &can_driver_config)
            .map_err(Error::DriverCreate)?;

    can_driver_start(&mut can_driver)?;

    loop {
        // Получение кадров
        step_receive(
            &mut can_driver,
            &ch_tx_recv_from_can,
            Duration::from_millis(200),
        )
        .await?;

        // Отправка кадров
        step_transmit(
            &mut can_driver,
            &mut ch_rx_send_to_can,
            Duration::from_millis(200),
        )
        .await?;

        // Проверка ошибок
        // TODO - ESP секунд через 10 отваливается
        // step_read_alerts(&mut can_driver, Duration::from_millis(20)).await?;
    }
}

/// Шаг получения кадров
async fn step_receive<'a>(
    can_driver: &mut AsyncCanDriver<'a, CanDriver<'a>>,
    ch_tx_recv_from_can: &Sender<CanFrame>,
    max_time: Duration,
) -> Result<(), Error> {
    let start_time = Instant::now();
    loop {
        // Если суммарное время превышено, переходим к следующему шагу
        if start_time.elapsed() >= max_time {
            break;
        }

        // Ожидаем получение кадра с таймаутом
        let result = timeout(max_time, can_driver.receive()).await;
        let frame = match result {
            Ok(Ok(frame)) => frame,
            Ok(Err(e)) => {
                warn!("Error receiveing CAN frame: {:?}", e);
                break;
            }
            Err(_) => break,
        };

        // Конвертируем данные
        let frame: Result<CanFrame, _> = frame.try_into();
        let Ok(frame) = frame else {
            warn!("Error converting CAN frame: {:?}", frame);
            continue;
        };

        // Отправка полученного кадра для дальнейшей обработки
        let res = ch_tx_recv_from_can.try_send(frame);
        if let Err(err) = res {
            warn!("Output channel full: {err}");
        }
    }

    Ok(())
}

/// Шаг отправки кадров
async fn step_transmit<'a>(
    can_driver: &mut AsyncCanDriver<'a, CanDriver<'a>>,
    ch_rx_send_to_can: &mut Receiver<CanFrame>,
    max_time: Duration,
) -> Result<(), Error> {
    let start_tx = Instant::now();

    loop {
        // Если суммарное время превышено, переходим к следующему шагу
        if start_tx.elapsed() >= max_time {
            break;
        }

        // Проверяем наличие кадров для отправки
        let frame = ch_rx_send_to_can.try_recv();
        let frame = match frame {
            Ok(val) => val,
            Err(err) => match err {
                TryRecvError::Empty => break,
                TryRecvError::Disconnected => todo!(),
            },
        };

        // Конвертируем данные
        let frame_conv: Result<esp_idf_svc::hal::can::Frame, _> = frame.try_into();
        let Ok(frame) = frame_conv else {
            warn!("Error converting CAN frame: {:?}", frame);
            continue;
        };

        // Отправка кадра
        let res = can_driver.transmit(&frame).await;
        if let Err(err) = res {
            warn!("Error transmitting CAN frame: {:?}", err);
        }
    }
    Ok(())
}

/// Шаг проверки сообщений и перезагрузка адаптера
async fn step_read_alerts<'a>(
    can_driver: &mut AsyncCanDriver<'a, CanDriver<'a>>,
    max_time: Duration,
) -> Result<(), Error> {
    let start_time = Instant::now();

    loop {
        // Если суммарное время превышено, переходим к следующему шагу
        if start_time.elapsed() >= max_time {
            break;
        }

        let result = timeout(max_time, can_driver.read_alerts()).await;
        match result {
            Ok(Ok(alerts)) => {
                warn!("Alerts: {:?}", alerts);
                if alerts.contains(Alert::ErrorPass) {
                    can_driver_recovery()?;
                }
                if alerts.contains(Alert::RecoveryInProgress) {
                    can_driver_start(can_driver)?;
                    break;
                }
            }
            Ok(Err(e)) => {
                warn!("Error receiving alerts : {:?}", e);
                break;
            }
            Err(_) => break,
        };
    }
    Ok(())
}

/// Запуск драйвера CAN
fn can_driver_start<'a>(can_driver: &mut AsyncCanDriver<'a, CanDriver<'a>>) -> Result<(), Error> {
    info!("CAN driver - start");
    let res = can_driver.start();
    if let Err(err) = res {
        warn!("Error starting CAN driver: {:?}", err);
        return Err(Error::DriverStart(err));
    }
    Ok(())
}

/// Восстановление драйвера CAN
///
/// Используется unsafe, поскольку в API AsyncCanDriver нет методов
fn can_driver_recovery() -> Result<(), Error> {
    info!("CAN driver - recovery");
    let res = esp!(unsafe { twai_initiate_recovery() });
    if let Err(err) = res {
        warn!("Error recovery CAN driver: {:?}", err);
        return Err(Error::DriverRecovery(err));
    }
    Ok(())
}
