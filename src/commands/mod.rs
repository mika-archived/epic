use clap::Parser;

#[derive(Parser)]
pub enum SubCommand {
    #[clap()]
    Accounts(accounts::Args),

    #[clap()]
    BlockNumber(blocknumber::Args),
}

pub mod accounts;
pub mod blocknumber;
