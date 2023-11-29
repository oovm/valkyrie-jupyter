use super::*;
use valkyrie_ast::ExpressionKind;

impl ValkyrieScope {
    pub async fn execute_statement(&mut self, stmt: StatementNode) -> ValkyrieResult<EvaluatedState> {
        match stmt {
            StatementNode::Nothing => Ok(ValkyrieValue::Nothing),
            StatementNode::Document(_) => {
                todo!()
            }
            StatementNode::Namespace(_) => Ok(ValkyrieValue::Null),
            StatementNode::Import(_) => {
                todo!()
            }
            StatementNode::Class(_) => {
                todo!()
            }
            StatementNode::While(_) => {
                todo!()
            }
            StatementNode::For(_) => {
                todo!()
            }
            StatementNode::Function(_) => {
                todo!()
            }
            StatementNode::Control(_) => {
                todo!()
            }
            StatementNode::Expression(v) => self.execute_expression_term(*v).await,
            StatementNode::Annotation(_) => {
                todo!()
            }
            StatementNode::Union(_) => {
                todo!()
            }
            StatementNode::Enumerate(_) => {
                todo!()
            }
            StatementNode::Guard(_) => {
                todo!()
            }
            StatementNode::Trait(_) => {
                todo!()
            }
            StatementNode::Extends(_) => {
                todo!()
            }
            StatementNode::Variable(_) => {
                todo!()
            }
        }
    }
    pub async fn execute_expression_term(&mut self, expr: ExpressionNode) -> ValkyrieResult<ValkyrieValue> {
        let value = self.execute_expr(expr.body).await?;
        Ok(value)
    }
    #[async_recursion]
    pub(crate) async fn execute_expr(&mut self, expr: ExpressionKind) -> ValkyrieResult<ValkyrieValue> {
        match expr {
            ExpressionKind::Placeholder => Err(ValkyrieError::custom("Placeholder expression should never be executed")),
            ExpressionKind::GenericCall(_) => {
                todo!()
            }
            ExpressionKind::New(_) => {
                todo!()
            }
            ExpressionKind::Slot(_) => {
                todo!()
            }
            ExpressionKind::Text(v) => Ok(ValkyrieValue::UTF8String(Gc::new(v.text))),
            ExpressionKind::If(v) => self.evaluate_switch(v.as_switch()).await,
            ExpressionKind::Switch(v) => self.evaluate_switch(*v).await,
            ExpressionKind::IfLet(_) => {
                todo!()
            }
            ExpressionKind::Null(v) => {
                if v.nil {
                    Ok(ValkyrieValue::Null)
                }
                else {
                    Ok(ValkyrieValue::Null)
                }
            }
            ExpressionKind::Boolean(v) => Ok(ValkyrieValue::Boolean(v.value)),
            ExpressionKind::Number(v) => self.execute_number(*v).await,
            ExpressionKind::Symbol(v) => self.execute_symbol(*v).await,
            ExpressionKind::String(v) => self.execute_string(*v).await,
            ExpressionKind::Formatted(_) => {
                todo!()
            }
            ExpressionKind::Try(_) => {
                todo!()
            }
            ExpressionKind::Tuple(v) => self.execute_tuple(*v).await,
            ExpressionKind::Array(v) => self.execute_array(*v).await,
            ExpressionKind::ApplyCall(_) => {
                todo!()
            }
            ExpressionKind::SubscriptCall(v) => self.execute_subscript(*v).await,
            ExpressionKind::OutputReference(_) => {
                todo!()
            }
            ExpressionKind::Lambda(_) => {
                todo!()
            }
            ExpressionKind::Object(_) => {
                todo!()
            }
            ExpressionKind::Unary(_) => {
                todo!()
            }
            ExpressionKind::Infix(_) => {
                todo!()
            }
            ExpressionKind::Match(_) => {
                todo!()
            }
            ExpressionKind::Procedural(_) => {
                todo!()
            }
            ExpressionKind::ClosureCall(_) => {
                todo!()
            }
            ExpressionKind::DotCall(_) => {
                todo!()
            }
            ExpressionKind::DotMatchCall(_) => {
                todo!()
            }
        }
    }
}
