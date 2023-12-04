use crate::{ValkyrieScope, ValkyrieVM};
use async_recursion::async_recursion;
use std::{future::Future, str::FromStr};
use valkyrie_ast::{
    helper::{StringInterpreter, ValkyrieNode},
    ControlKind, ControlNode, ExpressionKind, ExpressionNode, ForLoop, FormatterNode, NamePathNode, NumberLiteralNode,
    ProgramRoot, StatementKind, StringLiteralNode, SubscriptCallNode, SwitchStatement, TupleNode, WhileConditionNode,
    WhileLoop, WhileLoopKind,
};
use valkyrie_types::{
    Failure, FileID, Gc, MissingError, Num, ProgramContext, StringFormatterBuilder, Success, SyntaxError, ValkyrieError,
    ValkyrieList, ValkyrieResult, ValkyrieValue,
};
mod dispatch;
mod jmp_switch;
mod let_binding;

mod loopers;

pub type EvaluatedResult = Result<EvaluatedState, ValkyrieError>;

pub enum EvaluatedState {
    /// A normal return
    Normal(ValkyrieValue),
    /// A early return
    Return(ValkyrieValue),
    /// Unbounded raise
    ///
    /// Generally speaking, it is due to forgetting to handle errors.
    Raise(ValkyrieValue),
    /// Break the loop controller
    Break,
    /// Continue next loop
    Continue,
    /// Fallthrough next branch
    Fallthrough {
        /// Check condition of next branch
        checked: bool,
    },
}

impl Evaluate for ControlNode {
    #[async_recursion]
    async fn execute(&self, vm: &ValkyrieVM, scope: &ValkyrieScope) -> Self::Result {
        let result = match &self.expression {
            Some(s) => s.execute(vm, scope).await?,
            None => EvaluatedState::nothing(),
        };
        let output = match result {
            EvaluatedState::Normal(v) => v,
            // return { return x }
            _ => return Ok(result),
        };

        let value = match &self.kind {
            ControlKind::Goto => Err(ValkyrieError::runtime_error("Unexpected `raise` outside of function"))?,
            ControlKind::Raise => {
                todo!()
            }
            ControlKind::Break => {
                todo!()
            }
            ControlKind::Continue => {
                todo!()
            }
            ControlKind::Fallthrough => EvaluatedState::Fallthrough { checked: true },
            ControlKind::FallthroughUnchecked => EvaluatedState::Fallthrough { checked: false },
            ControlKind::Return => EvaluatedState::Return(output),
            ControlKind::Resume => {
                todo!()
            }
            ControlKind::YieldReturn => {
                todo!()
            }
            ControlKind::YieldBreak => {
                todo!()
            }
            ControlKind::YieldFrom => {
                todo!()
            }
            ControlKind::YieldSend => {
                todo!()
            }
        };
        Ok(value)
    }
}

impl EvaluatedState {
    pub fn nothing() -> Self {
        EvaluatedState::Normal(ValkyrieValue::Nothing)
    }
}

pub trait Evaluate {
    type Result = EvaluatedResult;
    #[async_recursion]
    async fn execute(&self, vm: &ValkyrieVM, scope: &ValkyrieScope) -> Self::Result {
        panic!()
    }
}

impl Evaluate for ProgramRoot {
    type Result = Vec<Result<ValkyrieValue, ValkyrieError>>;
    #[async_recursion]
    async fn execute(&self, vm: &ValkyrieVM, scope: &ValkyrieScope) -> Self::Result {
        let mut out = Vec::with_capacity(self.statements.len());
        for term in &self.statements {
            let result = match term.execute(vm, scope).await {
                Ok(o) => o,
                Err(e) => {
                    out.push(Err(e));
                    continue;
                }
            };
            match result {
                EvaluatedState::Normal(o) => {
                    if !o.is_nothing() {
                        out.push(Ok(o))
                    }
                }
                EvaluatedState::Return(_) => {
                    out.push(Err(ValkyrieError::runtime_error("Unexpected `raise` outside of function")))
                }
                EvaluatedState::Raise(_) => {
                    out.push(Err(ValkyrieError::runtime_error("Unexpected `raise` outside of function")))
                }
                EvaluatedState::Break => out.push(Err(ValkyrieError::runtime_error("Unexpected `raise` outside of function"))),
                EvaluatedState::Continue => {
                    out.push(Err(ValkyrieError::runtime_error("Unexpected `raise` outside of function")))
                }
                EvaluatedState::Fallthrough { .. } => {
                    out.push(Err(ValkyrieError::runtime_error("Unexpected `raise` outside of function")))
                }
            }
        }
        out
    }
}

