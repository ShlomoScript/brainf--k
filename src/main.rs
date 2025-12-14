use clap::Parser;
use clio::{Input, Output};
use std::io::{stdin, Read, Write};
use thiserror::Error;

fn main() -> Result<(), BfError> {
    let args = App::parse();

    let mut source = String::new();

    if let Some(mut file) = args.input {
        // CASE 1: --input FILE   (file)
        file.read_to_string(&mut source)?;
    } else if let Some(s) = args.string {
        // CASE 2: --string "<bf>"
        source = s;
    } else {
        // CASE 3: no args → try stdin (pipe)
        let mut stdin = Input::std(); // clio stdin reader
        stdin.read_to_string(&mut source)?;
    }

    // Compile and run
    let (code, _spans) = compile(&source)?;
    let mut vm = Interpreter::new(code, args.output);

    vm.run()?;

    Ok(())
}

/// Brainf--k Interpreter
#[derive(Parser)]
#[clap(name = "BrainF--k")]
struct App {
    /// Input file
    #[arg(conflicts_with = "string")]
    input: Option<Input>,

    /// Output file (default stdout)
    #[arg(short, long, default_value = "-")]
    output: Output,

    /// Source code provided directly
    #[arg(short, long)]
    string: Option<String>,
}

fn compile(source: &str) -> Result<(Vec<Instruction>, Vec<Span>), BfError> {
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
    println!("compiled");
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

struct Interpreter {
    source: Vec<Instruction>,
    tape: [u8; 30000],
    pointer: usize,
    index: usize,
    output: Output,
}
impl Interpreter {
    pub fn new(source: Vec<Instruction>, output: Output) -> Self {
        Self {
            source,
            tape: [0; 30000],
            pointer: 0,
            index: 0,
            output,
        }
    }
    pub fn run(&mut self) -> Result<(), RuntimeError> {
        use Instruction::*;
        let mut input = String::new();
        loop {
            let instruction = match self.source.get(self.index) {
                Some(i) => i,
                None => break,
            };
            match instruction {
                Increment(n) => {
                    *self.tape.get_mut(self.pointer).unwrap() =
                        self.tape[self.pointer].wrapping_add(*n)
                }
                Decrement(n) => {
                    *self.tape.get_mut(self.pointer).unwrap() =
                        self.tape[self.pointer].wrapping_sub(*n)
                }
                Forward(n) => {
                    self.pointer = self
                        .pointer
                        .checked_add(*n)
                        .ok_or(RuntimeError::PointerOverflow { ip: self.index })?;
                }
                Backward(n) => {
                    self.pointer = self
                        .pointer
                        .checked_sub(*n)
                        .ok_or(RuntimeError::PointerUnderflow { ip: self.index })?;
                }
                OpenBracket { jump } => {
                    if self.tape[self.pointer] == 0 {
                        self.index = *jump;
                    }
                }
                CloseBracket { jump } => {
                    if self.tape[self.pointer] != 0 {
                        self.index = *jump;
                    }
                }
                Output => {
                    self.output.write_all(&[self.tape[self.pointer]])?;
                    self.output.flush()?;
                }
                Input => {
                    input.clear();
                    stdin().read_line(&mut input)?;

                    self.tape[self.pointer] = input.chars().next().unwrap_or('\0') as u8;
                }
            }
            self.index += 1;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Increment(u8),
    Decrement(u8),
    Forward(usize),
    Backward(usize),
    OpenBracket { jump: usize },
    CloseBracket { jump: usize },
    Output,
    Input,
}

#[derive(Debug, Clone, Copy)]
struct Span {
    pub line: usize,
    pub column: usize,
}

impl Span {
    fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}

#[derive(Error, Debug)]
enum BfError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("compile error: {0}")]
    Compile(#[from] CompileError),

    #[error("runtime error: {0}")]
    Runtime(#[from] RuntimeError),
}
#[derive(Error, Debug)]
enum CompileError {
    #[error("unmatched '[' at {span:?}")]
    UnmatchedOpenBracket { span: Span },

    #[error("unmatched ']' at {span:?}")]
    UnmatchedCloseBracket { span: Span },
}

#[derive(Error, Debug)]
enum RuntimeError {
    #[error("data pointer overflow at instruction {ip}")]
    PointerOverflow { ip: usize },

    #[error("data pointer underflow at instruction {ip}")]
    PointerUnderflow { ip: usize },

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
