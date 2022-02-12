use crate::commands::GlobalArgs;
use crate::error::Result;
use crate::eth::client;
use crate::eth::default::RpcResult;

use clap::Parser;

#[derive(Parser)]
pub struct Args {}

pub async fn exec(_args: Args, global_args: GlobalArgs) -> Result<()> {
    let client = client::get_client(global_args.endpoint).await?;
    let result = client.get_accounts().await?;

    if global_args.is_json {
        println!("{}", RpcResult::from(result).to_json())
    } else {
        for account in result.into_iter() {
            println!("{}", account);
        }
    };

    Ok(())
}
