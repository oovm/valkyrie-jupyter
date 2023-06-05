use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};
use valkyrie_ast::ModifierPart;
use valkyrie_types::{ValkyrieFunction, ValkyrieValue};
use crate::{ValkyrieError, ValkyrieResult, ValkyrieVariable};

pub mod variable;
pub mod function;

#[derive(Clone, Debug)]
pub struct ValkyrieScope {
    parent: Option<Arc<Mutex<ValkyrieScope>>>,
    entries: BTreeMap<String, ValkyrieEntry>,
}

#[derive(Clone, Debug)]
pub enum ValkyrieEntry {
    Variable(Box<ValkyrieVariable>),
    Function(Box<ValkyrieFunction>),
}

impl ValkyrieScope {
    pub fn is_top(&self) -> bool {
        self.parent.is_none()
    }
}

impl Default for ValkyrieScope {
    fn default() -> Self {
        Self {
            parent: None,
            entries: Default::default(),
        }
    }
}
