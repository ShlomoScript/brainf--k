// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Shlomo Ballew

#[derive(Debug, Clone, Copy)]
pub struct Span {
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(line: usize, column: usize) -> Self {
        Self { line, column }
    }
}
