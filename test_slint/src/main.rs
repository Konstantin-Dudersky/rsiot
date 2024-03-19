use std::env;
use std::{sync::Arc, time::Duration};

use rsiot::{
    components::{cmp_inject_periodic, cmp_logger, cmp_slint},
    executor::{ComponentExecutor, ComponentExecutorConfig},
    message::{example_message::*, *},
};
use slint::{ComponentHandle, SharedString, Weak};
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
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let main_window = MainWindow::new().unwrap();

    let main_window_link = main_window.as_weak();

    std::thread::spawn(move || main_executor(main_window_link));
    main_window.run().unwrap();
}

#[tokio::main]
async fn main_executor(slint_inst: Weak<MainWindow>) {
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
        fn_input: |msg, window| match msg.data {
            MsgData::Custom(Custom::ValueInstantF64(value)) => window
                .upgrade_in_event_loop(move |h| {
                    h.global::<GlobalData>()
                        .set_primary(SharedString::from(value.to_string()));
                })
                .unwrap(),
            _ => (),
        },
        fn_output: |window, tx| {
            window
                .upgrade_in_event_loop(move |handle| {
                    let global = handle.global::<GlobalData>();
                    global.on_button(move |value| {
                        let msg =
                            Message::new_custom(Custom::ValueInstantString(value.to_string()));
                        tx.blocking_send(msg).unwrap();
                    });
                })
                .unwrap();
        },
    };

    let logger_config = cmp_logger::Config {
        level: Level::INFO,
        header: "".into(),
    };

    let set = LocalSet::new();
    set.spawn_local(async {
        ComponentExecutor::<Custom>::new(executor_config)
            .add_cmp(cmp_inject_periodic::Cmp::new(inject_config))
            .add_cmp(cmp_slint::Cmp::new(slint_config))
            .add_cmp(cmp_logger::Cmp::new(logger_config))
            .wait_result()
            .await
            .unwrap();
    });
    set.await;
}
