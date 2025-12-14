use crate::compile::CompileError;
use crate::vm::RuntimeError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BfError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("compile error: {0}")]
    Compile(#[from] CompileError),

    #[error("runtime error: {0}")]
    Runtime(#[from] RuntimeError),
}
