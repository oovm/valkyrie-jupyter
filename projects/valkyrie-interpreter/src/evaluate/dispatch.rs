use super::*;
use valkyrie_ast::{ExpressionKind, ForLoop, StatementBlock, StatementKind};

impl Evaluate for StatementBlock {
    #[async_recursion]
    async fn execute(&self, vm: &ValkyrieVM, scope: &ValkyrieScope) -> Self::Result {
        // `old { new }`
        let mut new = scope.fork();
        for term in &self.terms {
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
                EvaluatedState::Fallthrough { .. } => return Ok(result),
            }
        }
        Ok(EvaluatedState::Normal(ValkyrieValue::Null))
    }
}

impl Evaluate for StatementKind {
    #[async_recursion]
    async fn execute(&self, vm: &ValkyrieVM, scope: &ValkyrieScope) -> Self::Result {
        match self {
            StatementKind::Nothing => Ok(EvaluatedState::Normal(ValkyrieValue::Null)),
            StatementKind::Document(_) => {
                todo!()
            }
            StatementKind::Namespace(_) => Ok(EvaluatedState::Normal(ValkyrieValue::Null)),
            StatementKind::Import(_) => {
                todo!()
            }
            StatementKind::Class(_) => {
                todo!()
            }
            StatementKind::While(v) => v.execute(vm, scope).await,
            StatementKind::For(v) => v.execute(vm, scope).await,
            StatementKind::Function(_) => {
                todo!()
            }
            StatementKind::Control(v) => v.execute(vm, scope).await,
            StatementKind::Expression(v) => v.execute(vm, scope).await,
            StatementKind::Annotation(_) => {
                todo!()
            }
            StatementKind::Union(_) => {
                todo!()
            }
            StatementKind::Enumerate(_) => {
                todo!()
            }
            StatementKind::Guard(_) => {
                todo!()
            }
            StatementKind::Trait(_) => {
                todo!()
            }
            StatementKind::Extends(_) => {
                todo!()
            }
            StatementKind::Variable(_) => {
                todo!()
            }
        }
    }
}

impl Evaluate for ExpressionNode {
    #[async_recursion]
    async fn execute(&self, vm: &ValkyrieVM, scope: &ValkyrieScope) -> Self::Result {
        let body = self.body.execute(vm, scope).await?;
        if self.omit { Ok(EvaluatedState::Normal(ValkyrieValue::Nothing)) } else { Ok(body) }
    }
}
impl Evaluate for ExpressionKind {
    #[async_recursion]
    async fn execute(&self, vm: &ValkyrieVM, scope: &ValkyrieScope) -> Self::Result {
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
            ExpressionKind::Text(v) => Ok(EvaluatedState::Normal(ValkyrieValue::UTF8String(Gc::new(v.text.clone())))),
            ExpressionKind::If(v) => v.as_switch().execute(vm, scope).await,
            ExpressionKind::Switch(v) => v.execute(vm, scope).await,
            ExpressionKind::IfLet(_) => {
                todo!()
            }
            ExpressionKind::Null(v) => {
                if v.nil {
                    Ok(EvaluatedState::Normal(ValkyrieValue::Null))
                }
                else {
                    Ok(EvaluatedState::Normal(ValkyrieValue::Null))
                }
            }
            ExpressionKind::Boolean(v) => Ok(EvaluatedState::Normal(ValkyrieValue::Boolean(v.value))),
            ExpressionKind::Number(v) => v.execute(vm, scope).await,
            ExpressionKind::Symbol(v) => v.execute(vm, scope).await,
            ExpressionKind::String(v) => v.execute(vm, scope).await,
            ExpressionKind::Formatted(_) => {
                todo!()
            }
            ExpressionKind::Try(_) => {
                todo!()
            }
            ExpressionKind::Tuple(v) => v.execute(vm, scope).await,
            ExpressionKind::Array(v) => todo!(),
            ExpressionKind::ApplyCall(_) => {
                todo!()
            }
            ExpressionKind::SubscriptCall(v) => v.execute(vm, scope).await,
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
