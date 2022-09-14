// Copyright (C) 2022 Jingli Chen (Wine93), NetEase Inc.

use structopt::{clap::AppSettings, StructOpt};
use crate::app::OBM;
use crate::error::{Error, Result};

#[derive(Debug, StructOpt)]
#[structopt(
name = "run",
about = "Create a build container",
setting(AppSettings::ColoredHelp),
setting(AppSettings::NextLineHelp),
setting(AppSettings::UnifiedHelpMessage)
)]
pub(crate) struct RunOptions {
    #[structopt(long = "name")]
    /// Assign a name to the build container
    pub name: Option<String>,

    #[structopt(long = "image")]
    /// Specify image for build container
    pub image: Option<String>,

    #[structopt(long = "copy")]
    /// Copy project to container instead of bind mount a volume
    pub copy: bool,

    #[structopt(long = "rm")]
    /// Automatically remove the build container when it exits
    pub remove: bool,
}

pub(crate) struct RunCmd {
    obm: OBM,
    options: RunOptions,
}

impl RunCmd {
    pub(crate) fn new(obm: OBM, options: RunOptions) -> Self {
        Self{
            obm,
            options,
        }
    }

    pub(crate) fn run(self) -> Result<()> {
        Ok(())
    }
}
