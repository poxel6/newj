use newj::{cli::Cli, project::Project, template::Template};

fn main() {
    let cli = Cli::from_args();

    let project = Project {
        name: cli.name,
        domain: cli.domain,
    };

    if let Err(err) = project.new() {
        eprintln!("{}",err.to_string());
    }
}
