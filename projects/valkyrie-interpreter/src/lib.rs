#![feature(generator_trait)]
#![feature(try_trait_v2)]
#![feature(never_type)]

use crate::traits::ThisValidator;
use clap::{Parser, Subcommand};

use serde_json::Value;
use std::path::PathBuf;
use valkyrie_ast::StatementNode;
// use valkyrie_parser::ThisParser;
use valkyrie_antlr;

mod evaluate;
mod results;
mod scope;
mod traits;

pub use crate::scope::{function::ValkyrieFunction, variable::ValkyrieVariable, ValkyrieEntry, ValkyrieScope};
pub use valkyrie_types::{ValkyrieError, ValkyrieResult, ValkyrieValue};

pub struct ValkyrieVM {
    top_scope: ValkyrieScope,
}

impl Default for ValkyrieVM {
    fn default() -> Self {
        ValkyrieVM { top_scope: ValkyrieScope::default() }
    }
}
