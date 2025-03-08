//! Предыдущие способы определения ключа сообщения. После проверки способа на основе макроса
//! MsgKey можно будет удалить

use super::{MsgData, MsgDataBound};

/// Определить ключ сообщения по выводу Debug
pub fn define_key<TCustom>(data: &MsgData<TCustom>) -> String
where
    TCustom: MsgDataBound,
{
    let full_str = format!("{:?}", data);
    let parts = full_str.split(['(', '{']).collect::<Vec<&str>>();

    let mut final_parts = vec![];
    for part in parts {
        // Unit
        if part.is_empty() {
            break;
        }

        // Содержит пробелы
        // if part.contains(" ") {
        //     break;
        // }

        if part.chars().next().unwrap().is_alphabetic()
            && !part.starts_with("true")
            && !part.starts_with("false")
        {
            final_parts.push(part);
        } else {
            break;
        }
    }
    final_parts.join("-").trim().to_string()
}

/// Определить ключ сообщения по выводу Debug
#[deprecated]
fn define_key_old<TCustom>(data: &MsgData<TCustom>) -> String
where
    TCustom: MsgDataBound,
{
    let full_str = format!("{:?}", data);
    let parts = full_str.split('(').collect::<Vec<&str>>();

    // let mut final_parts = vec![];
    // for part in parts {
    //     if part.chars().next().unwrap().is_alphabetic()
    //         && !part.starts_with("true")
    //         && !part.starts_with("false")
    //     {
    //         final_parts.push(part);
    //     } else {
    //         break;
    //     }
    // }
    // final_parts.join("-")
    // Убираем последний элемент. Если тип unit (), нужно убрать два последних элемента
    let skip = if parts[parts.len() - 2].is_empty() {
        2
    } else {
        1
    };
    parts[0..parts.len() - skip].join("-")
}

#[cfg(test)]
mod tests {
    use super::super::example_message::*;
    use super::*;

    /// cargo test --target x86_64-unknown-linux-gnu -- message::define_key::tests::test1 --exact --show-output
    #[test]
    fn test1() {
        let data = MsgData::Custom(Custom::ValueInstantF64(12.34));
        let key = define_key(&data);
        assert_eq!(key, "Custom-ValueInstantF64");

        let data = MsgData::Custom(Custom::ValueInstantBool(false));
        let key = define_key(&data);
        assert_eq!(key, "Custom-ValueInstantBool");

        let data = MsgData::Custom(Custom::ValueInstantString("qwe".to_string()));
        let key = define_key(&data);
        assert_eq!(key, "Custom-ValueInstantString");

        let data = MsgData::Custom(Custom::DataUnit(()));
        let key = define_key(&data);
        assert_eq!(key, "Custom-DataUnit");

        let data = MsgData::Custom(Custom::DataGroup(DataGroup::DataGroupF64(12.45)));
        let key = define_key(&data);
        assert_eq!(key, "Custom-DataGroup-DataGroupF64");

        let data = MsgData::Custom(Custom::DataGroup(DataGroup::DataGroupStruct(
            StructInDataGroup {
                struct_field1: true,
                struct_field2: 3.56,
            },
        )));
        let key = define_key(&data);
        assert_eq!(key, "Custom-DataGroup-DataGroupStruct-StructInDataGroup");

        let data = MsgData::Custom(Custom::DataGroup(DataGroup::DataGroupVectorBool(vec![
            true, false,
        ])));
        let key = define_key(&data);
        assert_eq!(key, "Custom-DataGroup-DataGroupVectorBool");

        let data = MsgData::Custom(Custom::DataGroup(DataGroup::DataGroupVectorTuple(vec![
            (true, "qwe".to_string()),
            (false, "rty".to_string()),
        ])));
        let key = define_key(&data);
        assert_eq!(key, "Custom-DataGroup-DataGroupVectorTuple");

        let data = MsgData::Custom(Custom::Tuple(("qwe".to_string(), (true, false))));
        let key = define_key(&data);
        assert_eq!(key, "Custom-Tuple");

        let data = MsgData::Custom(Custom::ValueStruct { a: 7.89 });
        println!("Data: {:?}", data);
        let key = define_key(&data);
        assert_eq!(key, "Custom-ValueStruct");
    }
}
