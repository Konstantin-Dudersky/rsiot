use futures::future::LocalBoxFuture;
use rsiot::{
    components::cmp_external_fn_process,
    executor::{CmpInOut, ComponentExecutor, ComponentExecutorConfig, ComponentResult},
    message::{example_message::*, *},
};
use slint::SharedString;
use slint_interpreter::{ComponentCompiler, ComponentHandle, Value};
use tokio::task::LocalSet;

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

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "test_slint".into(),
        fn_auth: |msg, _| Some(msg),
    };

    fn fn_process_wrapper<TMsg>(in_out: CmpInOut<TMsg>) -> LocalBoxFuture<'static, ComponentResult>
    where
        TMsg: MsgDataBound + 'static,
    {
        Box::pin(async { fn_process(in_out).await })
    }

    async fn fn_process<TMsg>(_in_out: CmpInOut<TMsg>) -> ComponentResult {
        let mut compiler = ComponentCompiler::default();
        let definition = compiler.build_from_path("test.slint").await;
        slint_interpreter::print_diagnostics(&compiler.diagnostics());
        if let Some(definition) = definition {
            let instance = definition.create().unwrap();
            let ins2 = instance.clone_strong();
            ins2.set_property("text_content", Value::String(SharedString::from("123")))
                .unwrap();
            ins2.run().unwrap();
        }
        Ok(())
    }

    let external_process_config = cmp_external_fn_process::Config {
        fn_process: Box::new(fn_process_wrapper::<Custom>),
    };

    let set = LocalSet::new();
    set.spawn_local(async {
        ComponentExecutor::<Custom>::new(executor_config)
            .add_cmp(cmp_external_fn_process::Cmp::new(external_process_config))
            .wait_result()
            .await
            .unwrap();
    });
    set.await;
}
