// Copyright (C) 2022 Jingli Chen (Wine93), NetEase Inc.

use structopt::{clap::AppSettings, StructOpt};
use crate::app::OBM;
use crate::error::{Error, Result};

#[derive(Debug, StructOpt)]
#[structopt(
name = "enter",
about = "Enter build container",
setting(AppSettings::ColoredHelp),
setting(AppSettings::NextLineHelp),
setting(AppSettings::UnifiedHelpMessage)
)]
pub(crate) struct EnterOptions {
    /// Specify build container
    pub name: String,
}

pub(crate) struct EnterCmd {
    obm: OBM,
    options: EnterOptions,
}

impl EnterCmd {
    pub(crate) fn new(obm: OBM, options: EnterOptions) -> Self {
        Self{
            obm,
            options,
        }
    }

    pub(crate) fn run(self) -> Result<()> {
        Ok(())
    }
}
