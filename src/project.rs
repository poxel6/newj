use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

use crate::{
    cli::{Cli, prompt},
    languages::Languages,
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
        let structure = get_structure();
        craete_dirs(&structure, &self)?;
        create_files(&structure, &self)?;
        Ok(())
    }
}

fn craete_dirs(dirs: &Vec<PathBuf>, project: &Project) -> Result<(), io::Error> {
    for dir in dirs {
        let dir = dir.parent().unwrap();
        if dir.is_dir() {
            fs::create_dir_all(replace_placeholders(project, dir))?;
        }
    }
    Ok(())
}

fn create_files(files: &Vec<PathBuf>, project: &Project) -> Result<(), io::Error> {
    for file in files {
        File::create_new(replace_placeholders(project, file))?;
    }
    Ok(())
}

fn replace_placeholders(project: &Project, path: &Path) -> String {
    let domain = project.domain.split_once(".").unwrap();
    let path = path
        .to_str()
        .unwrap()
        .replace("template", &project.name)
        .replace("project-name", &project.name)
        .replace("org", domain.0)
        .replace("example", domain.1);
    path
}

fn get_structure() -> Vec<PathBuf> {
    let paths = fs::read_dir("template").unwrap();
    paths
        .into_iter()
        .map(|path| get_childs(path.unwrap().path()))
        .flatten()
        .collect()
}

fn get_childs(path: PathBuf) -> Vec<PathBuf> {
    let mut files = vec![];
    if path.is_dir() {
        for file in fs::read_dir(&path).unwrap() {
            (&mut files).append(&mut get_childs(file.unwrap().path()))
        }
    } else {
        files.push(path);
    }

    files.clone()
}
