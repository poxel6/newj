use clap::{Arg, ArgAction, ArgMatches, Command};

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
                    .default_value("exmpleapp")
                    .action(ArgAction::Set)
                    .help("Name of the project"),
            )
            .arg(
                Arg::new("domain")
                    .long("domain")
                    .short('d')
                    .default_value("com.example")
                    .action(ArgAction::Set)
                    .help("Domain/Id of the project"),
            );

        let matches = cmd.get_matches();
        let name = matches.get_one::<String>("name").unwrap().to_string();
        let domain = matches.get_one::<String>("domain").unwrap().to_string();

        Self {
            matches,
            name,
            domain,
        }
    }
}