impl Evaluate for WhileLoop {
    #[async_recursion]
    async fn execute(&self, vm: &ValkyrieVM, scope: &ValkyrieScope) -> Self::Result {
        let WhileLoop { kind, condition, then, .. } = self;

        let scope = scope.fork();

        loop {
            let cond_result = condition.execute(vm, &scope).await?;
            match cond_result {
                EvaluatedState::Normal(v) => match kind {
                    WhileLoopKind::While => {
                        // if !v.is_truthy() {
                        //     break;
                        // }
                    }
                    WhileLoopKind::Until => {
                        // if v.is_truthy() {
                        //     break;
                        // }
                    }
                },
                _ => return Ok(cond_result),
            }
            for i in &then.terms {
                let body_result = i.execute(vm, &scope).await?;
                match body_result {
                    EvaluatedState::Normal(_) | EvaluatedState::Continue => continue,
                    EvaluatedState::Break => break,
                    EvaluatedState::Return(_) | EvaluatedState::Raise(_) => return Ok(body_result),
                    EvaluatedState::Fallthrough { .. } => break,
                }
            }
        }
        Ok(EvaluatedState::nothing())
    }
}

impl Evaluate for WhileConditionNode {
    #[async_recursion]
    async fn execute(&self, vm: &ValkyrieVM, scope: &ValkyrieScope) -> Self::Result {
        match self {
            WhileConditionNode::Unconditional => Ok(EvaluatedState::Normal(ValkyrieValue::Boolean(true))),
            WhileConditionNode::Expression(_) => Ok(EvaluatedState::Normal(ValkyrieValue::Boolean(true))),
            WhileConditionNode::Case(_) => Ok(EvaluatedState::Normal(ValkyrieValue::Boolean(true))),
        }
    }
}

impl ValkyrieVM {
    pub async fn execute_script(&mut self, file: FileID) -> Vec<Result<ValkyrieValue, ValkyrieError>> {
        let mut errors = vec![];
        let ctx = ProgramContext { file };
        let res = match ctx.parse(&mut self.files) {
            Success { value, diagnostics } => {
                if !diagnostics.is_empty() {
                    errors.extend(diagnostics.into_iter().map(Err));
                    return errors;
                }
                value
            }
            Failure { fatal, diagnostics } => {
                errors.extend(diagnostics.into_iter().map(Err));
                errors.push(Err(fatal));
                return errors;
            }
        };
        res.execute(self, &self.top_scope).await
    }
}

impl Evaluate for SubscriptCallNode {
    #[async_recursion]
    async fn execute(&self, vm: &ValkyrieVM, scope: &ValkyrieScope) -> Self::Result {
        // let base = self.execute_expr(call.base).await?;
        // // let mut subs = vec![];
        // for term in call.terms {
        //     match term {
        //         ArrayTermNode::Index { index } => {
        //             let _ = self.execute_expr(index).await?;
        //         }
        //         ArrayTermNode::Range { head, tail, step } => {
        //             let _ = match head {
        //                 Some(s) => self.execute_expr(s).await?,
        //                 None => ValkyrieValue::from(1),
        //             };
        //             let _ = match tail {
        //                 Some(s) => self.execute_expr(s).await?,
        //                 None => ValkyrieValue::from(-1),
        //             };
        //             let _ = match step {
        //                 Some(s) => self.execute_expr(s).await?,
        //                 None => ValkyrieValue::from(1),
        //             };
        //         }
        //     }
        // }
        Err(ValkyrieError::custom("Subscripting not implemented"))
    }
}

