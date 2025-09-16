use std::{collections::HashMap, io::Cursor};

use quick_xml::{
    Writer,
    events::{BytesStart, BytesText, Event, attributes::Attributes},
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
        all: &'a HashMap<Vec<u8>, SvgChange>,
    },
    ChangeText {
        all: &'a HashMap<Vec<u8>, SvgChange>,
        id: Vec<u8>,
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
            Event::Start(e_start) => match self {
                States::FindId { all } => {
                    let id = find_id(e_start.attributes());
                    let Some(id) = id else { return (self, event) };

                    let svg_change = all.get(&id);
                    let Some(svg_change) = svg_change else {
                        return (self, event);
                    };

                    let name = String::from_utf8(e_start.name().as_ref().to_vec()).unwrap();
                    let mut e_start = e_start.clone();
                    let mut state = self;

                    for change in &svg_change.change {
                        match change {
                            SvgChangeType::ChangeAttr {
                                attr_name,
                                new_value,
                            } => {
                                e_start = change_attr(
                                    name.clone(),
                                    e_start.attributes(),
                                    attr_name,
                                    new_value,
                                );
                            }
                            SvgChangeType::ChangeAttrStyle {
                                attr_style_name,
                                new_value,
                            } => {
                                e_start = change_attr_style(
                                    name.clone(),
                                    e_start.attributes(),
                                    attr_style_name,
                                    new_value,
                                )
                            }
                            SvgChangeType::ChangeText { text } => {
                                state = Self::ChangeText {
                                    all,
                                    id: id.clone(),
                                    text: text.clone(),
                                }
                            }
                        }
                    }

                    (state, Event::Start(e_start))
                }
                States::ChangeText { all, id, text } => todo!(),
                States::FindChildId { id, change_childs } => todo!(),
                States::ChangeChildText { id, text } => todo!(),
            },

            Event::Text(e_text) => match self {
                States::FindId { all } => (self, event),
                States::ChangeText { all, id, text } => {
                    let e = Event::Text(BytesText::new(&text).into_owned());

                    let state = States::FindId { all };
                    (state, e)
                }
                States::FindChildId { id, change_childs } => todo!(),
                States::ChangeChildText { id, text } => todo!(),
            },

            Event::Empty(e_empty) => match self {
                States::FindId { all } => {
                    let id = find_id(e_empty.attributes());
                    let Some(id) = id else { return (self, event) };

                    let svg_change = all.get(&id);
                    let Some(svg_change) = svg_change else {
                        return (self, event);
                    };

                    let name = String::from_utf8(e_empty.name().as_ref().to_vec()).unwrap();
                    let mut e_empty = e_empty.clone();

                    for change in &svg_change.change {
                        match change {
                            SvgChangeType::ChangeAttr {
                                attr_name,
                                new_value,
                            } => {
                                e_empty = change_attr(
                                    name.clone(),
                                    e_empty.attributes(),
                                    attr_name,
                                    new_value,
                                );
                            }
                            SvgChangeType::ChangeAttrStyle {
                                attr_style_name,
                                new_value,
                            } => {
                                e_empty = change_attr_style(
                                    name.clone(),
                                    e_empty.attributes(),
                                    attr_style_name,
                                    new_value,
                                )
                            }
                            SvgChangeType::ChangeText { text } => todo!(),
                        }
                    }

                    (self, Event::Empty(e_empty))
                }
                States::ChangeText { all, id, text } => todo!(),
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

fn change_attr_style<'a>(
    name: String,
    attributes: Attributes,
    attr_style_name: &str,
    new_value: &str,
) -> BytesStart<'a> {
    let mut elem = BytesStart::new(name);

    for attr in attributes {
        let attr = attr.unwrap();
        let key = attr.key.as_ref();
        if key == b"style" {
            let style = String::from_utf8(attr.value.as_ref().to_vec()).unwrap();

            let mut style: HashMap<&str, &str> = style
                .split(";")
                .map(|e| {
                    let mut split = e.split(":");
                    let key = split.next().unwrap_or_default();
                    let value = split.next().unwrap_or_default();
                    (key.trim(), value.trim())
                })
                .collect();

            style.entry(attr_style_name).and_modify(|e| *e = new_value);

            let style = style
                .into_iter()
                .map(|(k, v)| format!("{}:{}", k, v))
                .collect::<Vec<String>>()
                .join(";");

            elem.push_attribute((key, style.as_bytes()));
        } else {
            elem.push_attribute((key, attr.value.as_ref()));
        }
    }
    elem
}
