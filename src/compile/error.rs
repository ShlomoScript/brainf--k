use super::span::Span;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompileError {
    #[error("unmatched '[' at {span:?}")]
    UnmatchedOpenBracket { span: Span },

    #[error("unmatched ']' at {span:?}")]
    UnmatchedCloseBracket { span: Span },
}
