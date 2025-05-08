use clap::Subcommand;

#[derive(Subcommand)]
pub enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}
