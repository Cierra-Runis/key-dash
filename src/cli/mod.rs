use clap::Parser;
use commands::Commands;
use std::path::PathBuf;
mod commands;

/// TODO: Add i18n support <https://github.com/clap-rs/clap/issues/380>
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    pub name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long)]
    pub debug: Option<bool>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

impl Cli {
    pub fn run() {
        // Receive command line arguments
        let cli = Cli::parse();

        // You can check the value provided by positional arguments, or option arguments
        if let Some(name) = cli.name.as_deref() {
            println!("Value for name: {name}");
        }

        if let Some(config_path) = cli.config.as_deref() {
            println!("Value for config: {}", config_path.display());
        }

        if let Some(debug) = cli.debug {
            println!("Debugging is: {}", debug);
        }

        // You can check for the existence of subcommands, and if found use their
        // matches just as you would the top level cmd
        match &cli.command {
            Some(Commands::Test { list }) => {
                if *list {
                    println!("Printing testing lists...");
                } else {
                    println!("Not printing testing lists...");
                }
            }
            None => {}
        }
    }
}

#[test]
fn test_cli() {
    let args = vec!["--name", "test_name", "--config", "test_config"];
    let cli = Cli::parse_from(args);
    assert_eq!(cli.name, Some("test_name".to_string()));
    assert_eq!(cli.config, Some(PathBuf::from("test_config")));
}
