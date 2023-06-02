use jupyter::JupyterKernelSockets;
use valkyrie_interpreter::{parse_repl, ValkyrieResult, ValkyrieValue, ValkyrieVM};
use crate::config::ValkyrieConfig;
use crate::{DisplayError, DisplayKeywords, DisplayNumber};

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
    pub(crate) async fn repl_parse_and_run(&mut self, code: &str) -> ValkyrieResult<()> {
        let terms = parse_repl(code)?;
        for i in terms {
            match self.vm.execute_statement(i).await {
                Ok(v) => self.send_value(v).await,
                Err(e) => {
                    // if e.is_fatal() {
                    //     return Err(e);
                    // } else {
                    //
                    // }
                    self.sockets.send_executed(DisplayError::new(format!("Error: {}", e))).await;
                }
            }
        }
        Ok(())
    }

    pub(crate) async fn send_value(&self, value: ValkyrieValue) {
        match value {
            // never type never sends
            ValkyrieValue::Nothing => {}
            ValkyrieValue::Null => self.sockets.send_executed(DisplayKeywords::new("null")).await,
            ValkyrieValue::Unit => self.sockets.send_executed(DisplayKeywords::new("( )")).await,
            ValkyrieValue::Boolean(v) => self.sockets.send_executed(DisplayKeywords::new(v)).await,
            ValkyrieValue::Integer(v) => self.sockets.send_executed(DisplayNumber::new(v)).await,
            ValkyrieValue::Float32(v) => self.sockets.send_executed(DisplayNumber::new(v)).await,
            ValkyrieValue::Float64(v) => self.sockets.send_executed(DisplayNumber::new(v)).await,
            ValkyrieValue::UTF8Character(v) => self.sockets.send_executed(v.to_string()).await,
            ValkyrieValue::UTF8String(v) => self.sockets.send_executed(v.as_str().to_string()).await,
            ValkyrieValue::Bytes(_) => {
                todo!()
            }
            ValkyrieValue::Class(_) => {
                todo!()
            }
            ValkyrieValue::Variant(_) => {
                todo!()
            }
            ValkyrieValue::Json(v) => self.sockets.send_executed((*v).clone()).await,
            ValkyrieValue::NDArray(_) => {
                todo!()
            }
            ValkyrieValue::Image(_) => {
                todo!()
            }
            ValkyrieValue::DataFrame(_) => {
                todo!()
            }
            ValkyrieValue::DataTable(_) => {
                todo!()
            }
        }
    }
}
