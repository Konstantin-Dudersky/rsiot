use std::fmt::Debug;

use esp_idf_svc::nvs::{EspDefaultNvsPartition, EspNvs, EspNvsPartition, NvsDefault};
use postcard::{from_bytes, to_stdvec};
use serde::{de::DeserializeOwned, Serialize};
use tracing::{error, info, warn};

use rsiot_component_core::{CacheType, ComponentInput, ComponentOutput};
use rsiot_messages_core::IMessage;

use super::{config::Config, error::Error};

type Result<T> = std::result::Result<T, Error>;

pub async fn fn_process<TMessage, TStorageData>(
    input: ComponentInput<TMessage>,
    output: ComponentOutput<TMessage>,
    config: Config<TMessage, TStorageData>,
    _cache: CacheType<TMessage>,
) where
    TMessage: IMessage,
    TStorageData: Debug + Default + DeserializeOwned + PartialEq + Serialize,
{
    info!("Starting cmp_storage_esp component");
    let res = task_main(input, output, config).await;
    if let Err(err) = res {
        error!("Error in cmp_storage_esp component: {}", err);
    }
}

async fn task_main<TMessage, TStorageData>(
    input: ComponentInput<TMessage>,
    output: ComponentOutput<TMessage>,
    config: Config<TMessage, TStorageData>,
) -> Result<()>
where
    TMessage: IMessage,
    TStorageData: Debug + Default + DeserializeOwned + PartialEq + Serialize,
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
        output
            .send(msg)
            .await
            .map_err(|e| Error::SendChannel(e.to_string()))?;
    }

    task_input(input, config, nvs, data).await?;
    Ok(())
}

async fn task_input<TMessage, TStorageData>(
    mut input: ComponentInput<TMessage>,
    config: Config<TMessage, TStorageData>,
    mut nvs: EspNvs<NvsDefault>,
    data: TStorageData,
) -> Result<()>
where
    TMessage: IMessage,
    TStorageData: Debug + Default + DeserializeOwned + PartialEq + Serialize,
{
    let mut data = data;
    while let Ok(msg) = input.recv().await {
        let new_data = (config.fn_input)(&data, &msg);
        let new_data = match new_data {
            Some(data) => data,
            None => continue,
        };
        if new_data == data {
            continue;
        }
        data = new_data;
        save_data(&mut nvs, &data)?;
    }
    Ok(())
}

fn load_data<TStorageData>(nvs: &mut EspNvs<NvsDefault>) -> Result<TStorageData>
where
    TStorageData: Debug + Default + DeserializeOwned + Serialize,
{
    let mut data_bytes: &mut [u8] = &mut [0; 1024];
    let data = nvs
        .get_raw("data", &mut data_bytes)
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
    Ok(())
}
