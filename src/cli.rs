use clap::Parser;
use inquire::Text;

use crate::languages::Languages;

#[derive(Debug, Parser)]
#[command(version, about = "Setup your project structure.", long_about = None, bin_name = "newj")]
pub struct Cli {
    #[arg(short, long)]
    /// Name to your beautiful project.
    pub name: Option<String>,

    /// Domain name of your project.
    #[arg(short, long)]
    pub domain: Option<String>,

    /// Preset for your project. Currently has: simple, spring, mc-plugin.
    #[arg(short, long, default_value_t = String::from("simple"))]
    pub preset: String,

    /// Language that you want to scaffold a project for.
    #[arg(value_enum, short, long, default_value_t = Languages::Java)]
    pub language: Languages
}

impl Cli {
    pub fn from_args() -> Self {
        let cli = Cli::parse();

        let name = cli.name;
        let domain = cli.domain;
        let preset = cli.preset;
        let language = cli.language;

        Self {
            name,
            domain,
            preset,
            language,
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
