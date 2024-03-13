use std::time::Duration;

use futures::future::BoxFuture;
use rsiot::{
    components::cmp_external_fn_process,
    executor::{CmpInOut, ComponentExecutor, ComponentExecutorConfig, ComponentResult},
    message::{example_message::*, *},
};
// use slint_interpreter::{ComponentCompiler, ComponentHandle};
use tokio::time::sleep;

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
    let executor_config = ComponentExecutorConfig {
        buffer_size: 100,
        executor_name: "test_slint".into(),
        fn_auth: |msg, _| Some(msg),
    };

    fn fn_process_wrapper<TMsg>(in_out: CmpInOut<TMsg>) -> BoxFuture<'static, ComponentResult>
    where
        TMsg: MsgDataBound + 'static,
    {
        Box::pin(async { fn_process(in_out).await })
    }

    async fn fn_process<TMsg>(_in_out: CmpInOut<TMsg>) -> ComponentResult {
        loop {
            sleep(Duration::from_secs(2)).await;
        }
    }

    let external_process_config = cmp_external_fn_process::Config {
        fn_process: Box::new(fn_process_wrapper::<Custom>),
    };

    ComponentExecutor::<Custom>::new(executor_config)
        .add_cmp(cmp_external_fn_process::Cmp::new(external_process_config))
        .wait_result()
        .await
        .unwrap();
}
