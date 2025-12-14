// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Shlomo Ballew

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("data pointer overflow at instruction {ip}")]
    PointerOverflow { ip: usize },

    #[error("data pointer underflow at instruction {ip}")]
    PointerUnderflow { ip: usize },

    #[error(transparent)]
    Io(#[from] std::io::Error),
}
