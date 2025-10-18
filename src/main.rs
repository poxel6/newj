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

    println!(
        "\t\x1b[1;32m{}:\x1b[0m {}",
        "FINISHED", "Project initialization."
    );
    println!("\t\x1b[1;32m{}:\x1b[0m       {}", "CD", &project.name);
    println!("\t\x1b[1;32m{}:\x1b[0m  gralde run", "EXECUTE");
}
