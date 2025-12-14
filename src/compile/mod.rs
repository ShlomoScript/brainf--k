// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Shlomo Ballew

mod error;
mod span;

use crate::vm::Instruction;
pub use error::CompileError;
use span::Span;

pub fn compile(source: &str) -> Result<(Vec<Instruction>, Vec<Span>), CompileError> {
    let mut instructions = Vec::new();
    let mut spans = Vec::new();

    let mut line = 1;
    let mut col = 0;

    let mut src = source.chars().peekable();
    while let Some(token) = src.peek() {
        col += 1;

        let span = Span::new(line, col);

        match token {
            '.' => {
                instructions.push(Instruction::Output);
                spans.push(span);
                src.next();
            }
            ',' => {
                instructions.push(Instruction::Input);
                spans.push(span);
                src.next();
            }
            '[' => {
                instructions.push(Instruction::OpenBracket { jump: 0 });
                spans.push(span);
                src.next();
            }
            ']' => {
                instructions.push(Instruction::CloseBracket { jump: 0 });
                spans.push(span);
                src.next();
            }
            '+' => {
                let mut count = 1;
                src.next();
                while let Some('+') = src.peek() {
                    count += 1;
                    src.next();
                }
                instructions.push(Instruction::Increment(count));
                spans.push(span);
            }
            '-' => {
                let mut count = 1;
                src.next();
                while let Some('-') = src.peek() {
                    count += 1;
                    src.next();
                }
                instructions.push(Instruction::Decrement(count));
                spans.push(span);
            }
            '>' => {
                let mut count = 1;
                src.next();
                while let Some('>') = src.peek() {
                    count += 1;
                    src.next();
                }
                instructions.push(Instruction::Forward(count));
                spans.push(span);
            }
            '<' => {
                let mut count = 1;
                src.next();
                while let Some('<') = src.peek() {
                    count += 1;
                    src.next();
                }
                instructions.push(Instruction::Backward(count));
                spans.push(span);
            }
            '\n' => {
                line += 1;
                col = 0;
                src.next();
            }
            _ => {
                src.next();
            }
        }
    }
    let instructions = build_jump_table(instructions, &spans)?;
    Ok((instructions, spans))
}

fn build_jump_table(
    mut code: Vec<Instruction>,
    spans: &[Span],
) -> Result<Vec<Instruction>, CompileError> {
    let mut stack = Vec::new();

    for i in 0..code.len() {
        match code[i] {
            Instruction::OpenBracket { .. } => stack.push(i),
            Instruction::CloseBracket { .. } => {
                let open = stack
                    .pop()
                    .ok_or(CompileError::UnmatchedCloseBracket { span: spans[i] })?;

                // fill jump locations
                code[open] = Instruction::OpenBracket { jump: i };
                code[i] = Instruction::CloseBracket { jump: open };
            }
            _ => {}
        }
    }

    if let Some(open) = stack.pop() {
        return Err(CompileError::UnmatchedOpenBracket { span: spans[open] });
    }

    Ok(code)
}
