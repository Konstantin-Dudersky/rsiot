use std::{collections::HashMap, io::Cursor};

use quick_xml::{
    Writer,
    events::{BytesStart, BytesText, Event, attributes::Attributes},
    reader::Reader,
};

use crate::{
    executor::{MsgBusInput, MsgBusOutput},
    message::{Message, MsgDataBound},
};

use super::{Config, Error, SvgChange, SvgChangeType};

pub async fn fn_process<TMsg>(
    config: Config<TMsg>,
    mut msgbus_input: MsgBusInput<TMsg>,
    msgbus_output: MsgBusOutput<TMsg>,
) -> super::Result<()>
where
    TMsg: MsgDataBound,
{
    let mut svg_file = config.file.to_string();
    let msg = (config.fn_output)(svg_file.as_bytes());
    let msg = Message::new_custom(msg);
    msgbus_output
        .send(msg)
        .await
        .map_err(|_| Error::TokioSyncMpscSend)?;

    while let Ok(msg) = msgbus_input.recv().await {
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

                    let res = state.process_event(e)?;

                    state = res.0;
                    writer.write_event(res.1).map_err(Error::WriteEvent)?;
                }
            }
        }
        let result = writer.into_inner().into_inner();

        let msg = (config.fn_output)(&result);
        let msg = Message::new_custom(msg);
        msgbus_output
            .send(msg)
            .await
            .map_err(|_| Error::TokioSyncMpscSend)?;

        svg_file = String::from_utf8(result)?;
    }

    Ok(())
}

#[allow(dead_code)]
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
    pub fn process_event(self, event: Event) -> Result<(States<'a>, Event), Error> {
        match &event {
            Event::Start(e_start) => match self {
                States::FindId { all } => {
                    let id = find_id(e_start.attributes())?;
                    let Some(id) = id else {
                        return Ok((self, event));
                    };

                    let svg_change = all.get(&id);
                    let Some(svg_change) = svg_change else {
                        return Ok((self, event));
                    };

                    let name = String::from_utf8(e_start.name().as_ref().to_vec())?;
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
                                )?;
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
                                )?
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

                    Ok((state, Event::Start(e_start)))
                }
                States::ChangeText {
                    all: _,
                    id: _,
                    text: _,
                } => todo!(),
                States::FindChildId {
                    id: _,
                    change_childs: _,
                } => todo!(),
                States::ChangeChildText { id: _, text: _ } => todo!(),
            },

            Event::Text(_e_text) => match self {
                States::FindId { all: _ } => Ok((self, event)),
                States::ChangeText { all, id: _, text } => {
                    let e = Event::Text(BytesText::new(&text).into_owned());

                    let state = States::FindId { all };
                    Ok((state, e))
                }
                States::FindChildId {
                    id: _,
                    change_childs: _,
                } => todo!(),
                States::ChangeChildText { id: _, text: _ } => todo!(),
            },

            Event::Empty(e_empty) => match self {
                States::FindId { all } => {
                    let id = find_id(e_empty.attributes())?;
                    let Some(id) = id else {
                        return Ok((self, event));
                    };

                    let svg_change = all.get(&id);
                    let Some(svg_change) = svg_change else {
                        return Ok((self, event));
                    };

                    let name = String::from_utf8(e_empty.name().as_ref().to_vec())?;
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
                                )?;
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
                                )?
                            }
                            SvgChangeType::ChangeText { text: _ } => todo!(),
                        }
                    }

                    Ok((self, Event::Empty(e_empty)))
                }
                States::ChangeText {
                    all: _,
                    id: _,
                    text: _,
                } => todo!(),
                States::FindChildId {
                    id: _,
                    change_childs: _,
                } => todo!(),
                States::ChangeChildText { id: _, text: _ } => todo!(),
            },

            _ => Ok((self, event)),
        }
    }
}

fn find_id(attributes: Attributes) -> Result<Option<Vec<u8>>, Error> {
    for attr in attributes {
        let attr = attr?;
        let key = attr.key.as_ref();

        if key == b"id" {
            let value: Vec<u8> = attr.value.as_ref().into();
            return Ok(Some(value));
        }
    }

    Ok(None)
}

fn change_attr<'a>(
    name: String,
    attributes: Attributes,
    attr_name: &str,
    new_value: &str,
) -> Result<BytesStart<'a>, Error> {
    let mut elem = BytesStart::new(name);

    for attr in attributes {
        let attr = attr?;
        let key = attr.key.as_ref();
        if key == attr_name.as_bytes() {
            elem.push_attribute((key, new_value.as_bytes()));
        } else {
            elem.push_attribute((key, attr.value.as_ref()));
        }
    }
    Ok(elem)
}

fn change_attr_style<'a>(
    name: String,
    attributes: Attributes,
    attr_style_name: &str,
    new_value: &str,
) -> Result<BytesStart<'a>, Error> {
    let mut elem = BytesStart::new(name);

    for attr in attributes {
        let attr = attr?;
        let key = attr.key.as_ref();
        if key == b"style" {
            let style = String::from_utf8(attr.value.as_ref().to_vec())?;

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
    Ok(elem)
}
