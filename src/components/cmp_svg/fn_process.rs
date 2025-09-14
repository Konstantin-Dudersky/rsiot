use std::{collections::HashMap, io::Cursor};

use quick_xml::{
    Writer,
    events::{BytesText, Event},
    reader::Reader,
};

use crate::{
    executor::CmpInOut,
    message::{Message, MsgDataBound},
};

use super::{Config, SvgChange, SvgChangeType};

pub async fn fn_process<TMsg>(
    config: Config<TMsg>,
    mut msg_bus: CmpInOut<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let mut svg_file = config.file.to_string();

    while let Ok(msg) = msg_bus.recv_input().await {
        let Some(msg) = msg.get_custom_data() else {
            continue;
        };

        let svg_change = (config.fn_input)(&msg);
        if svg_change.is_empty() {
            continue;
        }

        let svg_change: HashMap<&'static [u8], SvgChange> = svg_change
            .iter()
            .map(|c| (c.id.as_bytes(), c.clone()))
            .collect();

        let mut reader = Reader::from_str(&svg_file);
        reader.config_mut().trim_text(true);
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        loop {
            match reader.read_event() {
                Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),
                Ok(Event::Eof) => break,

                Ok(Event::Start(event_start)) => {
                    let name = event_start.name().as_ref().to_owned();
                    let name = String::from_utf8_lossy(&name);
                    println!("{:?}", name);

                    let test = b"sdfsdf";

                    println!(
                        "attributes values: {:?}",
                        event_start
                            .attributes()
                            .map(|a| {
                                let a = a.unwrap();

                                let key = a.key.as_ref().to_vec();
                                let key = String::from_utf8(key).unwrap();

                                let value = a.value.to_owned();
                                let value = String::from_utf8(value.into()).unwrap();
                                (key, value)
                            })
                            .collect::<Vec<_>>()
                    );
                    println!("\n");

                    match event_start.name().as_ref() {
                        b"tag1" => println!(
                            "attributes values: {:?}",
                            event_start
                                .attributes()
                                .map(|a| a.unwrap().value)
                                .collect::<Vec<_>>()
                        ),
                        _ => (),
                    }
                    writer.write_event(Event::Start(event_start)).is_ok();
                }
                Ok(Event::Text(event_text)) => {
                    println!("txt: {:?}", event_text.decode().unwrap().into_owned());

                    let text = BytesText::new("2");
                    writer.write_event(Event::Text(text)).is_ok();
                }

                Ok(Event::End(event_end)) => {
                    println!("end: {:?}", event_end.name());
                    writer.write_event(Event::End(event_end)).is_ok();
                }

                // There are several other `Event`s we do not consider here
                Ok(event) => {
                    writer.write_event(event).is_ok();
                }
            }
        }
        let result = writer.into_inner().into_inner();

        let msg = (config.fn_output)(&result);
        let msg = Message::new_custom(msg);
        msg_bus.send_output(msg).await.unwrap();

        svg_file = String::from_utf8(result).unwrap();
    }

    Ok(())
}

enum States<'a> {
    FindId {
        all: &'a HashMap<&'static [u8], Vec<SvgChangeType>>,
    },
    ChangeText {
        id: String,
        text: String,
    },
    FindChildId {
        id: String,
        change_childs: HashMap<String, Vec<SvgChangeType>>,
    },
    ChangeChildText {
        id: String,
        text: String,
    },
}
