use crate::{ValkyrieEntry, ValkyrieResult, ValkyrieScope, ValkyrieVM};
use async_recursion::async_recursion;
use serde_json::Value;
use std::{
    collections::BTreeMap,
    str::FromStr,
    sync::{Arc, Mutex},
};
use valkyrie_ast::{
    CallNode, ExpressionBody, ExpressionNode, IdentifierNode, LetBindNode, NamePathNode, NumberLiteralNode, PrettyPrint,
    ProgramRoot, StatementBody, StatementNode, StringLiteralNode, SubscriptNode, SubscriptTermNode, SwitchStatement, TableKind,
    TableNode,
};
use valkyrie_parser::{ReplRoot, ThisParser};
use valkyrie_types::{JsonValue, SyntaxError, ValkyrieError, ValkyrieFunction, ValkyrieTable, ValkyrieValue};

mod dispatch;
mod jmp_switch;
mod let_binding;

pub fn parse_repl(text: &str) -> ValkyrieResult<Vec<StatementNode>> {
    Ok(ReplRoot::parse_text(text)?.statements)
}

impl ValkyrieVM {
    pub async fn execute_script(&mut self, code: &str) -> ValkyrieResult<Vec<ValkyrieValue>> {
        let mut output = Vec::new();
        let code = ProgramRoot::parse_text(code)?;
        for i in code.statements {
            output.push(self.execute_statement(i).await?)
        }
        Ok(output)
    }
    pub async fn execute_statement(&mut self, stmt: StatementNode) -> ValkyrieResult<ValkyrieValue> {
        self.top_scope.execute_statement(stmt).await
    }
}

impl ValkyrieScope {
    pub(crate) async fn execute_subscript(&mut self, call: CallNode<SubscriptNode>) -> ValkyrieResult<ValkyrieValue> {
        let base = self.execute_expr(call.base).await?;
        let mut subs = vec![];
        for term in call.rest.terms {
            match term {
                SubscriptTermNode::Index(node) => subs.push(self.execute_term_expression(node).await?),
                SubscriptTermNode::Slice(node) => {
                    todo!()
                }
            }
        }
        Err(ValkyrieError::custom("Subscripting not implemented"))
    }

    pub(crate) async fn execute_number(&mut self, number: NumberLiteralNode) -> ValkyrieResult<ValkyrieValue> {
        match number.unit {
            Some(s) => match s.name.as_str() {
                "f32" => Ok(ValkyrieValue::Decimal(number.value.parse::<f64>()?)),
                "f64" => Ok(ValkyrieValue::Decimal(number.value.parse::<f64>()?)),
                _ => Err(ValkyrieError::custom(format!("Unknown unit: {}", s.name))),
            },
            None => match number.value.parse() {
                Ok(v) => Ok(ValkyrieValue::Integer(v)),
                Err(e) => Err(ValkyrieError::custom(format!("Unknown number: {}", e))),
            },
        }
    }
    pub(crate) async fn execute_symbol(&mut self, symbol: NamePathNode) -> ValkyrieResult<ValkyrieValue> {
        let mut new = symbol.clone();
        match symbol.names.len() {
            0 => Err(SyntaxError::new("Unreachable empty symbol name").with_span(&symbol.span).into()),
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
    pub(crate) async fn execute_table(&mut self, table: TableNode) -> ValkyrieResult<ValkyrieValue> {
        match table.kind {
            TableKind::Tuple => {
                todo!()
            }
            TableKind::OffsetTable => {
                todo!()
            }
            TableKind::OrdinalTable => {
                todo!()
            }
        }
    }

    pub(crate) async fn execute_string(&mut self, mut string: StringLiteralNode) -> ValkyrieResult<ValkyrieValue> {
        match &string.unit {
            Some(s) => match s.name.as_str() {
                "r" => Ok(ValkyrieValue::UTF8String(Arc::new(string.as_raw().text))),
                "re" => self.execute_regex(&string.raw),
                "sh" => self.execute_shell(&string.raw).await,
                "json" => self.execute_json(&string.raw),
                "html" => Ok(ValkyrieValue::Html(Arc::new(string.as_raw().text))),
                _ => Err(ValkyrieError::custom(format!("Unknown handler: {}", s.name))),
            },
            // TODO: template string
            None => Ok(ValkyrieValue::UTF8String(Arc::new(string.as_escaped()))),
        }
    }
    fn execute_regex(&mut self, string: &str) -> ValkyrieResult<ValkyrieValue> {
        let value = JsonValue::from_str(string)?;
        Ok(ValkyrieValue::Json(Arc::new(value)))
    }
    fn execute_json(&mut self, string: &str) -> ValkyrieResult<ValkyrieValue> {
        let value = json5::from_str(string)?;
        Ok(ValkyrieValue::Json(Arc::new(value)))
    }
    async fn execute_shell(&self, shell: &str) -> ValkyrieResult<ValkyrieValue> {
        Ok(ValkyrieValue::UTF8String(Arc::new(shell.to_string())))
    }
}
