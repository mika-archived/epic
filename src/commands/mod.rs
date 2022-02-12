use clap::Parser;

#[derive(Parser)]
pub enum SubCommand {
    #[clap()]
    BlockNumber(blocknumber::Args),
}

pub mod blocknumber;
