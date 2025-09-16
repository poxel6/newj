use crate::strings::Strings;
use std::{
    fs::{self},
    io::{self},
};

#[derive(Debug)]
pub struct PrjectStructure {
    pub directories: Vec<String>,
    pub files: Vec<String>,
}

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub domain: String,
    // pub modular: bool,
}

impl Project {
    pub fn new(name: &String, domain: &String) -> Result<Self, io::Error> {
        let project = Self {
            name: name.to_string(),
            domain: domain.to_string(),
        };
        PrjectStructure::new(&project)?;
        Ok(project)
    }
}

impl PrjectStructure {
    pub fn create_dirs(&self) {
        for dir in &self.directories {
            _ = fs::create_dir_all(&dir);
        }
    }
    pub fn create_files(&self) {
        for file in &self.files {
            _ = fs::File::create_new(&file);
        }
    }

    pub fn new(project: &Project) -> Result<Self, io::Error> {
        let module = "";
        let root = format!("{}/{module}/src", project.name);
        // let root = if modular { 
        //     format!("{}/src", project.name)
        // }
        // else {
        //     format!("{}/app/src", project.name)
        // };
        let language = "java";
        let domain_path = project.domain.replace(".", "/");
        let name = &project.name;
        let main = format!(
            "{root}/main/{language}/{domain_path}/{}",
            name.to_lowercase()
        );
        let test = format!(
            "{root}/test/{language}/{domain_path}/{}",
            name.to_lowercase()
        );
        let project = Self {
            directories: vec![format!("{main}"), format!("{test}")],
            files: vec![
                format!("{root}/build.gradle.kt"),
                format!("{root}/.gitignore"),
                format!("{root}/gradle.properties"),
                format!("{main}/App.java"),
                format!("{main}/{}.java", name.to_capitilize()),
                format!("{test}/{}Test.java", name.to_capitilize()),
            ],
        };

        project.create_dirs();
        project.create_files();

        Ok(project)
    }
}
