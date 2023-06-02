use jupyter::{async_trait, Executed, ExecutionReply, ExecutionRequest, ExecutionResult, JupyterKernelProtocol, JupyterTheme, LanguageInfo, UnboundedSender, Value};
use jupyter_derive::{include_png32, include_png64};
use crate::executor::ValkyrieExecutor;
use crate::protocol::display::DisplayError;

pub mod display;

#[async_trait]
impl JupyterKernelProtocol for ValkyrieExecutor {
    fn language_info(&self) -> LanguageInfo {
        LanguageInfo::new("valkyrie", "Valkyrie")
            .with_syntax("scala", "scala")
            .with_version(env!("CARGO_PKG_VERSION"))
    }

    async fn running(&mut self, code: ExecutionRequest) -> ExecutionReply {
        match self.repl_parse_and_run(&code.code).await {
            Ok(_) => code.as_reply(true, code.execution_count),
            Err(e) => {
                self.sockets.send_executed(DisplayError::new(e.to_string())).await;
                code.as_reply(false, code.execution_count)
            }
        }
    }


    fn running_time(&self, time: f64) -> String {
        if self.config.running_time { format!("<sub>Elapsed time: {:.2} seconds.</sub>", time) } else { String::new() }
    }

    async fn bind_execution_socket(&self, sender: UnboundedSender<ExecutionResult>) {
        self.sockets.bind_execution_socket(sender).await
    }
}
