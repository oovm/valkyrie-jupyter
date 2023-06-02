use serde_json::Value;
use std::{
    collections::BTreeMap,
    str::FromStr,
    sync::{Arc, Mutex},
};
use valkyrie_ast::{ExpressionBody, ExpressionNode, LetBindNode, NamePathNode, NumberLiteralNode, PatternType, PrettyPrint, StatementNode, StatementType, StringLiteralNode};
use valkyrie_parser::{ReplRoot, ThisParser};
use valkyrie_types::{ValkyrieDataTable, ValkyrieError, ValkyrieResult, ValkyrieValue, SyntaxError, JsonValue};
use crate::ValkyrieVM;

pub struct ValkyrieScope {
    parent: Option<Arc<Mutex<ValkyrieScope>>>,
    variables: BTreeMap<String, ValkyrieValue>,
}

impl ValkyrieScope {
    pub fn define_variable<S>(&mut self, name: S, value: ValkyrieValue) -> ValkyrieResult<ValkyrieValue> where S: ToString {
        let out = value.clone();
        match self.variables.insert(name.to_string(), value) {
            Some(_) => {}
            None => {}
        }
        Ok(out)
    }
}

impl Default for ValkyrieScope {
    fn default() -> Self {
        Self {
            parent: None,
            variables: Default::default(),
        }
    }
}

pub fn parse_repl(text: &str) -> ValkyrieResult<Vec<StatementNode>> {
    Ok(ReplRoot::parse_text(text)?.statements)
}

impl ValkyrieVM {
    pub async fn execute_statement(&mut self, stmt: StatementNode) -> ValkyrieResult<ValkyrieValue> {
        let output = self.execute_stmt(stmt.r#type).await?;
        if stmt.end_semicolon {
            Ok(ValkyrieValue::Nothing)
        } else {
            Ok(output)
        }
    }
    pub(crate) async fn execute_stmt(&mut self, stmt: StatementType) -> ValkyrieResult<ValkyrieValue> {
        match stmt {
            StatementType::Nothing => {
                Ok(ValkyrieValue::Nothing)
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
    pub(crate) async fn execute_expr_node(&mut self, expr: ExpressionNode) -> ValkyrieResult<ValkyrieValue> {
        self.execute_expr(expr.body).await
    }
    pub(crate) async fn execute_let_bind(&mut self, bind: LetBindNode) -> ValkyrieResult<ValkyrieValue> {
        match bind.pattern {
            PatternType::Tuple(t) => {
                match t.as_slice() {
                    [] => {
                        Err(ValkyrieError::custom("Empty tuple patterns are not allowed"))
                    }
                    [v] => {
                        let rhs = match bind.body {
                            None => {
                                ValkyrieValue::Nothing
                            }
                            Some(v) => {
                                self.execute_expr_node(v).await?
                            }
                        };
                        self.top_scope.define_variable(&v.key.name, rhs)
                    }
                    _ => {
                        return Err(ValkyrieError::custom("Tuple patterns are not yet supported"));
                    }
                }
            }
            PatternType::Case => {
                Err(ValkyrieError::custom("Case patterns are not yet supported"))
            }
        }
    }

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
            ExpressionBody::Table(v) => {
                todo!()
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
            ExpressionBody::Subscript(_) => {
                todo!()
            }
            ExpressionBody::GenericCall(_) => {
                todo!()
            }
            ExpressionBody::New(_) => { todo!() }
        }
    }
    pub(crate) async fn execute_number(&mut self, number: NumberLiteralNode) -> ValkyrieResult<ValkyrieValue> {
        match number.unit {
            Some(s) => match s.name.as_str() {
                "f32" => Ok(ValkyrieValue::Float32(number.value.parse::<f32>()?)),
                "f64" => Ok(ValkyrieValue::Float64(number.value.parse::<f64>()?)),
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
                        match self.top_scope.variables.get(&head.name) {
                            Some(v) => Ok(v.clone()),
                            None => Err(ValkyrieError::custom(format!("Undefined symbol: {}", symbol.pretty_string(144)))),
                        }
                    }
                }
            }
            _ => Err(ValkyrieError::custom(format!("Unknown symbol: {:?}", symbol.names))),
        }
    }
    pub(crate) async fn execute_string(&mut self, mut string: StringLiteralNode) -> ValkyrieResult<ValkyrieValue> {
        match string.unit {
            Some(s) => match s.name.as_str() {
                // "re" => todo!(),
                "sh" => self.execute_shell(&string.value).await,
                "json" => self.execute_json(&string.value),
                _ => Err(ValkyrieError::custom(format!("Unknown handler: {}", s.name))),
            },
            // TODO: template string
            None => Ok(ValkyrieValue::UTF8String(Arc::new(string.value))),
        }
    }
    pub(crate) fn execute_json(&mut self, string: &str) -> ValkyrieResult<ValkyrieValue> {
        let value = JsonValue::from_str(string)?;
        Ok(ValkyrieValue::Json(Arc::new(value)))
    }
    pub(crate) async fn execute_shell(&self, shell: &str) -> ValkyrieResult<ValkyrieValue> {
        Ok(ValkyrieValue::UTF8String(Arc::new(shell.to_string())))
    }
}
