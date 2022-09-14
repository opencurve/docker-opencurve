// Copyright (C) 2022 Jingli Chen (Wine93), NetEase Inc.

pub(crate) mod shell;
pub(crate) mod docker;

use handlebars::Handlebars;
use std::collections::BTreeMap;
pub(crate) use crate::error::Result;

pub(crate) trait Build {
    fn to_string(&mut self) -> String;
}

pub(crate) struct Command {
    template: String,
    options: Vec<String>,
    data: BTreeMap<String, String>
}

impl Command {
    pub fn new(template: String) -> Self {
        Self {
            template: template,
            options: Vec::new(),
            data: BTreeMap::new()
        }
    }

    pub fn add_option(&mut self, option: String)  {
        self.options.push(option);
    }

    pub fn set_data(&mut self, k: String, v: String)  {
        self.data[k] = v
    }

    pub fn to_string(&mut self) -> String {
        let mut registry = Handlebars::new();
        self.data["options"] = self.options.join(" ");
        registry.render_template(self.template, &self.data).unwrap()
    }
}
