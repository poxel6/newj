use clap::{Arg, ArgAction, ArgMatches, Command};
use inquire::Text;

#[derive(Debug)]
pub struct Cli {
    pub matches: ArgMatches,
    pub name: String,
    pub domain: String,
}

impl Cli {
    pub fn from_args() -> Self {
        let cmd = Command::new("newj")
            .bin_name("newj")
            .about("Setup your project structure")
            .arg(
                Arg::new("name")
                    .long("name")
                    .short('n')
                    .action(ArgAction::Set)
                    .required(false)
                    .help("Name of the project"),
            )
            .arg(
                Arg::new("domain")
                    .long("domain")
                    .short('d')
                    .action(ArgAction::Set)
                    .required(false)
                    .help("Domain/Id of the project"),
            );

        let matches = cmd.get_matches();

        let name = match matches.get_one::<String>("name") {
            Some(name) => name.to_string(),
            None => prompt("Enter project name", "app"),
        };
        let domain = match matches.get_one::<String>("domain") {
            Some(domain) => domain.to_string(),
            None => prompt("Enter package name", "org.example"),
        };

        Self {
            matches: matches.clone(),
            name,
            domain,
        }
    }
}

fn prompt(prompt: &str, default: &str) -> String {
    Text::new(&format!("{} (default: {}):", &prompt, &default))
        .prompt()
        .unwrap_or_else(|_| default.to_string())
}
