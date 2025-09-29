#[cfg(all(
    feature = "serde_cbor",
    feature = "serde_json",
    feature = "serde_messagepack",
    feature = "serde_postcard",
    feature = "serde_toml"
))]
fn main() {
    use rsiot::serde_utils::{SerdeAlg, SerdeAlgKind};
    use serde::{Deserialize, Serialize};

    #[derive(Deserialize, Serialize)]
    struct TestStruct {
        pub test_enum: TestEnum,
    }

    #[allow(missing_docs)]
    #[derive(Deserialize, Serialize)]
    pub enum TestEnum {
        VariantEmpty,
        VariantFloat(f64),
        VariantBool(bool),
        VariantString(String),
        ValueInt(u64),
    }

    tracing_subscriber::fmt().init();

    let serde = vec![
        ("CBOR", SerdeAlg::new(SerdeAlgKind::Cbor)),
        ("JSON", SerdeAlg::new(SerdeAlgKind::Json)),
        ("MessagePack", SerdeAlg::new(SerdeAlgKind::MessagePack)),
        ("Postcard", SerdeAlg::new(SerdeAlgKind::Postcard)),
        ("Toml", SerdeAlg::new(SerdeAlgKind::Toml)),
    ];

    println!("\n\n VariantEmpty: \n");
    let data = TestStruct {
        test_enum: TestEnum::VariantEmpty,
    };
    for (name, serde) in &serde {
        let ser = serde.serialize(&data).unwrap();
        println!("{name}: {:?}", ser.len());
    }

    println!("\n\n VariantFloat(123.456): \n");
    let data = TestStruct {
        test_enum: TestEnum::VariantFloat(123.456),
    };
    for (name, serde) in &serde {
        let ser = serde.serialize(&data).unwrap();
        println!("{name}: {:?}", ser.len());
    }

    println!("\n\n VariantBool(true): \n");
    let data = TestStruct {
        test_enum: TestEnum::VariantBool(true),
    };
    for (name, serde) in &serde {
        let ser = serde.serialize(&data).unwrap();
        println!("{name}: {:?}", ser.len());
    }

    println!("\n\n VariantString: \n");
    let data = TestStruct {
        test_enum: TestEnum::VariantString("Hello World!".to_string()),
    };
    for (name, serde) in &serde {
        let ser = serde.serialize(&data).unwrap();
        println!("{name}: {:?}", ser.len());
    }

    println!("\n\n ValueInt(123456): \n");
    let data = TestStruct {
        test_enum: TestEnum::ValueInt(123456),
    };
    for (name, serde) in &serde {
        let ser = serde.serialize(&data).unwrap();
        println!("{name}: {:?}", ser.len());
    }
}

#[cfg(not(all(
    feature = "serde_cbor",
    feature = "serde_json",
    feature = "serde_messagepack",
    feature = "serde_postcard",
    feature = "serde_toml"
)))]
fn main() {
    unimplemented!()
}
