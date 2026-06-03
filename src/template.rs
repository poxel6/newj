use std::{
    fs::{self, File},
    io,
    path::{Path, PathBuf},
};

use crate::project::Project;

pub struct Template {
    project: Project,
}

impl Template {
    pub fn new(project: &Project) -> Self {
        Self {
            project: project.clone(),
        }
    }

    pub fn default(project: &Project) -> Result<(), io::Error> {
        let template = Template::new(project);
        let structure = template.get_structure();
        template.craete_dirs(&structure)?;
        template.create_files(&structure)?;
        Ok(())
    }
}

// private
impl Template {
    fn craete_dirs(&self, dirs: &Vec<PathBuf>) -> Result<(), io::Error> {
        for dir in dirs {
            let dir = dir.parent().unwrap();
            if dir.is_dir() {
                fs::create_dir_all(self.replace_placeholders(dir))?;
            }
        }
        Ok(())
    }

    fn create_files(&self, files: &Vec<PathBuf>) -> Result<(), io::Error> {
        for file in files {
            File::create_new(self.replace_placeholders(file))?;
        }
        Ok(())
    }

    fn replace_placeholders(&self, path: &Path) -> String {
        let project = &self.project;
        let domain = project.domain.split_once(".").unwrap();
        let template = format!(
            "template/{}/{}",
            project.language.as_str(),
            project.preset.as_str()
        );

        let path = path
            .to_str()
            .unwrap()
            .replace(&template, &project.name)
            .replace("project-name", &project.name)
            .replace("org", domain.0)
            .replace("example", domain.1);

        path
    }

    fn get_structure(&self) -> Vec<PathBuf> {
        let project = &self.project;
        let paths = fs::read_dir(format!(
            "template/{}/{}/",
            project.language.as_str(),
            project.preset
        ))
        .unwrap();
        paths
            .into_iter()
            .map(|path| get_childs(path.unwrap().path()))
            .flatten()
            .collect()
    }
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
