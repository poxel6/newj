use crate::{project::Project, strings::Strings};
use std::fs;

#[derive(Default, Debug)]
pub struct Template {
    pub name: String,
    pub content: String,
    pub path: String,
}

impl Template {
    pub fn new(name: &str, project: &Project) -> Self {
        let path = format!("template/{}", name);
        let holder = Placeholder::from(project);
        let content = fs::read_to_string(&path).unwrap_or_else(|_| "".to_string());
        let content = holder.replace(content);
        Self {
            name: name.to_string(),
            content,
            path,
        }
    }
}

#[derive(Debug)]
pub struct Placeholder<'a>(&'a Project);

impl<'a> Placeholder<'a> {
    pub fn replace(&self, text: String) -> String {
        let text = text.replace(
            "{{MAIN_CLASS}}",
            format!("{}.{}", self.0.domain, self.0.name).as_str(),
        );
        let text = text.replace("{{FILE_NAME}}", &self.0.name.to_capitilize().to_string());
        text
    }
}

impl<'a> From<&'a Project> for Placeholder<'a> {
    fn from(project: &'a Project) -> Self {
        Self(project)
    }
}
