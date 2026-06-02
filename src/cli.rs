use clap::Parser;
use inquire::Text;

#[derive(Debug, Parser)]
#[command(version, about = "Setup your project structure.", long_about = None, bin_name = "newj")]
pub struct Cli {
    #[arg(short, long)]
    /// Name to your beautiful project.
    pub name: Option<String>,

    /// Domain name of your project.
    #[arg(short, long)]
    pub domain: Option<String>,

    /// Preset for your project. currently has: simple, spring, mc-plugin.
    #[arg(short, long, default_value_t = String::from("simple"))]
    pub preset: String,
}

impl Cli {
    pub fn from_args() -> Self {
        let cli = Cli::parse();

        let name = cli.name;
        let domain = cli.domain;
        let preset = cli.preset;

        Self {
            name,
            domain,
            preset,
        }
    }
}

pub fn prompt(prompt: &str, default: &str) -> String {
    Text::new(&prompt)
        .with_default(default)
        .with_placeholder(default)
        .prompt()
        .unwrap_or_else(|_| default.to_string())
}
