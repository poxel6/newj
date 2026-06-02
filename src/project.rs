use std::io;

use crate::{
    cli::{Cli, prompt},
    languages::Languages,
    template::Template,
};

#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    pub domain: String,
    pub preset: String,
    pub language: Languages,
}

impl From<Cli> for Project {
    fn from(cli: Cli) -> Self {
        let name = cli
            .name
            .clone()
            .unwrap_or_else(|| prompt("Enter project name", "app"));

        let domain = cli
            .domain
            .clone()
            .unwrap_or_else(|| prompt("Enter package name", "org.example"));

        let preset = cli.preset;

        let language = Languages::Java;

        Self {
            name,
            domain,
            preset,
            language,
        }
    }
}

impl Project {
    pub fn init(&self) -> Result<(), io::Error> {
        Template::new(self)
    }
}
