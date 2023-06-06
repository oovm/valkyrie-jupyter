use std::convert::Infallible;
use std::ops::FromResidual;
use valkyrie_types::ValkyrieValue;
use crate::ValkyrieError;

pub enum ValkyrieOutput {
    Normal(ValkyrieValue),
    Return(ValkyrieValue),
    Throw(ValkyrieValue),
    Break(String),
    Continue(String),
    Error(ValkyrieError),
}

impl FromResidual<Result<Infallible, ValkyrieError>> for ValkyrieOutput {
    fn from_residual(residual: Result<Infallible, ValkyrieError>) -> Self {
        match residual {
            Ok(_) => unreachable!(),
            Err(e) => {
                ValkyrieOutput::Error(e)
            }
        }
    }
}