use super::*;
use valkyrie_ast::{ExpressionKind, PatternCondition, PatternWhenNode};

impl Evaluate for SwitchStatement {
    async fn execute(self, vm: &mut ValkyrieVM, scope: &mut ValkyrieScope) -> Self::Result {
        let mut last = ValkyrieValue::Unit;
        // for branch in node.branches {
        //     let is_true = match branch.condition {
        //         PatternCondition::Case(_) => Err(SyntaxError::new("case is not supported in switch statements"))?,
        //         PatternCondition::When(v) => self.evaluate_pattern_when(v).await?,
        //         PatternCondition::Type(_) => Err(SyntaxError::new("case is not supported in switch statements"))?,
        //         PatternCondition::Else(_) => true,
        //     };
        //     if !is_true {
        //         continue;
        //     }
        //     let mut child = self.fork();
        //     for stmt in branch.statements.terms {
        //         last = child.execute_statement(stmt).await?;
        //     }
        //     break;
        // }
        // no condition, no statements, return unit
        Ok(last)
    }
}

impl Evaluate for PatternWhenNode {
    async fn execute(self, vm: &mut ValkyrieVM, scope: &mut ValkyrieScope) -> Self::Result {
        match self.execute_expression_term(node.guard).await? {
            ValkyrieValue::Boolean(v) => Ok(v),
            _ => Err(SyntaxError::new("condition guard must be a boolean expression"))?,
        }
    }
}
