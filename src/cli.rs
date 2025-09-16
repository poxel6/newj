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
            .about("Setup your java project structure")
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

        let name = matches
            .get_one::<String>("name")
            .unwrap_or(
                &Text::new("Enter project name (default: app):")
                    .prompt()
                    .unwrap_or_else(|_| String::from("app")),
            )
            .to_string();

        let domain = matches
            .get_one::<String>("domain")
            .unwrap_or(
                &Text::new("Enter package name (default: org.example):")
                    .prompt()
                    .unwrap_or_else(|_| String::from("org.example")),
            )
            .to_string();

        Self {
            matches,
            name,
            domain,
        }
    }
}
