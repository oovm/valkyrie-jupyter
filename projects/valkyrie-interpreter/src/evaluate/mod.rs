use crate::{ValkyrieEntry, ValkyrieScope, ValkyrieVM};
use async_recursion::async_recursion;
use std::{future::Future, str::FromStr};
use valkyrie_ast::{
    ExpressionNode, ForLoop, NamePathNode, NumberLiteralNode, ProgramRoot, StatementNode, StringLiteralNode, SubscriptCallNode,
    SwitchStatement, TupleNode, WhileConditionNode, WhileLoop, WhileLoopKind,
};
use valkyrie_parser::ProgramContext;
use valkyrie_types::{
    FileID, Gc, Num, ProgramContext, SyntaxError, ValkyrieError, ValkyrieList, ValkyrieNumber, ValkyrieResult, ValkyrieValue,
};

mod dispatch;
mod jmp_switch;
mod let_binding;

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
}

pub trait Evaluate {
    type Result = EvaluatedResult;
    async fn execute(self, vm: &mut ValkyrieVM, scope: &mut ValkyrieScope) -> Self::Result;
}

impl Evaluate for ProgramRoot {
    type Result = Vec<ValkyrieValue>;
    async fn execute(self, vm: &mut ValkyrieVM, scope: &mut ValkyrieScope) -> Self::Result {
        let mut output = Vec::with_capacity(self.statements.len());

        for term in &self.statements {
            match term.execute(vm, &mut vm.top_scope).await {
                Ok(_) => {}
                Err(e) => vm.errors,
            }
        }

        output
    }
}

impl Evaluate for WhileLoop {
    async fn execute(self, vm: &mut ValkyrieVM, scope: &mut ValkyrieScope) -> Self::Result {
        let WhileLoop { kind, condition, then, .. } = self;

        let mut scope = scope.fork();

        loop {
            let cond_result = self.execute_while_cond(condition).await?;
            match cond_result {
                EvaluatedState::Normal(v) => match kind {
                    WhileLoopKind::While => {
                        if !v.is_truthy() {
                            break;
                        }
                    }
                    WhileLoopKind::Until => {
                        if v.is_truthy() {
                            break;
                        }
                    }
                },
                _ => return Ok(cond_result),
            }
            for i in then.terms {
                let body_result = self.execute_statement(i).await?;
                match body_result {
                    EvaluatedState::Normal(_) | EvaluatedState::Continue => continue,
                    EvaluatedState::Break => break,
                    EvaluatedState::Return(_) | EvaluatedState::Raise(_) => return Ok(body_result),
                }
            }
        }
        Ok(EvaluatedState::Normal(ValkyrieValue::Null))
    }
}

impl Evaluate for WhileConditionNode {
    async fn execute(self, vm: &mut ValkyrieVM, scope: &mut ValkyrieScope) -> Self::Result {
        match node {
            WhileConditionNode::Unconditional => Ok(EvaluatedState::Normal(ValkyrieValue::Boolean(true))),
            WhileConditionNode::Expression(_) => Ok(EvaluatedState::Normal(ValkyrieValue::Boolean(true))),
            WhileConditionNode::Case(_) => Ok(EvaluatedState::Normal(ValkyrieValue::Boolean(true))),
        }
    }
}

impl Evaluate for StatementNode {
    async fn execute(self, vm: &mut ValkyrieVM, scope: &mut ValkyrieScope) -> Self::Result {
        scope.execute_statement(stmt).await
    }
}

impl ValkyrieVM {
    pub async fn execute_script(&mut self, file: FileID) -> ValkyrieResult<Vec<ValkyrieValue>> {
        let mut output = Vec::new();
        for i in self.parse_statements(file)? {
            output.push(self.execute_statement(i).await?)
        }
        Ok(output)
    }
    pub fn parse_statements(&mut self, file: FileID) -> ValkyrieResult<Vec<StatementNode>> {
        let ctx = ProgramContext { file };
        match ctx.parse(&mut self.files) {
            Ok(async_recursion) => Ok(async_recursion.statements),
            Err(e) => Err(ValkyrieError::from(SyntaxError::new(e.to_string()))),
        }
    }

    pub async fn execute_statement(&mut self, stmt: StatementNode) -> ValkyrieResult<EvaluatedState> {
        self.top_scope.execute_statement(stmt).await
    }
}

