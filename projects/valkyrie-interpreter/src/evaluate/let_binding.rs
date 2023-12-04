use super::*;
use valkyrie_ast::{PatternNode, VariableDeclaration, VariantDeclaration};

impl Evaluate for VariableDeclaration {
    #[async_recursion]
    async fn execute(&self, vm: &ValkyrieVM, scope: &ValkyrieScope) -> Self::Result {
        todo!()
        // match bind.pattern {
        //     PatternNode::Tuple(t) => match t.terms.as_slice() {
        //         [] => Err(ValkyrieError::custom("Empty tuple patterns are not allowed")),
        //         [v] => {
        //             let rhs = match bind.body {
        //                 None => ValkyrieValue::Nothing,
        //                 Some(v) => self.execute_expression_term(v).await?,
        //             };
        //             todo!()
        //             // self.define_variable(&v.key.name, &v.modifiers, rhs)
        //         }
        //         _ => {
        //             return Err(ValkyrieError::custom("Tuple patterns are not yet supported"));
        //         }
        //     },
        //     PatternNode::Symbol(_) => Err(ValkyrieError::custom("Case symbol are not yet supported")),
        //     PatternNode::Class(_) => Err(ValkyrieError::custom("Case class are not yet supported")),
        //     PatternNode::Union(_) => Err(ValkyrieError::custom("Case union are not yet supported")),
        //     PatternNode::Array(_) => Err(ValkyrieError::custom("Case array are not yet supported")),
        //     PatternNode::Atom(v) => {
        //         let rhs = match bind.body {
        //             Some(v) => self.execute_expression_term(v).await?,
        //             None => ValkyrieValue::Nothing,
        //         };
        //         self.define_variable(&v, rhs)
        //     }
        // }
    }
}
