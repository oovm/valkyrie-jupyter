use crate::executor::ValkyrieExecutor;
use jupyter::{
    async_trait, Executed, ExecutionReply, ExecutionRequest, ExecutionResult, JupyterConnection, JupyterError,
    JupyterKernelProtocol, LanguageInfo, UnboundedSender, Value,
};
use jupyter_derive::{include_png32, include_png64};

pub mod display;

#[async_trait]
impl JupyterKernelProtocol for ValkyrieExecutor {
    fn language_info(&self) -> LanguageInfo {
        let mut info =
            LanguageInfo::new("valkyrie", "Valkyrie").with_syntax("scala", "scala").with_version(env!("CARGO_PKG_VERSION"));
        info.png_32 = include_png32!();
        info.png_64 = include_png32!();
        return info;
    }

    fn connected(&mut self, context: JupyterConnection) {
        self.sockets = context.sockets;
    }

    async fn running(&mut self, code: ExecutionRequest) -> ExecutionReply {
        match self.repl_parse_and_run(&code).await {
            Ok(_) => ExecutionReply::new(true),
            Err(e) => {
                self.sockets.send_executed(JupyterError::custom(e.to_string()), &code.header).await;
                ExecutionReply::new(false)
            }
        }
    }

    fn running_time(&self, time: f64) -> String {
        if self.config.running_time { format!("<sub>Elapsed time: {:.2} seconds.</sub>", time) } else { String::new() }
    }
}