impl Evaluate for SubscriptCallNode {
    async fn execute(self, vm: &mut ValkyrieVM, scope: &mut ValkyrieScope) -> Self::Result {
        let base = self.execute_expr(call.base).await?;
        // let mut subs = vec![];
        for term in call.terms {
            match term {
                ArrayTermNode::Index { index } => {
                    let _ = self.execute_expr(index).await?;
                }
                ArrayTermNode::Range { head, tail, step } => {
                    let _ = match head {
                        Some(s) => self.execute_expr(s).await?,
                        None => ValkyrieValue::from(1),
                    };
                    let _ = match tail {
                        Some(s) => self.execute_expr(s).await?,
                        None => ValkyrieValue::from(-1),
                    };
                    let _ = match step {
                        Some(s) => self.execute_expr(s).await?,
                        None => ValkyrieValue::from(1),
                    };
                }
            }
        }
        Err(ValkyrieError::custom("Subscripting not implemented"))
    }
}

impl Evaluate for NumberLiteralNode {
    async fn execute(self, vm: &mut ValkyrieVM, scope: &mut ValkyrieScope) -> Self::Result {
        match number.unit {
            Some(s) => match s.name.as_str() {
                "f32" => ValkyrieValue::parse_decimal(&number.value, 10),
                "f64" => ValkyrieValue::parse_decimal(&number.value, 10),
                "u8" => ValkyrieValue::parse_integer(&number.value, 10),
                "u16" => ValkyrieValue::parse_integer(&number.value, 10),
                "u32" => ValkyrieValue::parse_integer(&number.value, 10),
                _ => Err(ValkyrieError::custom(format!("Unknown unit: {}", s.name))),
            },
            None => Ok(ValkyrieValue::Number(ValkyrieNumber::from_str_radix(&number.value, 10)?)),
        }
    }
}

impl Evaluate for NamePathNode {
    async fn execute(self, vm: &mut ValkyrieVM, scope: &mut ValkyrieScope) -> Self::Result {
        let mut new = symbol.clone();
        match symbol.names.len() {
            0 => Err(SyntaxError::new("Unreachable empty symbol name").with_range(&symbol.get_range()).into()),
            1 => {
                let head = unsafe { new.names.pop().unwrap_unchecked() };
                match head.name.as_str() {
                    "true" => Ok(ValkyrieValue::Boolean(true)),
                    "false" => Ok(ValkyrieValue::Boolean(false)),
                    "null" => Ok(ValkyrieValue::Null),
                    _ => match self.get_variable(&head.name)? {
                        ValkyrieEntry::Variable(v) => Ok(v.value),
                        ValkyrieEntry::Function(v) => Err(ValkyrieError::custom(format!("Symbol is a function: {:?}", v))),
                    },
                }
            }
            _ => Err(ValkyrieError::custom(format!("Unknown symbol: {:?}", symbol.names))),
        }
    }
}

impl Evaluate for TupleNode {
    async fn execute(self, vm: &mut ValkyrieVM, scope: &mut ValkyrieScope) -> Self::Result {
        let mut list = ValkyrieList::default();
        for x in table.terms {
            let value = self.execute_expr(x.value).await?;
            match x.key {
                TupleKeyType::Nothing => list.append_one(value),
                TupleKeyType::Identifier(v) => list.append_named(v.name.as_str(), value)?,
                // FIXME
                TupleKeyType::Number(_) => list.append_one(value),
                TupleKeyType::Subscript(_) => list.append_one(value),
            }
        }
        Ok(ValkyrieValue::List(list))
    }
}

impl Evaluate for StringLiteralNode {
    async fn execute(self, vm: &mut ValkyrieVM, scope: &mut ValkyrieScope) -> Self::Result {
        match &self.handler {
            Some(s) => match s.name.as_str() {
                "r" => Ok(ValkyrieValue::UTF8String(Gc::new(self.as_raw().text))),
                "re" => self.execute_regex(&self.literal),
                "sh" => self.execute_shell(&self.literal).await,
                "json" => self.execute_json(&self.literal),
                "html" => Ok(ValkyrieValue::Html(Gc::new(self.as_raw().text))),
                _ => Err(ValkyrieError::custom(format!("Unknown handler: {}", s.name))),
            },
            // TODO: template string
            None => Ok(ValkyrieValue::UTF8String(Gc::new(self.as_escaped()))),
        }
    }
}

fn execute_regex(&mut self, string: &str) -> ValkyrieResult<ValkyrieValue> {
    Ok(ValkyrieValue::UTF8String(Gc::new(string.to_string())))
}
fn execute_json(&mut self, string: &str) -> ValkyrieResult<ValkyrieValue> {
    Ok(json5::from_str(string)?)
}
async fn execute_shell(&self, shell: &str) -> ValkyrieResult<ValkyrieValue> {
    Ok(ValkyrieValue::UTF8String(Gc::new(shell.to_string())))
}

impl ValkyrieScope {
    pub(crate) async fn execute_array(&mut self, table: ArrayNode) -> ValkyrieResult<ValkyrieValue> {
        Err(ValkyrieError::custom(format!("Unknown execute_array: {:?}", table)))
    }
}
