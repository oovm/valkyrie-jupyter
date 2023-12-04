#![feature(generator_trait)]
#![feature(try_trait_v2)]
#![feature(never_type)]
#![feature(associated_type_defaults)]

pub use crate::scope::{function::ValkyrieFunction, variable::ValkyrieVariable, ValkyrieEntry, ValkyrieScope};
use crate::traits::ThisValidator;
use clap::{Parser, Subcommand};
use std::{io::Error, path::Path};
use valkyrie_types::{FileCache, FileID};
pub use valkyrie_types::{ValkyrieEnumerate, ValkyrieError, ValkyrieValue};

mod evaluate;
mod results;
mod scope;
mod traits;

pub struct ValkyrieVM {
    files: FileCache,
    top_scope: ValkyrieScope,
}

impl AsRef<FileCache> for ValkyrieVM {
    fn as_ref(&self) -> &FileCache {
        &self.files
    }
}

impl Default for ValkyrieVM {
    fn default() -> Self {
        ValkyrieVM { files: FileCache::default(), top_scope: ValkyrieScope::default() }
    }
}

impl ValkyrieVM {
    pub fn load_snippet(&mut self, snippet: &str, name: &str) -> FileID {
        self.files.load_text(snippet, name)
    }
    pub fn load_file<P: AsRef<Path>>(&mut self, path: P) -> Result<FileID, Error> {
        self.files.load_local(path)
    }
}
