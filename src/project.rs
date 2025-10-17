use std::{
    error,
    fs::{self, File},
    io,
};

use crate::template::Template;

#[derive(Debug)]
pub struct Project {
    pub name: String,
    pub domain: String,
}

impl Project {
    pub fn new(&self) -> Result<Self, Box<dyn error::Error>> {
        let name = &self.name;
        let domain = &self.domain.replace(".", "/");
        let root = format!("{name}/src");
        let java = format!("java/{domain}/{name}");
        let main = format!("{root}/main/{java}");
        let test = format!("{root}/test/{java}");
        let main_resources = format!("{root}/main/resources");
        let test_resources = format!("{root}/test/resources");

        Structure::from(vec![
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
            ProjectFile::new().path(&main_resources),
            ProjectFile::new().path(&test_resources),
        ])
        .create()?;
        Ok(Self {
            name: self.name.clone(),
            domain: self.domain.clone(),
        })
    }
}

pub struct Structure {
    pub files: Vec<ProjectFile>,
}

impl From<Vec<ProjectFile>> for Structure {
    fn from(files: Vec<ProjectFile>) -> Self {
        Self { files }
    }
}

impl Structure {
    pub fn create(&self) -> Result<(), io::Error> {
        for file in &self.files {
            fs::create_dir_all(&file.path)?;
            if file.name == "" {
                continue;
            }
            File::create_new(format!("{}/{}", file.path, file.name))?;
            fs::write(
                format!("{}/{}", file.path, file.name),
                file.template.content.as_bytes(),
            )?
        }
        Ok(())
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
