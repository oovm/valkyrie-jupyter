use crate::{ValkyrieEntry, ValkyrieResult, ValkyrieScope, ValkyrieVM};
use async_recursion::async_recursion;
use std::str::FromStr;
use valkyrie_antlr::ValkyrieProgramParser;
use valkyrie_ast::{
    ArrayNode, ArrayTermNode, CallNode, ExpressionNode, ExpressionType, LetBindNode, LetPattern, NamePathNode,
    NumberLiteralNode, StatementNode, StringLiteralNode, SubscriptCallNode, SwitchStatement, TupleNode,
};
use valkyrie_types::{Gc, Num, SyntaxError, ValkyrieError, ValkyrieNumber, ValkyrieValue};

mod dispatch;
mod jmp_switch;
mod let_binding;

impl ValkyrieVM {
    pub async fn execute_script(&mut self, code: &str) -> ValkyrieResult<Vec<ValkyrieValue>> {
        let mut output = Vec::new();
        for i in ValkyrieVM::parse_statements(code)? {
            output.push(self.execute_statement(i).await?)
        }
        Ok(output)
    }
    pub fn parse_statements(code: &str) -> ValkyrieResult<Vec<StatementNode>> {
        match ValkyrieProgramParser::parse(code) {
            Ok(async_recursion) => Ok(async_recursion.statements),
            Err(e) => Err(ValkyrieError::Syntax(Box::new(SyntaxError::new(e)))),
        }
    }

    pub async fn execute_statement(&mut self, stmt: StatementNode) -> ValkyrieResult<ValkyrieValue> {
        self.top_scope.execute_statement(stmt).await
    }
}

impl ValkyrieScope {
    pub(crate) async fn execute_subscript(&mut self, call: SubscriptCallNode) -> ValkyrieResult<ValkyrieValue> {
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

    pub(crate) async fn execute_number(&mut self, number: NumberLiteralNode) -> ValkyrieResult<ValkyrieValue> {
        match number.unit {
            Some(s) => match s.name.as_str() {
                "f32" => ValkyrieValue::parse_decimal(&number.value, 10),
                "f64" => ValkyrieValue::parse_decimal(&number.value, 10),
                _ => Err(ValkyrieError::custom(format!("Unknown unit: {}", s.name))),
            },
            None => Ok(ValkyrieValue::Number(ValkyrieNumber::from_str_radix(&number.value, 10)?)),
        }
    }
    pub(crate) async fn execute_symbol(&mut self, symbol: NamePathNode) -> ValkyrieResult<ValkyrieValue> {
        let mut new = symbol.clone();
        match symbol.names.len() {
            0 => Err(SyntaxError::new("Unreachable empty symbol name").with_span(&symbol.get_range()).into()),
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
    pub(crate) async fn execute_tuple(&mut self, table: TupleNode) -> ValkyrieResult<ValkyrieValue> {
        Err(ValkyrieError::custom(format!("Unknown execute_tuple: {:?}", table)))
    }
    pub(crate) async fn execute_array(&mut self, table: ArrayNode) -> ValkyrieResult<ValkyrieValue> {
        Err(ValkyrieError::custom(format!("Unknown execute_array: {:?}", table)))
    }
    pub(crate) async fn execute_string(&mut self, mut string: StringLiteralNode) -> ValkyrieResult<ValkyrieValue> {
        match &string.handler {
            Some(s) => match s.name.as_str() {
                "r" => Ok(ValkyrieValue::UTF8String(Gc::new(string.as_raw().text))),
                "re" => self.execute_regex(&string.literal),
                "sh" => self.execute_shell(&string.literal).await,
                "json" => self.execute_json(&string.literal),
                "html" => Ok(ValkyrieValue::Html(Gc::new(string.as_raw().text))),
                _ => Err(ValkyrieError::custom(format!("Unknown handler: {}", s.name))),
            },
            // TODO: template string
            None => Ok(ValkyrieValue::UTF8String(Gc::new(string.as_escaped()))),
        }
    }
    fn execute_regex(&mut self, string: &str) -> ValkyrieResult<ValkyrieValue> {
        let value = json5::from_str(string)?;
        Ok(ValkyrieValue::UTF8String(Gc::new(value)))
    }
    fn execute_json(&mut self, string: &str) -> ValkyrieResult<ValkyrieValue> {
        Ok(json5::from_str(string)?)
    }
    async fn execute_shell(&self, shell: &str) -> ValkyrieResult<ValkyrieValue> {
        Ok(ValkyrieValue::UTF8String(Gc::new(shell.to_string())))
    }
}
