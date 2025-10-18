use std::{
    fs::{self, File},
    io,
};

use crate::{cli::Cli, template::Template};

#[derive(Debug, Clone, Default)]
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
    pub fn init(&self) -> Result<(), io::Error> {
        let structure = get_structure(&self);
        craete_dirs(&structure)?;
        create_files(&structure)?;
        Ok(())
    }
}

pub enum ProjectPaths {
    Root,
    MainResources,
    TestResources,
    Working,
    TestWorking,
}

#[derive(Default, Debug)]
pub struct ProjectFile {
    pub name: String,
    pub path: String,
    pub template: Template,
    project: Project,
}

impl ProjectFile {
    pub fn new(project: &Project) -> Self {
        Self {
            project: project.clone(),
            ..Default::default()
        }
    }
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.to_string();
        self
    }
    pub fn path(mut self, path: ProjectPaths) -> Self {
        let name = &self.project.name;
        let domain = &self.project.domain.replace(".", "/");

        let root = format!("{name}");
        let main = format!("{root}/src/main");
        let test = format!("{root}/src/test");
        self.path = match path {
            ProjectPaths::Root => root,
            ProjectPaths::MainResources => format!("{main}/resources"),
            ProjectPaths::TestResources => format!("{test}/resources"),
            ProjectPaths::Working => format!("{main}/java/{domain}/{name}"),
            ProjectPaths::TestWorking => format!("{test}/java/{domain}/{name}"),
        };
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
        let new_file = File::create_new(format!("{}/{}", file.path, file.name));
        if let Err(err) = new_file {
            match err.kind() {
                // path are defined in a way that
                // there is no distinction between
                // dirs and files so I discard them here.
                io::ErrorKind::IsADirectory => (),
                _ => return Err(err),
            }
        }
        fs::write(
            format!("{}/{}", file.path, file.name),
            file.template.content.as_bytes(),
        )?;
    }
    Ok(())
}

fn get_structure(project: &Project) -> Vec<ProjectFile> {
    vec![
        ProjectFile::new(project)
            .name("App.java")
            .path(ProjectPaths::Working)
            .template(Template::new("Main.java", project)),
        ProjectFile::new(project)
            .name("build.gradle.kt")
            .path(ProjectPaths::Root)
            .template(Template::new("gradle.kt", project)),
        ProjectFile::new(project)
            .name("AppTest.java")
            .path(ProjectPaths::TestWorking)
            .template(Template::new("Test.java", project)),
        ProjectFile::new(project)
            .name(".gitignore")
            .path(ProjectPaths::Root)
            .template(Template::new("gitignore", project)),
        ProjectFile::new(project).path(ProjectPaths::MainResources),
        ProjectFile::new(project).path(ProjectPaths::TestResources),
    ]
}
