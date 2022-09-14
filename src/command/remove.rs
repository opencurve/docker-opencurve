// Copyright (C) 2022 Jingli Chen (Wine93), NetEase Inc.

use structopt::{clap::AppSettings, StructOpt};
use crate::app::OBM;
use crate::error::{Error, Result};

#[derive(Debug, StructOpt)]
#[structopt(
name = "remove",
about = "Remove build container",
setting(AppSettings::ColoredHelp),
setting(AppSettings::NextLineHelp),
setting(AppSettings::UnifiedHelpMessage)
)]
pub(crate) struct RemoveOptions {
    /// Specify build container
    pub name: String,
}

pub(crate) struct RemoveCmd {
    obm: OBM,
    options: RemoveOptions,
}

impl RemoveCmd {
    pub(crate) fn new(obm: OBM, options: RemoveOptions) -> Self {
        Self{
            obm,
            options,
        }
    }

    pub(crate) fn run(self) -> Result<()> {
        Ok(())
    }
}
