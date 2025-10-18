use crate::{project::Project, strings::Strings};
use std::{collections::HashMap, fs};

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
            format!(
                "{}.{}.App", /* Main file name */
                &self.0.domain, &self.0.name
            )
            .as_str(),
        );
        let text = text.replace(
            "{{PACKAGE}}",
            format!("{}.{}", &self.0.domain, &self.0.name).as_str(),
        );
        let text = text.replace("{{FILE_NAME}}", &self.0.name.to_capitilize().to_string());
        let mut preset: HashMap<String, Vec<&str>> = HashMap::new();
        preset.insert(
            "mc-plugin".to_string(),
            vec![
                "compileOnly(\"io.papermc.paper:paper-api:1.21.4-R0.1-SNAPSHOT\")",
                "compileOnly(\"org.projectlombok:lombok:1.18.42\")",
                "annotationProcessor(\"org.projectlombok:lombok:1.18.42\")",
                "testCompileOnly(\"org.projectlombok:lombok:1.18.42\")",
                "testAnnotationProcessor(\"org.projectlombok:lombok:1.18.42\")",
                "testImplementation(\"org.junit.jupiter:junit-jupiter-api:5.8.1\")",
                "testRuntimeOnly(\"org.junit.jupiter:junit-jupiter-engine:5.8.1\")",
            ],
        );
        preset.insert(
            "spring".to_string(),
            vec![
                "compileOnly(\"org.projectlombok:lombok\")",
                "annotationProcessor(\"org.projectlombok:lombok\")",
                "implementation(\"org.springframework.boot:spring-boot-starter-data-mongodb\")",
                "implementation(\"org.springframework.boot:spring-boot-starter-thymeleaf\")",
                "implementation(\"org.springframework.boot:spring-boot-starter-web\")",
                "testImplementation(\"org.springframework.boot:spring-boot-starter-test\")",
                "testRuntimeOnly(\"org.junit.platform:junit-platform-launcher\")",
                "developmentOnly(\"org.springframework.boot:spring-boot-devtools\")",
            ],
        );
        preset.insert(
            "simple".to_string(),
            vec![
                "implementation(\"com.google.guava:guava:11.0.2\")",
                "compileOnly(\"org.projectlombok:lombok:1.18.42\")",
                "annotationProcessor(\"org.projectlombok:lombok:1.18.42\")",
                "testCompileOnly(\"org.projectlombok:lombok:1.18.42\")",
                "testAnnotationProcessor(\"org.projectlombok:lombok:1.18.42\")",
                "testImplementation(\"org.junit.jupiter:junit-jupiter-api:5.8.1\")",
                "testRuntimeOnly(\"org.junit.jupiter:junit-jupiter-engine:5.8.1\")",
            ],
        );
        let preset_key = preset
            .keys()
            .filter(|key| **key == self.0.preset.as_str())
            .last()
            .expect("to not be empty");
        let preset_value = preset.get(preset_key).expect("to not throw");
        let mut val = String::new();
        preset_value
            .iter()
            .for_each(|dep| val.push_str(&format!("{}\n\t", dep)));
        let text = text.replace("{{DEPENDENCIES}}", &val);
        text
    }
}

impl<'a> From<&'a Project> for Placeholder<'a> {
    fn from(project: &'a Project) -> Self {
        Self(project)
    }
}
