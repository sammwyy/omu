use clap::Parser;
use commands::Cli;

mod commands;

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    cli.execute()
}
