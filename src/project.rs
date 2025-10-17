use std::{
    fs::{self, File},
    io,
};

use crate::{cli::Cli, template::Template};

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub domain: String,
}

impl From<Cli> for Project {
    fn from(cli: Cli) -> Self {
        Self {
            name: cli.name,
            domain: cli.domain,
        }
    }
}

impl Project {
    pub fn init(&self) -> Result<Self, io::Error> {
        let name = &self.name;
        let domain = &self.domain.replace(".", "/");
        let root = format!("{name}/src");
        let java = format!("java/{domain}/{name}");
        let main = format!("{root}/main/{java}");
        let test = format!("{root}/test/{java}");
        let main_resources = format!("{root}/main/resources");
        let test_resources = format!("{root}/test/resources");

        let structure = vec![
            ProjectFile::new()
                .name("App.java")
                .path(&main)
                .template(Template::new("Main.java", &self)),
            ProjectFile::new()
                .name("build.gradle.kt")
                .path(&root)
                .template(Template::new("gradle.kt", &self)),
            ProjectFile::new()
                .name("AppTest.java")
                .path(&test)
                .template(Template::new("Test.java", &self)),
            ProjectFile::new()
                .name(".gitignore")
                .path(&root)
                .template(Template::new("gitignore", &self)),
            ProjectFile::new().path(&main_resources),
            ProjectFile::new().path(&test_resources),
        ];
        craete_dirs(&structure)?;
        create_files(&structure)?;
        Ok(Self {
            name: self.name.clone(),
            domain: self.domain.clone(),
        })
    }
}

#[derive(Default, Debug)]
pub struct ProjectFile {
    pub name: String,
    pub path: String,
    pub template: Template,
}

impl ProjectFile {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
    pub fn path(mut self, path: &str) -> Self {
        self.path = format!("{}", path);
        self
    }
    pub fn template(mut self, template: Template) -> Self {
        self.template = template;
        self
    }
}

fn craete_dirs(files: &Vec<ProjectFile>) -> Result<(), io::Error> {
    for file in files {
        fs::create_dir_all(&file.path)?;
    }
    Ok(())
}

fn create_files(files: &Vec<ProjectFile>) -> Result<(), io::Error> {
    for file in files {
        File::create_new(format!("{}/{}", file.path, file.name))?;
        let write_result = fs::write(
            format!("{}/{}", file.path, file.name),
            file.template.content.as_bytes(),
        );
        if let Err(err) = write_result {
            match err.kind() {
                // path are defined in a way that
                // there is no distinction between
                // dirs and files so I discard them here.
                io::ErrorKind::IsADirectory =>  println!("it ballright"),
                _ =>  (),
            }
        }
    }
    Ok(())
}
