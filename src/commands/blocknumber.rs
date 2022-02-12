use crate::error::{Error, Result};
use crate::eth::client;
use crate::eth::default::RpcResult;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(long)]
    endpoint: Option<String>,

    #[clap(long)]
    json: bool,
}

pub async fn exec(args: Args) -> Result<()> {
    let client = client::get_client(args.endpoint).await?;
    let result = client.get_block_number().await?;

    let output = if args.json {
        RpcResult::from(result).to_json()
    } else {
        let block_number =
            u128::from_str_radix(&result[2..], 16).map_err(|err| Error::ParseIntError(err))?;
        format!("{}", block_number)
    };

    println!("{}", output);

    Ok(())
}
