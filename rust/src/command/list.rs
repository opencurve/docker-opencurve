// Copyright (C) 2022 Jingli Chen (Wine93), NetEase Inc.

use structopt::{clap::AppSettings, StructOpt};
use handlebars::Handlebars;
use crate::app::OBM;
use crate::error::Result;

#[derive(Debug, StructOpt)]
#[structopt(
name = "list",
about = "List build containers",
setting(AppSettings::ColoredHelp),
setting(AppSettings::NextLineHelp),
setting(AppSettings::UnifiedHelpMessage)
)]
pub(crate) struct ListOptions {
    #[structopt(short = "v", long = "verbose")]
    /// Verbose output for list
    pub verbose: bool,
}

pub(crate) struct ListCmd {
    obm: OBM,
    options: ListOptions,
}

impl ListCmd {
    pub(crate) fn new(obm: OBM, options: ListOptions) -> Self {
        Self{
            obm,
            options,
        }
    }

    pub(crate) fn run(self) -> Result<()> {
        Ok(())
    }
}

