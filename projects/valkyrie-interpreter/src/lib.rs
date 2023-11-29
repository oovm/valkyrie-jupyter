#![feature(generator_trait)]
#![feature(try_trait_v2)]
#![feature(never_type)]

use crate::traits::ThisValidator;
use clap::{Parser, Subcommand};

mod evaluate;
mod results;
mod scope;
mod traits;

pub use crate::scope::{function::ValkyrieFunction, variable::ValkyrieVariable, ValkyrieEntry, ValkyrieScope};
pub use valkyrie_types::{ValkyrieEnumerate, ValkyrieError, ValkyrieValue};

pub struct ValkyrieVM {
    files: FileCache,
    top_scope: ValkyrieScope,
}

impl Default for ValkyrieVM {
    fn default() -> Self {
        ValkyrieVM { files: (), top_scope: ValkyrieScope::default() }
    }
}
