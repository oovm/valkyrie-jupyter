#![feature(generator_trait)]
#![feature(try_trait_v2)]
#![feature(never_type)]
#![feature(associated_type_defaults)]

pub use crate::scope::{function::ValkyrieFunction, variable::ValkyrieVariable, ValkyrieEntry, ValkyrieScope};
use crate::traits::ThisValidator;
use clap::{Parser, Subcommand};
use valkyrie_types::FileCache;
pub use valkyrie_types::{ValkyrieEnumerate, ValkyrieError, ValkyrieValue};

mod evaluate;
mod results;
mod scope;
mod traits;

pub struct ValkyrieVM {
    files: FileCache,
    top_scope: ValkyrieScope,
    errors: Vec<ValkyrieError>,
}

impl Default for ValkyrieVM {
    fn default() -> Self {
        ValkyrieVM { files: FileCache::default(), top_scope: ValkyrieScope::default(), errors: vec![] }
    }
}

impl ValkyrieVM {
    pub fn add_error<E>(&mut self, error: E)
    where
        E: Into<ValkyrieError>,
    {
        self.errors.push(error.into())
    }
}
