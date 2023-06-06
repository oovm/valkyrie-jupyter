use crate::ValkyrieOutput::Error;
use super::*;

impl ValkyrieVM {
    pub(crate) async fn execute_let_bind(&mut self, bind: LetBindNode) -> ValkyrieOutput {
        match bind.pattern {
            PatternType::Tuple(t) => {
                match t.as_slice() {
                    [] => {
                        Error(ValkyrieError::custom("Empty tuple patterns are not allowed"))
                    }
                    [v] => {
                        let rhs = match bind.body {
                            None => {
                                ValkyrieValue::Nothing
                            }
                            Some(v) => {
                                self.execute_expr_node(v).await?
                            }
                        };
                        self.top_scope.define_variable(&v.key.name, v.get_modifiers(), rhs)
                    }
                    _ => {
                        return Error(ValkyrieError::custom("Tuple patterns are not yet supported"));
                    }
                }
            }
            PatternType::Case => {
                Error(ValkyrieError::custom("Case patterns are not yet supported"))
            }
        }
    }
}