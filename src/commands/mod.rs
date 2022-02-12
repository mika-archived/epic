use clap::Parser;

#[derive(Parser)]
pub enum SubCommand {
    #[clap()]
    Accounts(accounts::Args),

    #[clap()]
    BlockNumber(blocknumber::Args),

    #[clap()]
    ChainId(chainid::Args),
}

pub struct GlobalArgs {
    endpoint: Option<String>,
    is_json: bool,
}

impl GlobalArgs {
    pub fn new(endpoint: Option<String>, is_json: bool) -> Self {
        Self { endpoint, is_json }
    }
}

pub mod accounts;
pub mod blocknumber;
pub mod chainid;
