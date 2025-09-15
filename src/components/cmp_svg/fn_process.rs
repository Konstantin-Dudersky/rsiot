use std::{collections::HashMap, io::Cursor};

use quick_xml::{
    Writer,
    events::{BytesStart, BytesText, Event, attributes::Attributes},
    reader::Reader,
};
use tracing::info;

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

        let svg_change: HashMap<Vec<u8>, SvgChange> = svg_change
            .iter()
            .map(|c| (c.id.as_bytes().to_vec(), c.clone()))
            .collect();

        let mut state = States::FindId { all: &svg_change };

        let mut reader = Reader::from_str(&svg_file);
        reader.config_mut().trim_text(true);
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        loop {
            match reader.read_event() {
                Err(e) => panic!("Error at position {}: {:?}", reader.error_position(), e),

                Ok(e) => {
                    if matches!(e, Event::Eof) {
                        break;
                    }

                    let res = state.process_event(e);

                    state = res.0;
                    writer.write_event(res.1).is_ok();
                } // Ok(Event::Eof) => break,

                  // Ok(Event::Start(event_start)) => {
                  //     let name = event_start.name().as_ref().to_owned();
                  //     let name = String::from_utf8_lossy(&name);
                  //     println!("{:?}", name);

                  //     let test = b"sdfsdf";

                  //     println!(
                  //         "attributes values: {:?}",
                  //         event_start
                  //             .attributes()
                  //             .map(|a| {
                  //                 let a = a.unwrap();

                  //                 let key = a.key.as_ref().to_vec();
                  //                 let key = String::from_utf8(key).unwrap();

                  //                 let value = a.value.to_owned();
                  //                 let value = String::from_utf8(value.into()).unwrap();
                  //                 (key, value)
                  //             })
                  //             .collect::<Vec<_>>()
                  //     );
                  //     println!("\n");

                  //     match event_start.name().as_ref() {
                  //         b"tag1" => println!(
                  //             "attributes values: {:?}",
                  //             event_start
                  //                 .attributes()
                  //                 .map(|a| a.unwrap().value)
                  //                 .collect::<Vec<_>>()
                  //         ),
                  //         _ => (),
                  //     }
                  //     writer.write_event(Event::Start(event_start)).is_ok();
                  // }
                  // Ok(Event::Text(event_text)) => {
                  //     println!("txt: {:?}", event_text.decode().unwrap().into_owned());

                  //     let text = BytesText::new("2");
                  //     writer.write_event(Event::Text(text)).is_ok();
                  // }

                  // Ok(Event::End(event_end)) => {
                  //     println!("end: {:?}", event_end.name());
                  //     writer.write_event(Event::End(event_end)).is_ok();
                  // }

                  // // There are several other `Event`s we do not consider here
                  // Ok(event) => {
                  //     writer.write_event(event).is_ok();
                  // }
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
        all: &'a HashMap<Vec<u8>, SvgChange>,
    },
    ChangeText {
        id: String,
        text: String,
    },
    FindChildId {
        id: String,
        change_childs: HashMap<String, Vec<SvgChange>>,
    },
    ChangeChildText {
        id: String,
        text: String,
    },
}

impl<'a> States<'a> {
    pub fn process_event(self, event: Event) -> (States<'a>, Event) {
        match &event {
            Event::Start(e) => match self {
                States::FindId { all } => {
                    let id = find_id(e.attributes());
                    let Some(id) = id else { return (self, event) };

                    let svg_change = all.get(&id);
                    let Some(svg_change) = svg_change else {
                        return (self, event);
                    };

                    for change in &svg_change.change {
                        match change {
                            SvgChangeType::ChangeAttr {
                                attr_name,
                                new_value,
                            } => {
                                let name = String::from_utf8(e.name().as_ref().to_vec()).unwrap();
                                let event = change_attr(name, e.attributes(), attr_name, new_value);
                                return (self, Event::Start(event));
                            }
                            SvgChangeType::ChangeAttrStyle => todo!(),
                            SvgChangeType::ChangeText { text } => todo!(),
                        }
                    }

                    (self, event)
                }
                States::ChangeText { id, text } => todo!(),
                States::FindChildId { id, change_childs } => todo!(),
                States::ChangeChildText { id, text } => todo!(),
            },

            Event::Text(bytes_text) => {
                let text = BytesText::new("3");
                let event1 = Event::Text(text);
                (self, event1)
            }

            Event::Empty(e) => match self {
                States::FindId { all } => {
                    let id = find_id(e.attributes());
                    let Some(id) = id else { return (self, event) };

                    let svg_change = all.get(&id);
                    let Some(svg_change) = svg_change else {
                        return (self, event);
                    };

                    for change in &svg_change.change {
                        match change {
                            SvgChangeType::ChangeAttr {
                                attr_name,
                                new_value,
                            } => {
                                let name = String::from_utf8(e.name().as_ref().to_vec()).unwrap();
                                let event = change_attr(name, e.attributes(), attr_name, new_value);
                                return (self, Event::Empty(event));
                            }
                            SvgChangeType::ChangeAttrStyle => todo!(),
                            SvgChangeType::ChangeText { text } => todo!(),
                        }
                    }

                    (self, event)
                }
                States::ChangeText { id, text } => todo!(),
                States::FindChildId { id, change_childs } => todo!(),
                States::ChangeChildText { id, text } => todo!(),
            },

            _ => (self, event),
        }
    }
}

fn find_id(attributes: Attributes) -> Option<Vec<u8>> {
    for attr in attributes {
        let attr = attr.unwrap();
        let key = attr.key.as_ref();

        if key == b"id" {
            let value: Vec<u8> = attr.value.as_ref().into();
            return Some(value);
        }
    }

    None
}

fn change_attr<'a>(
    name: String,
    attributes: Attributes,
    attr_name: &str,
    new_value: &str,
) -> BytesStart<'a> {
    let mut elem = BytesStart::new(name);

    for attr in attributes {
        let attr = attr.unwrap();
        let key = attr.key.as_ref();
        if key == attr_name.as_bytes() {
            elem.push_attribute((key, new_value.as_bytes()));
        } else {
            elem.push_attribute((key, attr.value.as_ref()));
        }
    }
    elem
}
