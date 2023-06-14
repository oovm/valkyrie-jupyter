use super::*;
use valkyrie_ast::PatternExpressionNode;

impl ValkyrieVM {
    pub(crate) async fn execute_let_bind(&mut self, bind: LetBindNode) -> ValkyrieResult<ValkyrieValue> {
        match bind.pattern {
            PatternExpressionNode::Tuple(t) => match t.as_slice() {
                [] => Err(ValkyrieError::custom("Empty tuple patterns are not allowed")),
                [v] => {
                    let rhs = match bind.body {
                        None => ValkyrieValue::Nothing,
                        Some(v) => self.execute_expr_node(v).await?,
                    };
                    self.top_scope.define_variable(&v.key.name, &v.modifiers, rhs)
                }
                _ => {
                    return Err(ValkyrieError::custom("Tuple patterns are not yet supported"));
                }
            },
            PatternExpressionNode::Case => Err(ValkyrieError::custom("Case patterns are not yet supported")),
        }
    }
}