impl Evaluate for NumberLiteralNode {
    #[async_recursion]
    async fn execute(&self, vm: &ValkyrieVM, scope: &ValkyrieScope) -> Self::Result {
        match &self.unit {
            Some(s) => match s.name.as_str() {
                // "f32" => ValkyrieValue::parse_decimal(&number.value, 10),
                // "f64" => ValkyrieValue::parse_decimal(&number.value, 10),
                // "u8" => ValkyrieValue::parse_integer(&number.value, 10),
                // "u16" => ValkyrieValue::parse_integer(&number.value, 10),
                // "u32" => ValkyrieValue::parse_integer(&number.value, 10),
                _ => Err(ValkyrieError::custom(format!("Unknown unit: {}", s.name))),
            },
            None => Err(SyntaxError::new(format!("Can't parse number: `{}`", self)).with_span(self.span).into()),
        }
    }
}

impl Evaluate for NamePathNode {
    #[async_recursion]
    async fn execute(&self, vm: &ValkyrieVM, scope: &ValkyrieScope) -> Self::Result {
        Err(MissingError::undefined(&self.to_string()).with_span(self.span).into())
        // match self.get_variable(&head.name)? {
        //     ValkyrieEntry::Variable(v) => Ok(v.value),
        //     ValkyrieEntry::Function(v) => Err(ValkyrieError::custom(format!("Symbol is a function: {:?}", v))),
        // }
    }
}

impl Evaluate for TupleNode {
    #[async_recursion]
    async fn execute(&self, vm: &ValkyrieVM, scope: &ValkyrieScope) -> Self::Result {
        let mut list = ValkyrieList::default();
        for x in &self.terms.terms {
            let value = x.value.execute(vm, scope).await?;
            // match x.key {
            //     ArgumentKey::Nothing => list.append_one(value),
            //     ArgumentKey::Symbol(v) => list.append_named(v.name.as_str(), value)?,
            // }
        }
        Err(ValkyrieError::custom(format!("TODO: impl Evaluate for TupleNode")))
        // Ok(ValkyrieValue::List(list))
    }
}

impl Evaluate for StringLiteralNode {
    #[async_recursion]
    async fn execute(&self, vm: &ValkyrieVM, scope: &ValkyrieScope) -> Self::Result {
        match &self.handler {
            Some(s) => match s.name.as_str() {
                "r" => return Ok(EvaluatedState::Normal(ValkyrieValue::UTF8String(Gc::new(self.as_raw().text)))),
                // "re" => self.execute_regex(&self.literal),
                // "sh" => self.execute_shell(&self.literal).await,
                // "json" => self.execute_json(&self.literal),
                // "html" => Ok(ValkyrieValue::Html(Gc::new(self.as_raw().text))),
                _ => Err(ValkyrieError::custom(format!("Unknown handler: {}", s.name))),
            },
            // TODO: template string
            None => {
                let mut sf = StringFormatterBuilder::new(FileID::default());
                match sf.interpret(&self.literal) {
                    Success { value, .. } => value.execute(vm, scope).await,
                    Failure { .. } => {
                        todo!()
                    }
                }
            }
        }
    }
}
impl Evaluate for FormatterNode {
    #[async_recursion]
    async fn execute(&self, vm: &ValkyrieVM, scope: &ValkyrieScope) -> Self::Result {
        Err(ValkyrieError::custom(format!("TODO: impl Evaluate for FormatterNode")))
    }
}
fn execute_regex(string: &str) -> ValkyrieResult<ValkyrieValue> {
    Ok(ValkyrieValue::UTF8String(Gc::new(string.to_string())))
}
fn execute_json(string: &str) -> ValkyrieResult<ValkyrieValue> {
    Ok(json5::from_str(string)?)
}
async fn execute_shell(vm: &mut ValkyrieVM, shell: &str) -> ValkyrieResult<ValkyrieValue> {
    Ok(ValkyrieValue::UTF8String(Gc::new(shell.to_string())))
}
