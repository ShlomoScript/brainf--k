// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Shlomo Ballew

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
