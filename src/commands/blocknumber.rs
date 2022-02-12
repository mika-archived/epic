use crate::commands::GlobalArgs;
use crate::error::{Error, Result};
use crate::eth::client;
use crate::eth::default::RpcResult;

use clap::Parser;

#[derive(Parser)]
pub struct Args {}

pub async fn exec(_args: Args, global_args: GlobalArgs) -> Result<()> {
    let client = client::get_client(global_args.endpoint).await?;
    let result = client.get_block_number().await?;

    let output = if global_args.is_json {
        RpcResult::from(result).to_json()
    } else {
        let block_number =
            u128::from_str_radix(&result[2..], 16).map_err(|err| Error::ParseIntError(err))?;
        format!("{}", block_number)
    };

    println!("{}", output);

    Ok(())
}
