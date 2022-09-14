// Copyright (C) 2022 Jingli Chen (Wine93), NetEase Inc.

mod app;
mod command;
mod error;
mod shell;
pub mod cli;

pub(crate) use error::Result;
use cli::Cli;

fn main() {
    let obm = app::OBM::new();
    let cli = cli::Cli::new(obm);
    cli.execute();
}
