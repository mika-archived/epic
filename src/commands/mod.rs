use clap::Parser;

#[derive(Parser)]
pub enum SubCommand {
    #[clap()]
    Accounts(accounts::Args),

    #[clap()]
    BlockNumber(block_number::Args),

    #[clap()]
    ChainId(chain_id::Args),

    #[clap()]
    FeeHistory(fee_history::Args),
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
pub mod block_number;
pub mod chain_id;
pub mod fee_history;
