use serde_json::Value;
use std::{
    collections::BTreeMap,
    str::FromStr,
    sync::{Arc, Mutex},
};
use valkyrie_ast::{ExpressionBody, ExpressionNode, IdentifierNode, LetBindNode, ModifierPart, NamePathNode, NumberLiteralNode, PatternType, PrettyPrint, StatementNode, StatementType, StringLiteralNode, TableKind, TableNode};
use valkyrie_parser::{ReplRoot, ThisParser};
use valkyrie_types::{ValkyrieDataTable, ValkyrieError, ValkyrieResult, ValkyrieValue, SyntaxError, JsonValue};
use crate::ValkyrieVM;

mod let_binding;

pub struct ValkyrieScope {
    parent: Option<Arc<Mutex<ValkyrieScope>>>,
    variables: BTreeMap<String, ValkyrieVariable>,
}

pub struct ValkyrieVariable {
    /// Weathers the name can be rebinding.
    protected: bool,
    /// Weathers the value can change.
    mutable: bool,
    /// A mutable value can't be changed to a value with a different type.
    typing: Option<String>,
    /// The stored value.
    value: ValkyrieValue,
}

impl ValkyrieScope {
    pub fn define_variable<S>(&mut self, name: S, attributes: ModifierPart, value: ValkyrieValue) -> ValkyrieResult<ValkyrieValue> where S: ToString {
        let name = name.to_string();
        let out = value.clone();
        match self.variables.get(name.as_str()) {
            Some(s) => {
                if s.protected {
                    Err(ValkyrieError::custom(format!("Variable {} can't rebind", name)))?
                }
            }
            None => {}
        }
        let protected = attributes.contains("final");
        let mutable = attributes.contains("mut");
        let var = ValkyrieVariable {
            protected,
            mutable,
            typing: None,
            value,
        };
        match self.variables.insert(name.to_string(), var) {
            Some(_) => {}
            None => {}
        }
        Ok(out)
    }
    pub fn get_variable(&self, name: &str) -> ValkyrieResult<ValkyrieValue> {
        match self.variables.get(name) {
            Some(s) => {
                Ok(s.value.clone())
            }
            None => {
                match &self.parent {
                    Some(s) => {
                        let s = s.lock().unwrap();
                        s.get_variable(name)
                    }
                    None => {
                        Err(ValkyrieError::custom(format!("Undefined symbol: {}", name)))?
                    }
                }
            }
        }
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
                        self.top_scope.get_variable(&head.name)
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
        let value = JsonValue::from_str(string)?;
        Ok(ValkyrieValue::Json(Arc::new(value)))
    }
    async fn execute_shell(&self, shell: &str) -> ValkyrieResult<ValkyrieValue> {
        Ok(ValkyrieValue::UTF8String(Arc::new(shell.to_string())))
    }
}
