use serde_json::Value;
use std::{
    collections::BTreeMap,
    str::FromStr,
    sync::{Arc, Mutex},
};
use valkyrie_ast::{CallNode, ExpressionBody, ExpressionNode, IdentifierNode, LetBindNode, ModifierPart, NamePathNode, NumberLiteralNode, PatternType, PrettyPrint, StatementNode, StatementType, StringLiteralNode, SubscriptNode, SubscriptTermNode, TableKind, TableNode};
use valkyrie_parser::{ReplRoot, ThisParser};
use valkyrie_types::{ValkyrieTable, ValkyrieError, ValkyrieResult, ValkyrieValue, SyntaxError, JsonValue, ValkyrieFunction};
use crate::{ValkyrieEntry, ValkyrieVM};
use async_recursion::async_recursion;
use crate::results::ValkyrieOutput;
use crate::results::ValkyrieOutput::Normal;

mod let_binding;

pub fn parse_repl(text: &str) -> ValkyrieResult<Vec<StatementNode>> {
    Ok(ReplRoot::parse_text(text)?.statements)
}

impl ValkyrieVM {
    pub async fn execute_statement(&mut self, stmt: StatementNode) -> ValkyrieOutput {
        let output = self.execute_stmt(stmt.r#type).await?;
        if stmt.end_semicolon {
            Normal(ValkyrieValue::Nothing)
        } else {
            Normal(output)
        }
    }
    pub(crate) async fn execute_stmt(&mut self, stmt: StatementType) -> ValkyrieOutput {
        match stmt {
            StatementType::Nothing => {
                Normal(ValkyrieValue::Nothing)
            }

            StatementType::Document(_) => { todo!() }
            StatementType::Namespace(_) => { todo!() }
            StatementType::Import(_) => { todo!() }
            StatementType::Class(_) => { todo!() }
            StatementType::While(_) => { todo!() }
            StatementType::For(_) => { todo!() }
            StatementType::LetBind(v) => {
                self.execute_let_bind(*v).await
            }
            StatementType::Function(_) => { todo!() }
            StatementType::Control(_) => { todo!() }
            StatementType::Expression(v) => {
                self.execute_expr_node(*v).await
            }
        }
    }
    pub(crate) async fn execute_expr_node(&mut self, expr: ExpressionNode) -> ValkyrieOutput {
        let value = self.execute_expr(expr.body).await?;
    }
    #[async_recursion]
    pub(crate) async fn execute_expr(&mut self, expr: ExpressionBody) -> ValkyrieOutput {
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
            ExpressionBody::Table(v) => {
                self.execute_table(*v).await
            }
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
            ExpressionBody::Subscript(v) => {
                self.execute_subscript(*v).await
            }
            ExpressionBody::GenericCall(_) => {
                todo!()
            }
            ExpressionBody::New(_) => { todo!() }
        }
    }
    pub(crate) async fn execute_subscript(&mut self, call: CallNode<SubscriptNode>) -> ValkyrieResult<ValkyrieValue> {
        let base = self.execute_expr(call.base).await?;
        let mut subs = vec![];
        for term in call.rest.terms {
            match term {
                SubscriptTermNode::Index(node) => {
                    subs.push(self.execute_expr_node(node).await?)
                }
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
                    _ => {
                        match self.top_scope.get_variable(&head.name)? {
                            ValkyrieEntry::Variable(v) => {
                                Ok(v.value)
                            }
                            ValkyrieEntry::Function(v) => {
                                Err(ValkyrieError::custom(format!("Symbol is a function: {:?}", v)))
                            }
                        }
                    }
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
            TableKind::OffsetTable => { todo!() }
            TableKind::OrdinalTable => { todo!() }
        }
    }

    pub(crate) async fn execute_string(&mut self, mut string: StringLiteralNode) -> ValkyrieResult<ValkyrieValue> {
        match &string.unit {
            Some(s) => match s.name.as_str() {
                "r" => Ok(ValkyrieValue::UTF8String(Arc::new(string.as_raw()))),
                "re" => self.execute_regex(&string.value),
                "sh" => self.execute_shell(&string.value).await,
                "json" => self.execute_json(&string.value),
                "html" => Ok(ValkyrieValue::Html(Arc::new(string.as_raw()))),
                _ => Err(ValkyrieError::custom(format!("Unknown handler: {}", s.name))),
            },
            // TODO: template string
            None => {
                Ok(ValkyrieValue::UTF8String(Arc::new(string.as_escaped())))
            }
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
