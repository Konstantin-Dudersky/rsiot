use std::fmt::Debug;

use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs, EspNvsPartition, NvsDefault};
use postcard::{from_bytes, to_stdvec};
use serde::{de::DeserializeOwned, Serialize};
use tracing::{debug, info, warn};

use crate::{
    executor::{CmpInOut, ComponentError},
    message::{MsgDataBound, ServiceBound},
};

use super::{config::Config, error::Error};

type Result<T> = std::result::Result<T, Error>;

pub async fn fn_process<TMessage, TService, TStorageData>(
    input: CmpInOut<TMessage, TService>,
    config: Config<TMessage, TStorageData>,
) -> std::result::Result<(), ComponentError>
where
    TMessage: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
    TStorageData: Debug + Default + DeserializeOwned + PartialEq + Serialize + 'static,
{
    info!("Starting cmp_storage_esp component");
    task_main(input, config).await.map_err(|err| {
        let err = err.to_string();
        ComponentError::Execution(err)
    })
}

async fn task_main<TMessage, TService, TStorageData>(
    msg_bus: CmpInOut<TMessage, TService>,
    config: Config<TMessage, TStorageData>,
) -> Result<()>
where
    TMessage: MsgDataBound + 'static,
    TService: ServiceBound + 'static,
    TStorageData: Debug + Default + DeserializeOwned + PartialEq + Serialize + 'static,
{
    let nvs_default_partition: EspNvsPartition<NvsDefault> =
        EspDefaultNvsPartition::take().map_err(Error::TakePartition)?;

    let test_namespace = "test_ns";
    let mut nvs = match EspNvs::new(nvs_default_partition, test_namespace, true) {
        Ok(nvs) => {
            info!("Got namespace {:?} from default partition", test_namespace);
            nvs
        }
        Err(e) => panic!("Could't get namespace {:?}", e),
    };

    let data = load_data(&mut nvs)?;
    let msgs = (config.fn_output)(&data);
    for msg in msgs {
        msg_bus
            .send_output(msg)
            .await
            .map_err(|e| Error::SendChannel(e.to_string()))?;
    }

    task_input(msg_bus, config, nvs, data).await?;
    Ok(())
}

async fn task_input<TMessage, TService, TStorageData>(
    mut msg_bus: CmpInOut<TMessage, TService>,
    config: Config<TMessage, TStorageData>,
    mut nvs: EspNvs<NvsDefault>,
    data: TStorageData,
) -> Result<()>
where
    TMessage: MsgDataBound,
    TService: ServiceBound,
    TStorageData: Debug + Default + DeserializeOwned + PartialEq + Serialize,
{
    let mut data = data;
    while let Ok(msg) = msg_bus.recv_input().await {
        let new_data = (config.fn_input)(&data, &msg);
        let Some(new_data) = new_data else { continue };
        if new_data == data {
            continue;
        }
        data = new_data;
        save_data(&mut nvs, &data)?;

        let data = load_data(&mut nvs)?;
        let msgs = (config.fn_output)(&data);
        for msg in msgs {
            msg_bus
                .send_output(msg)
                .await
                .map_err(|e| Error::SendChannel(e.to_string()))?;
        }
    }
    Ok(())
}

fn load_data<TStorageData>(nvs: &mut EspNvs<NvsDefault>) -> Result<TStorageData>
where
    TStorageData: Debug + Default + DeserializeOwned + Serialize,
{
    let data_bytes: &mut [u8] = &mut [0; 1024];
    let data = nvs
        .get_raw("data", data_bytes)
        .map_err(Error::ReadFromEsp)?;

    match data {
        Some(data) => {
            let data = from_bytes(data);
            match data {
                Ok(data) => {
                    info!("Data from storage loaded: {:?}", data);
                    Ok(data)
                }
                Err(err) => {
                    let data = TStorageData::default();
                    warn!(
                        "Error deserialization data from storage: {:?}; load default: {:?}",
                        err, data
                    );
                    save_data(nvs, &data)?;
                    Ok(data)
                }
            }
        }
        None => {
            let data = TStorageData::default();
            warn!("Storage empty, generate default: {:?}", data);
            save_data(nvs, &data)?;
            Ok(data)
        }
    }
}

fn save_data<TStorageData>(nvs: &mut EspNvs<NvsDefault>, data: &TStorageData) -> Result<()>
where
    TStorageData: Debug + Default + DeserializeOwned + Serialize,
{
    let data = to_stdvec::<TStorageData>(data)?;
    nvs.set_raw("data", &data).map_err(Error::SaveToEsp)?;
    debug!("Data saved to storage");
    Ok(())
}
