use clap::Parser;

mod commands;
mod error;
mod eth;

#[derive(Parser)]
#[clap(author, about, version)]
struct Args {
    #[clap(subcommand)]
    subcommand: commands::SubCommand,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();

    let result = match args.subcommand {
        commands::SubCommand::BlockNumber(a) => commands::blocknumber::exec(a).await,
    };

    match result {
        Ok(()) => Ok(()),
        Err(e) => Err(Box::from(e)),
    }
}
