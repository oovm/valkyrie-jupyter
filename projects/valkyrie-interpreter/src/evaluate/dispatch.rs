use super::*;
use valkyrie_ast::{ExpressionKind, ForLoop, StatementBlock};

impl Evaluate for ForLoop {
    async fn execute(self, vm: &mut ValkyrieVM, scope: &mut ValkyrieScope) -> Self::Result {
        let Self { pattern, iterator, condition, body, span } = self;

        loop {
            let guard = match condition {
                Some(s) => s.execute(vm, scope).await?,
                None => Ok(EvaluatedState::Normal(ValkyrieValue::Boolean(true))),
            };

            let cond_result = self.execute_while_cond(condition).await?;
            match cond_result {
                EvaluatedState::Normal(v) => {
                    if !v.is_truthy() {
                        break;
                    }
                }
                _ => return Ok(cond_result),
            }
            let result = body.execute(vm, scope).await?;
            match body.execute(vm, scope) {
                // 从头开始
                EvaluatedState::Normal(_) => continue,
                // 从头开始
                EvaluatedState::Continue => continue,
                // 结束循环
                EvaluatedState::Break => break,
                // 结束循环, 结束函数
                EvaluatedState::Return(_) => return Ok(result),
                // 结束循环, 结束函数
                EvaluatedState::Raise(_) => return Ok(result),
            }
        }
        Ok(EvaluatedState::Normal(ValkyrieValue::Null))
    }
}

impl Evaluate for StatementBlock {
    async fn execute(self, vm: &mut ValkyrieVM, scope: &mut ValkyrieScope) -> Self::Result {
        // `old { new }`
        let mut new = scope.fork();
        for term in self.terms {
            let result = term.execute(vm, &mut new).await?;
            match result {
                // 运行下一句
                EvaluatedState::Normal(_) => continue,
                // 停止运行下一句, 从头开始
                EvaluatedState::Continue => break,
                // 停止运行下一句, 结束段落
                EvaluatedState::Break => return Ok(result),
                // 停止运行下一句, 结束函数
                EvaluatedState::Return(_) => return Ok(result),
                // 停止运行下一句, 结束函数
                EvaluatedState::Raise(_) => return Ok(result),
            }
        }
        Ok(EvaluatedState::Normal(ValkyrieValue::Null))
    }
}

impl Evaluate for StatementNode {
    async fn execute(self, vm: &mut ValkyrieVM, scope: &mut ValkyrieScope) -> Self::Result {
        match self {
            StatementNode::Nothing => Ok(EvaluatedState::Normal(ValkyrieValue::Null)),
            StatementNode::Document(_) => {
                todo!()
            }
            StatementNode::Namespace(_) => Ok(EvaluatedState::Normal(ValkyrieValue::Null)),
            StatementNode::Import(_) => {
                todo!()
            }
            StatementNode::Class(_) => {
                todo!()
            }
            StatementNode::While(v) => v.execute(vm, scope)?,
            StatementNode::For(v) => v.execute(vm, scope)?,
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
}

impl Evaluate for ExpressionNode {
    async fn execute(self, vm: &mut ValkyrieVM, scope: &mut ValkyrieScope) -> Self::Result {
        let value = self.execute_expr(expr.body).await?;
        Ok(value)
    }
}
impl Evaluate for ExpressionKind {
    async fn execute(self, vm: &mut ValkyrieVM, scope: &mut ValkyrieScope) -> Self::Result {
        match self {
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
