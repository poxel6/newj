use newj::{cli::Cli, project::Project};

fn main() {
    let cli = Cli::from_args();

    let project = Project::from(cli);
    if let Err(err) = project.init() {
        eprintln!("{}", err.to_string());
    }
}
