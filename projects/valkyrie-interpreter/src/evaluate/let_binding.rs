use super::*;

impl ValkyrieScope {
    pub(crate) async fn execute_let_bind(&mut self, bind: LetBindNode) -> ValkyrieResult<ValkyrieValue> {
        match bind.pattern {
            LetPattern::Tuple(t) => match t.terms.as_slice() {
                [] => Err(ValkyrieError::custom("Empty tuple patterns are not allowed")),
                [v] => {
                    let rhs = match bind.body {
                        None => ValkyrieValue::Nothing,
                        Some(v) => self.execute_expression_term(v).await?,
                    };
                    todo!()
                    // self.define_variable(&v.key.name, &v.modifiers, rhs)
                }
                _ => {
                    return Err(ValkyrieError::custom("Tuple patterns are not yet supported"));
                }
            },
            LetPattern::Symbol(_) => Err(ValkyrieError::custom("Case symbol are not yet supported")),
            LetPattern::Class(_) => Err(ValkyrieError::custom("Case class are not yet supported")),
            LetPattern::Union(_) => Err(ValkyrieError::custom("Case union are not yet supported")),
            LetPattern::Array(_) => Err(ValkyrieError::custom("Case array are not yet supported")),
            LetPattern::Atom(v) => {
                let rhs = match bind.body {
                    Some(v) => self.execute_expression_term(v).await?,
                    None => ValkyrieValue::Nothing,
                };
                self.define_variable(&v, rhs)
            }
        }
    }
}
