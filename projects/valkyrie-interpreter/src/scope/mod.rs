use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
};

use crate::{ValkyrieError, ValkyrieVariable};
use dashmap::DashMap;
use valkyrie_types::{ValkyrieFunction, ValkyrieValue};
pub mod function;
pub mod variable;

#[derive(Clone, Debug)]
pub struct ValkyrieScope {
    parent: Option<Arc<Mutex<ValkyrieScope>>>,
    entries: DashMap<String, ValkyrieEntry>,
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
    pub fn fork(&self) -> Self {
        Self { parent: Some(Arc::new(Mutex::new(self.clone()))), entries: Default::default() }
    }
}

impl Default for ValkyrieScope {
    fn default() -> Self {
        Self { parent: None, entries: Default::default() }
    }
}
