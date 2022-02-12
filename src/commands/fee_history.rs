use crate::commands::GlobalArgs;
use crate::error::{Error, Result};
use crate::eth::client;
use crate::eth::default::RpcResult;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(long)]
    oldest_block: u16,

    #[clap(long)]
    newest_block: String,

    #[clap(long, use_delimiter = true)]
    reward_percentiles: Option<Vec<f32>>,
}

pub async fn exec(args: Args, global_args: GlobalArgs) -> Result<()> {
    let client = client::get_client(global_args.endpoint).await?;
    let reward_percentiles = args.reward_percentiles.unwrap_or([].to_vec());

    if args.oldest_block == 0 || args.oldest_block > 1024 {
        return Err(Error::Args("invalid oldest block number".to_string()));
    }

    let result = if args.newest_block.parse::<u128>().is_err() {
        // call as str
        client
            .get_fee_history_by_str(args.oldest_block, args.newest_block, reward_percentiles)
            .await?
    } else {
        // call as int
        let newest_block = args.newest_block.parse::<u128>().unwrap();
        client
            .get_fee_history_by_int(args.oldest_block, newest_block, reward_percentiles)
            .await?
    };

    if global_args.is_json {
        println!("{}", RpcResult::from(result).to_json())
    } else {
        println!("{}", result);
    }

    Ok(())
}
