use crate::strings::Strings;
use std::{
    error,
    fs::{self},
};

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub domain: String,
    // pub modular: bool,
}

impl Project {
    pub fn new(name: &String, domain: &String) -> Result<(), Box<dyn error::Error>> {
        let project = Self {
            name: name.to_string(),
            domain: domain.to_string(),
        };
        let JavaProject { directories, files } = JavaProject::new(project);
        PrjectStructure::new(directories, files)?;
        Ok(())
    }
}

pub struct JavaProject {
    pub directories: Vec<ProjectDirectory>,
    pub files: Vec<ProjectFile>,
}

impl JavaProject {
    pub fn new(project: Project) -> Self {
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

        let directories = ProjectDirectory::from(vec![format!("{main}"), format!("{test}")]);
        let files = ProjectFile::from(vec![
            format!("{root}/build.gradle.kt"),
            format!("{root}/.gitignore"),
            format!("{root}/gradle.properties"),
            format!("{main}/App.java"),
            format!("{main}/{}.java", name.to_capitilize()),
            format!("{test}/{}Test.java", name.to_capitilize()),
        ]);

        Self { directories, files }
    }
}

#[derive(Debug)]
pub struct ProjectFile(pub String);

impl ProjectFile {
    fn new(name: &String) -> Self {
        Self(name.clone())
    }
    pub fn from(names: Vec<String>) -> Vec<Self> {
        names.iter().map(|n| Self::new(n)).collect()
    }
    pub fn create_all(names: Vec<String>) -> Result<(), Box<dyn error::Error>> {
        for name in names {
            fs::File::create_new(name)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct ProjectDirectory {
    pub path: String,
    pub directories: Vec<ProjectDirectory>,
    pub files: Vec<ProjectFile>,
}

impl ProjectDirectory {
    pub fn new(path: &String) -> Self {
        Self {
            path: path.clone(),
            directories: vec![],
            files: vec![],
        }
    }
    pub fn from(dirs: Vec<String>) -> Vec<Self> {
        dirs.iter().map(|d| Self::new(d)).collect()
    }
}

#[derive(Debug)]
pub struct PrjectStructure {
    pub directories: Vec<ProjectDirectory>,
    pub files: Vec<ProjectFile>,
}

impl PrjectStructure {
    pub fn new(
        directories: Vec<ProjectDirectory>,
        files: Vec<ProjectFile>,
    ) -> Result<Self, Box<dyn error::Error>> {
        for dir in &directories {
            fs::create_dir_all(dir.path.clone())?;
        }
        for file in &files {
            fs::File::create_new(file.0.clone())?;
        }
        Ok(Self { directories, files })
    }
}
