use crate::strings::Strings;
use clap::{Arg, ArgAction, ArgMatches, Command};
use inquire::Text;

#[derive(Debug)]
pub struct Cli {
    pub matches: ArgMatches,
    pub name: String,
    pub domain: String,
    pub preset: String,
}

impl Cli {
    pub fn from_args() -> Self {
        let cmd = Command::new("newj")
            .bin_name("newj")
            .about("Setup your project structure")
            .arg(arg_from("name"))
            .arg(arg_from("domain"))
            .arg(arg_from("preset"));

        let matches = cmd.get_matches();

        let name = match matches.get_one::<String>("name") {
            Some(name) => name.to_string(),
            None => prompt("Enter project name", "app"),
        };

        let domain = match matches.get_one::<String>("domain") {
            Some(domain) => domain.to_string(),
            None => prompt("Enter package name", "org.example"),
        };

        let preset = matches
            .get_one::<String>("preset")
            .unwrap_or(&String::from("simple"))
            .to_string();

        let matches = matches.clone();

        Self {
            matches,
            name,
            domain,
            preset,
        }
    }
}

fn prompt(prompt: &str, default: &str) -> String {
    Text::new(&prompt)
        .with_default(default)
        .with_placeholder(default)
        .prompt()
        .unwrap_or_else(|_| default.to_string())
}

fn arg_from(str: &str) -> Arg {
    Arg::new(str.to_string())
        .long(str.to_string())
        .short(str.chars().nth(0).expect("Expects string to not be empty"))
        .action(ArgAction::Set)
        .required(false)
        .help(format!(
            "{} of the project",
            str.to_string().to_capitilize()
        ))
}
