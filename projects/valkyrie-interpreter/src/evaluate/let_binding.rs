use super::*;
use valkyrie_ast::PatternExpressionType;

impl ValkyrieScope {
    pub(crate) async fn execute_let_bind(&mut self, bind: LetBindNode) -> ValkyrieResult<ValkyrieValue> {
        match bind.pattern {
            PatternExpressionType::Tuple(t) => match t.terms.as_slice() {
                [] => Err(ValkyrieError::custom("Empty tuple patterns are not allowed")),
                [v] => {
                    let rhs = match bind.body {
                        None => ValkyrieValue::Nothing,
                        Some(v) => self.execute_term_expression(v).await?,
                    };
                    todo!()
                    // self.define_variable(&v.key.name, &v.modifiers, rhs)
                }
                _ => {
                    return Err(ValkyrieError::custom("Tuple patterns are not yet supported"));
                }
            },
            PatternExpressionType::Symbol(_) => Err(ValkyrieError::custom("Case symbol are not yet supported")),
            PatternExpressionType::Class(_) => Err(ValkyrieError::custom("Case class are not yet supported")),
            PatternExpressionType::Union(_) => Err(ValkyrieError::custom("Case union are not yet supported")),
            PatternExpressionType::Array(_) => Err(ValkyrieError::custom("Case array are not yet supported")),
        }
    }
}
