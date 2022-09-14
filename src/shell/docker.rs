// Copyright (C) 2022 Jingli Chen (Wine93), NetEase Inc.

use askama::Template;
use handlebars::Handlebars;
use crate::shell::{BuildCommand, Options};

#[derive(Default)]
pub(crate) struct CreateContainer {
    pub image: String,
    pub command: String,
    pub name: String,
    pub remove: bool,
}

impl BuildCommand for CreateContainer {
    fn to_string(&mut self) -> String {
        let mut reg = Handlebars::new();

        let mut options = Options::new();
        if self.remove {
            options.add(String::from("--rm"));
        }
        self.options = options.to_string();

        return self.render().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use crate::shell::docker::CreateContainer;

    #[test]
    fn test_create_container() {
        let cases: Vec<(CreateContainer, String)> = vec![
            (
                CreateContainer{
                    image: "opencurvedocker/curvebs:v1.2".to_string(),
                    command: "/bin/bash".to_string(),
                    ..Default::default()
                },
                "docker create opencurvedocker/curvebs:v1.2 /bin/bash".to_string()
            ),
        ];

        for (idx, (cmd, result)) in cases.into_iter().enumerate() {
            assert_eq!(cmd.to_string(), result, "case {}", idx);
        }
    }
}



