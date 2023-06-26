use super::*;
use valkyrie_ast::ExpressionType;

impl ValkyrieScope {
    pub async fn execute_statement(&mut self, stmt: StatementNode) -> ValkyrieResult<ValkyrieValue> {
        let output = self.execute_stmt(stmt.r#type).await?;
        if stmt.end_semicolon { Ok(ValkyrieValue::Nothing) } else { Ok(output) }
    }
    pub(crate) async fn execute_stmt(&mut self, stmt: StatementType) -> ValkyrieResult<ValkyrieValue> {
        match stmt {
            StatementType::Nothing => Ok(ValkyrieValue::Nothing),
            StatementType::Document(_) => {
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
            StatementType::While(_) => {
                todo!()
            }
            StatementType::For(_) => {
                todo!()
            }
            StatementType::LetBind(v) => self.execute_let_bind(*v).await,
            StatementType::Function(_) => {
                todo!()
            }
            StatementType::Control(_) => {
                todo!()
            }
            StatementType::Expression(v) => self.execute_term_expression(*v).await,
            StatementType::Annotation(_) => {
                todo!()
            }
            StatementType::ClassField(_) => {
                todo!()
            }
            StatementType::ClassMethod(_) => {
                todo!()
            }
            StatementType::Union(_) => {
                todo!()
            }
            StatementType::UnionField(_) => {
                todo!()
            }
            StatementType::Enumerate(_) => {
                todo!()
            }
            StatementType::EnumerateField(_) => {
                todo!()
            }
            StatementType::Flags(_) => {
                todo!()
            }
            StatementType::Tagged(_) => {
                todo!()
            }
            StatementType::Variant(_) => {
                todo!()
            }
            StatementType::Guard(_) => {
                todo!()
            }
            StatementType::GuardLet(_) => {
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
            ExpressionType::Number(v) => self.execute_number(*v).await,
            ExpressionType::Symbol(v) => self.execute_symbol(*v).await,
            ExpressionType::String(v) => self.execute_string(*v).await,
            ExpressionType::Table(v) => self.execute_table(*v).await,
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
            ExpressionType::Text(v) => Ok(ValkyrieValue::UTF8String(Arc::new(v.text))),
            ExpressionType::Resume(_) => {
                todo!()
            }
            ExpressionType::If(v) => self.evaluate_switch(v.as_switch()).await,
            ExpressionType::Switch(v) => self.evaluate_switch(*v).await,
            ExpressionType::IfLet(_) => {
                todo!()
            }
        }
    }
}
