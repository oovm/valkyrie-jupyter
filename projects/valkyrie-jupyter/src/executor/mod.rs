use crate::{config::ValkyrieConfig, DisplayKeywords, DisplayNumber};
use jupyter::{value_type::HtmlText, ExecutionRequest, JupyterError, JupyterKernelSockets, JupyterMessage};
use valkyrie_interpreter::{ValkyrieResult, ValkyrieVM, ValkyrieValue};

pub struct ValkyrieExecutor {
    pub(crate) vm: ValkyrieVM,
    pub(crate) sockets: JupyterKernelSockets,
    pub(crate) config: ValkyrieConfig,
}

impl Default for ValkyrieExecutor {
    fn default() -> Self {
        ValkyrieExecutor { vm: ValkyrieVM::default(), sockets: Default::default(), config: ValkyrieConfig::default() }
    }
}

impl ValkyrieExecutor {
    pub(crate) async fn repl_parse_and_run(&mut self, code: &ExecutionRequest) -> ValkyrieResult<()> {
        let tasks = ValkyrieVM::parse_statements(&code.code)?;
        for task in ValkyrieVM::parse_statements(&code.code)? {
            match self.vm.execute_statement(task).await {
                Ok(v) => self.send_value(v, &code.header).await,
                Err(e) => self.sockets.send_executed(JupyterError::custom(format!("Error: {}", e)), &code.header).await,
            }
        }
        Ok(())
    }

    pub(crate) async fn send_value(&self, value: ValkyrieValue, parent: &JupyterMessage) {
        match value {
            // never type never sends
            ValkyrieValue::Nothing => {}
            ValkyrieValue::Null => self.sockets.send_executed(DisplayKeywords::new("null"), parent).await,
            ValkyrieValue::Unit => self.sockets.send_executed(DisplayKeywords::new("( )"), parent).await,
            ValkyrieValue::Boolean(v) => self.sockets.send_executed(DisplayKeywords::new(v), parent).await,
            ValkyrieValue::Number(v) => self.sockets.send_executed(DisplayNumber::new(v), parent).await,
            ValkyrieValue::Unicode(v) => self.sockets.send_executed(v.to_string(), parent).await,
            ValkyrieValue::UTF8String(v) => self.sockets.send_executed(v.get().clone(), parent).await,
            ValkyrieValue::Bytes(_) => {
                todo!()
            }
            ValkyrieValue::Class(_) => {
                todo!()
            }
            ValkyrieValue::Variant(_) => {
                todo!()
            }
            ValkyrieValue::NDArray(_) => {
                todo!()
            }
            ValkyrieValue::Image(_) => {
                todo!()
            }
            ValkyrieValue::Table(_) => {
                todo!()
            }
            ValkyrieValue::Html(v) => {
                self.sockets.send_executed(HtmlText::new(v), parent).await;
            }
            ValkyrieValue::Uninitialized => {
                todo!()
            }
        }
    }
}
