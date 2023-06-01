
pub struct ValkyrieExecutor {
    vm: ValkyrieVM,
    sockets: JupyterServerSockets,
    config: ValkyrieConfig,
}

impl Default for ValkyrieExecutor {
    fn default() -> Self {
        ValkyrieExecutor { sockets: Default::default(), config: ValkyrieConfig::default() }
    }
}

impl ValkyrieExecutor {
    pub(crate) async fn repl_parse_and_run(&mut self, code: &str) -> ValkyrieResult<()> {
        let terms = StatementNode::parse_many(code)?;
        for i in terms {
            match self.execute_repl(i).await {
                Ok(v) => self.send_value(v).await,
                Err(e) => {
                    if e.is_fatal() {
                        return Err(e);
                    }
                    else {
                        self.sockets.send_executed(DisplayError::new(format!("Error: {}", e))).await;
                    }
                }
            }
        }
        Ok(())
    }

    pub async fn execute_repl(&mut self, tree: StatementNode) -> ValkyrieResult<ValkyrieValue> {
        match tree.r#type {
            StatementType::Nothing => {
                todo!()
            }
            StatementType::Namespace(_) => {
                todo!()
            }
            StatementType::Import(_) => {
                todo!()
            }
            StatementType::Class(_) => {
                todo!()
            }
            StatementType::Function(_) => {
                todo!()
            }
            StatementType::While(_) => {
                todo!()
            }
            StatementType::For(_) => {
                todo!()
            }
            StatementType::Expression(_) => {
                todo!()
            }
        }
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
            ValkyrieValue::UTF8Character(v) => self.sockets.send_executed(Value::String(v.to_string())).await,
            ValkyrieValue::UTF8String(v) => self.sockets.send_executed(Value::String(v.as_str().to_string())).await,
            ValkyrieValue::Bytes(_) => {
                todo!()
            }
            ValkyrieValue::Class(_) => {
                todo!()
            }
            ValkyrieValue::Variant(_) => {
                todo!()
            }
            ValkyrieValue::Json(v) => self.sockets.send_executed(DisplayNumber::new(v)).await,
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
