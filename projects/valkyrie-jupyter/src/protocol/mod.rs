
#[async_trait]
impl JupyterServerProtocol for ValkyrieExecutor {
    fn language_info(&self) -> LanguageInfo {
        LanguageInfo {
            language: "Valkyrie".to_string(),
            png_64: include_png64!(),
            png_32: include_png32!(),
            language_key: "valkyrie".to_string(),
            file_extensions: ".vk".to_string(),
        }
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
