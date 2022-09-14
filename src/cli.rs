// Copyright (C) 2022 Jingli Chen (Wine93), NetEase Inc.

use structopt::{clap::AppSettings, StructOpt};
use crate::app::OBM;
use crate::error::Result;
use crate::command::list::{ListCmd, ListOptions};
use crate::command::run::{RunCmd, RunOptions};
use crate::command::enter::{EnterCmd, EnterOptions};
use crate::command::remove::{RemoveCmd, RemoveOptions};

#[derive(Debug, StructOpt)]
#[structopt(
name = "obm",
about = "OpenCurve Project Build Manager",
setting(AppSettings::ColoredHelp),
setting(AppSettings::NextLineHelp),
setting(AppSettings::UnifiedHelpMessage)
)]
pub(crate) enum Command {
    #[structopt(name = "run")]
    RunOptions(RunOptions),

    #[structopt(name = "ls")]
    ListOptions(ListOptions),

    #[structopt(name = "cd")]
    EnterOptions(EnterOptions),

    #[structopt(name = "rm")]
    RemoveOptions(RemoveOptions),
}

pub(crate) struct Cli {
    obm: OBM,
}

impl Cli {
    pub fn new(obm: OBM) -> Self {
        Self{ obm }
    }

    pub fn execute(self) -> Result<()> {
        let args = Command::from_args();

        match args {
            Command::RunOptions(options) => {
                RunCmd::new(self.obm, options).run()
            },
            Command::ListOptions(options) => {
                ListCmd::new(self.obm, options).run()
            },
            Command::EnterOptions(options) => {
                EnterCmd::new(self.obm, options).run()
            },
            Command::RemoveOptions(options) => {
                RemoveCmd::new(self.obm, options).run()
            },
            _ => Ok(()),
        }
    }
}
