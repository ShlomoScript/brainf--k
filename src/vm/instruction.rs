#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    Increment(u8),
    Decrement(u8),
    Forward(usize),
    Backward(usize),
    OpenBracket { jump: usize },
    CloseBracket { jump: usize },
    Output,
    Input,
}
