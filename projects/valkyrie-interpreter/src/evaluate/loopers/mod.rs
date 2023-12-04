use super::*;

impl Evaluate for ForLoop {
    #[async_recursion]
    async fn execute(&self, vm: &ValkyrieVM, scope: &ValkyrieScope) -> Self::Result {
        let Self { pattern, iterator, condition, label, body, span } = self;

        loop {
            let guard = match &condition {
                Some(s) => s.execute(vm, scope).await?,
                None => EvaluatedState::Normal(ValkyrieValue::Boolean(true)),
            };
            match guard {
                EvaluatedState::Normal(v) => {
                    // if !v.is_truthy() {
                    //     break;
                    // }
                    v;
                }
                _ => return Ok(guard),
            }
            let result = body.execute(vm, scope).await?;
            match body.execute(vm, scope).await? {
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
                EvaluatedState::Fallthrough { .. } => return Ok(result),
            }
        }
        Ok(EvaluatedState::Normal(ValkyrieValue::Null))
    }
}
