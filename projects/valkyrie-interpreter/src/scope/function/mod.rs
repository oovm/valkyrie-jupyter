use super::*;

pub struct ValkyrieFunction {
    /// Weathers the name can be rebinding.
    protected: bool,
    /// Weathers the value can change.
    mutable: bool,
    /// A mutable value can't be changed to a value with a different type.
    typing: Option<String>,
    /// The stored value.
    value: ValkyrieValue,
}