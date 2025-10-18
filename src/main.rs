use std::io;

use newj::{cli::Cli, project::Project};

fn main() {
    let cli = Cli::from_args();

    let project = Project::from(cli);
    if let Err(err) = project.init() {
        match err.kind() {
            io::ErrorKind::IsADirectory => (),
            _ => eprintln!("{}", err.to_string()),
        }
    }
}
