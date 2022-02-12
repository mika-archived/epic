use clap::Parser;

mod commands;
mod error;
mod eth;

#[derive(Parser)]
#[clap(author, about, version)]
struct Args {
    #[clap(subcommand)]
    subcommand: commands::SubCommand,

    // global
    #[clap(long, global = true)]
    endpoint: Option<String>,

    #[clap(long = "json", global = true)]
    is_json: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Args = Args::parse();
    let globals = commands::GlobalArgs::new(args.endpoint, args.is_json);

    let result = match args.subcommand {
        commands::SubCommand::Accounts(a) => commands::accounts::exec(a, globals).await,
        commands::SubCommand::BlockNumber(a) => commands::blocknumber::exec(a, globals).await,
        commands::SubCommand::ChainId(a) => commands::chainid::exec(a, globals).await,
    };

    match result {
        Ok(()) => Ok(()),
        Err(e) => Err(Box::from(e)),
    }
}
