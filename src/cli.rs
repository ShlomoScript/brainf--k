// SPDX-License-Identifier: MIT OR Apache-2.0
// Copyright (c) 2025 Shlomo Ballew

use clap::Parser;
use clio::{Input, Output};

/// Brainf--k Interpreter
#[derive(Parser)]
#[clap(name = "BrainF--k")]
pub struct App {
    /// Input file
    #[arg(conflicts_with = "string")]
    pub input: Option<Input>,

    /// Output file (default stdout)
    #[arg(short, long, default_value = "-")]
    pub output: Output,

    /// Source code provided directly
    #[arg(short, long)]
    pub string: Option<String>,
}
