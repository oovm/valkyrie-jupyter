#![feature(generator_trait)]

use crate::traits::ThisValidator;
use clap::{Parser, Subcommand};

use serde_json::Value;
use std::path::PathBuf;
use valkyrie_ast::{StatementNode, StatementType};
use valkyrie_parser::ThisParser;

mod evaluate;
mod traits;

pub use valkyrie_types::{ValkyrieValue};
pub use valkyrie_types::{ValkyrieError, ValkyrieResult};
pub use crate::evaluate::parse_repl;
use crate::evaluate::ValkyrieScope;


pub struct ValkyrieVM {
    top_scope: ValkyrieScope,
}


impl Default for ValkyrieVM {
    fn default() -> Self {
        ValkyrieVM { top_scope: ValkyrieScope::default() }
    }
}