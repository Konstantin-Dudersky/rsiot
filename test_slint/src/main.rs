use std::env;
use std::{sync::Arc, time::Duration};

use rsiot::{
    components::{cmp_inject_periodic, cmp_slint},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    message::{example_message::*, *},
};
use slint::{spawn_local, ComponentHandle, SharedString, Weak};
use slint_interpreter::{ComponentCompiler, ComponentInstance, Value};
use tokio::{sync::Mutex, task::LocalSet};
use tracing::Level;

// #[tokio::main]
// async fn main() {
//     let mut compiler = ComponentCompiler::default();
//     let definition = compiler.build_from_path("test.slint").await;
//     slint_interpreter::print_diagnostics(&compiler.diagnostics());
//     if let Some(definition) = definition {
//         let instance = definition.create().unwrap();
//         instance.run().unwrap();
//     }
// }

// #[tokio::main]
// async fn main() {
//     tracing_subscriber::fmt()
//         .with_max_level(Level::TRACE)
//         .init();

//     let mut compiler = ComponentCompiler::default();
//     let definition = compiler.build_from_path("./test.slint").await;
//     slint_interpreter::print_diagnostics(&compiler.diagnostics());
//     if let Some(definition) = definition {
//         let instance = definition.create().unwrap();
//         let instance_copy = instance.as_weak();

//         std::thread::spawn(move || main_executor(instance_copy));

//         instance.run().unwrap();
//     }
// }

slint::include_modules!();
fn main() {
    let main_window = MainWindow::new().unwrap();

    let main_window_link = main_window.as_weak();

    std::thread::spawn(move || main_executor(main_window_link.into()));
    main_window.run().unwrap();
}

#[tokio::main]
async fn main_executor(slint_inst: Weak<ComponentInstance>) {
    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "test_slint".into(),
        fn_auth: |msg, _| Some(msg),
    };

    let mut counter = 0.0;
    let inject_config = cmp_inject_periodic::Config {
        period: Duration::from_secs(2),
        fn_periodic: move || {
            let msg = Message::new_custom(Custom::ValueInstantF64(counter));
            counter += 1.0;
            vec![msg]
        },
    };

    let slint_config = cmp_slint::Config {
        instance: Arc::new(Mutex::new(slint_inst)),
        fn_input: |msg| {
            vec![(
                "text_content".into(),
                Value::String(SharedString::from("456")),
            )]
        },
    };

    let set = LocalSet::new();
    set.spawn_local(async {
        ComponentExecutor::<Custom>::new(executor_config)
            .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
            .add_cmp(cmp_slint::Cmp::new(slint_config))
            .wait_result()
            .await
            .unwrap();
    });
    set.await;
}
