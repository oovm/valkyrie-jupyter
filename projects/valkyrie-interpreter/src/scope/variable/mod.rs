use super::*;

#[derive(Clone, Debug)]
pub struct ValkyrieVariable {
    /// Weathers the name can be rebinding.
    pub protected: bool,
    /// Weathers the value can change.
    pub mutable: bool,
    /// A mutable value can't be changed to a value with a different type.
    pub typing: Option<String>,
    /// The stored value.
    pub value: ValkyrieValue,
}

impl ValkyrieScope {
    pub fn define_variable<S>(&mut self, name: S, attributes: ModifierPart, value: ValkyrieValue) -> ValkyrieResult<ValkyrieValue> where S: ToString {
        let name = name.to_string();
        let out = value.clone();
        match self.entries.get(name.as_str()) {
            Some(s) => {
                match s {
                    ValkyrieEntry::Variable(s) => {
                        if s.protected {
                            Err(ValkyrieError::custom(format!("Variable {} can't rebind", name)))?
                        }
                    }
                    ValkyrieEntry::Function(_) => {
                        Err(ValkyrieError::custom(format!("Variable {} can't rebind to a function", name)))?
                    }
                }
            }
            None => {}
        }
        let protected = attributes.contains("final");
        let mutable = attributes.contains("mut");
        let var = ValkyrieEntry::Variable(Box::new(ValkyrieVariable {
            protected,
            mutable,
            typing: None,
            value,
        }));
        match self.entries.insert(name.to_string(), var) {
            Some(_) => {}
            None => {}
        }
        Ok(out)
    }
    pub fn set_variable(&mut self, name: &str, value: ValkyrieEntry) -> ValkyrieResult<ValkyrieValue> {
        match self.entries.get_mut(name) {
            Some(entry) => {
                match entry {
                    ValkyrieEntry::Variable(s) => {
                        todo!()
                    }
                    ValkyrieEntry::Function(s) => {
                        Err(ValkyrieError::custom(format!("Variable {} can't be changed to a function", name)))?
                    }
                }
                todo!()
            }
            None => {
                match &self.parent {
                    Some(s) => {
                        let mut s = s.lock().unwrap();
                        s.set_variable(name, value)
                    }
                    None => {
                        Err(ValkyrieError::custom(format!("Undefined symbol: {}", name)))?
                    }
                }
            }
        }
    }

    pub fn get_variable(&self, name: &str) -> ValkyrieResult<ValkyrieEntry> {
        match self.entries.get(name) {
            Some(s) => {
                Ok(s.clone())
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
