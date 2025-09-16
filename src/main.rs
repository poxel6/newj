use newj::{cli::Cli, project::Project};

fn main() {
    let cli = Cli::from_args();
    if let Err(err) = Project::new(&cli.name, &cli.domain) {
        eprintln!("{err}");
    }
}
