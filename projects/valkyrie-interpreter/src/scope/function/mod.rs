use super::*;

/// A function.
pub struct ValkyrieFunction {
    /// Weathers the name can be rebinding.
    pub protected: bool,
    /// Weathers the value can change.
    pub mutable: bool,
    /// A mutable value can't be changed to a value with a different type.
    pub typing: Option<String>,
    /// The stored value.
    pub value: ValkyrieValue,
}
