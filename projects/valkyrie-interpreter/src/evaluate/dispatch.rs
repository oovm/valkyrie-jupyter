use super::*;

impl ValkyrieScope {
    pub async fn execute_statement(&mut self, stmt: StatementNode) -> ValkyrieResult<ValkyrieValue> {
        let output = self.execute_stmt(stmt.r#type).await?;
        if stmt.end_semicolon { Ok(ValkyrieValue::Nothing) } else { Ok(output) }
    }
    pub(crate) async fn execute_stmt(&mut self, stmt: StatementBody) -> ValkyrieResult<ValkyrieValue> {
        match stmt {
            StatementBody::Nothing => Ok(ValkyrieValue::Nothing),
            StatementBody::Document(_) => {
                todo!()
            }
            StatementBody::Namespace(_) => {
                todo!()
            }
            StatementBody::Import(_) => {
                todo!()
            }
            StatementBody::Class(_) => {
                todo!()
            }
            StatementBody::While(_) => {
                todo!()
            }
            StatementBody::For(_) => {
                todo!()
            }
            StatementBody::LetBind(v) => self.execute_let_bind(*v).await,
            StatementBody::Function(_) => {
                todo!()
            }
            StatementBody::Control(_) => {
                todo!()
            }
            StatementBody::Expression(v) => self.execute_term_expression(*v).await,
            StatementBody::Annotation(_) => {
                todo!()
            }
            StatementBody::ClassField(_) => {
                todo!()
            }
            StatementBody::ClassMethod(_) => {
                todo!()
            }
            StatementBody::Union(_) => {
                todo!()
            }
            StatementBody::UnionField(_) => {
                todo!()
            }
            StatementBody::Enumerate(_) => {
                todo!()
            }
            StatementBody::EnumerateField(_) => {
                todo!()
            }
            StatementBody::Flags(_) => {
                todo!()
            }
            StatementBody::Tagged(_) => {
                todo!()
            }
            StatementBody::Variant(_) => {
                todo!()
            }
            StatementBody::Guard(_) => {
                todo!()
            }
        }
    }
    pub async fn execute_term_expression(&mut self, expr: ExpressionNode) -> ValkyrieResult<ValkyrieValue> {
        let value = self.execute_expr(expr.body).await?;
        Ok(value)
    }
    #[async_recursion]
    pub(crate) async fn execute_expr(&mut self, expr: ExpressionBody) -> ValkyrieResult<ValkyrieValue> {
        match expr {
            ExpressionBody::Placeholder => Err(ValkyrieError::custom("Placeholder expression should never be executed")),
            ExpressionBody::Prefix(_) => {
                todo!()
            }
            ExpressionBody::Binary(_) => {
                todo!()
            }
            ExpressionBody::Suffix(_) => {
                todo!()
            }
            ExpressionBody::Number(v) => self.execute_number(*v).await,
            ExpressionBody::Symbol(v) => self.execute_symbol(*v).await,
            ExpressionBody::String(v) => self.execute_string(*v).await,
            ExpressionBody::Table(v) => self.execute_table(*v).await,
            ExpressionBody::Apply(_) => {
                todo!()
            }
            ExpressionBody::ApplyDot(_) => {
                todo!()
            }
            ExpressionBody::LambdaCall(_) => {
                todo!()
            }
            ExpressionBody::LambdaDot(_) => {
                todo!()
            }
            ExpressionBody::Subscript(v) => self.execute_subscript(*v).await,
            ExpressionBody::GenericCall(_) => {
                todo!()
            }
            ExpressionBody::New(_) => {
                todo!()
            }
            ExpressionBody::Slot(_) => {
                todo!()
            }
            ExpressionBody::Text(v) => Ok(ValkyrieValue::UTF8String(Arc::new(v.text))),
            ExpressionBody::Resume(_) => {
                todo!()
            }
            ExpressionBody::If(v) => self.evaluate_switch(v.as_switch()).await,
            ExpressionBody::Switch(v) => self.evaluate_switch(*v).await,
        }
    }
}
