use super::*;
use valkyrie_ast::ExpressionType;

impl ValkyrieScope {
    pub async fn execute_statement(&mut self, stmt: StatementNode) -> ValkyrieResult<ValkyrieValue> {
        match stmt {
            StatementNode::Nothing => Ok(ValkyrieValue::Nothing),
            StatementNode::Document(_) => {
                todo!()
            }
            StatementNode::Namespace(_) => {
                todo!()
            }
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
            StatementNode::LetBind(v) => self.execute_let_bind(*v).await,
            StatementNode::Function(_) => {
                todo!()
            }
            StatementNode::Control(_) => {
                todo!()
            }
            StatementNode::Expression(v) => self.execute_term_expression(*v).await,
            StatementNode::Annotation(_) => {
                todo!()
            }
            StatementNode::Union(_) => {
                todo!()
            }
            StatementNode::UnionField(_) => {
                todo!()
            }
            StatementNode::Enumerate(_) => {
                todo!()
            }
            StatementNode::EnumerateField(_) => {
                todo!()
            }
            StatementNode::Flags(_) => {
                todo!()
            }
            StatementNode::Tagged(_) => {
                todo!()
            }
            StatementNode::Variant(_) => {
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
        }
    }
    pub async fn execute_term_expression(&mut self, expr: ExpressionNode) -> ValkyrieResult<ValkyrieValue> {
        let value = self.execute_expr(expr.body).await?;
        Ok(value)
    }
    #[async_recursion]
    pub(crate) async fn execute_expr(&mut self, expr: ExpressionType) -> ValkyrieResult<ValkyrieValue> {
        match expr {
            ExpressionType::Placeholder => Err(ValkyrieError::custom("Placeholder expression should never be executed")),

            ExpressionType::Prefix(_) => {
                todo!()
            }
            ExpressionType::Binary(_) => {
                todo!()
            }
            ExpressionType::Suffix(_) => {
                todo!()
            }

            ExpressionType::Apply(_) => {
                todo!()
            }
            ExpressionType::ApplyDot(_) => {
                todo!()
            }
            ExpressionType::LambdaCall(_) => {
                todo!()
            }
            ExpressionType::LambdaDot(_) => {
                todo!()
            }
            ExpressionType::Subscript(v) => self.execute_subscript(*v).await,
            ExpressionType::GenericCall(_) => {
                todo!()
            }
            ExpressionType::New(_) => {
                todo!()
            }
            ExpressionType::Slot(_) => {
                todo!()
            }
            ExpressionType::Text(v) => Ok(ValkyrieValue::UTF8String(Gc::new(v.text))),
            ExpressionType::Resume(_) => {
                todo!()
            }
            ExpressionType::If(v) => self.evaluate_switch(v.as_switch()).await,
            ExpressionType::Switch(v) => self.evaluate_switch(*v).await,
            ExpressionType::IfLet(_) => {
                todo!()
            }
            ExpressionType::Null(v) => {
                if v.nil {
                    Ok(ValkyrieValue::Null)
                }
                else {
                    Ok(ValkyrieValue::Null)
                }
            }
            ExpressionType::Boolean(v) => Ok(ValkyrieValue::Boolean(v.value)),
            ExpressionType::Number(v) => self.execute_number(*v).await,
            ExpressionType::Symbol(v) => self.execute_symbol(*v).await,
            ExpressionType::String(v) => self.execute_string(*v).await,
            ExpressionType::Table(v) => self.execute_table(*v).await,
            ExpressionType::Formatted(_) => {
                todo!()
            }
            ExpressionType::Try(_) => {
                todo!()
            }
            ExpressionType::MatchDot(_) => {
                todo!()
            }
        }
    }
}
