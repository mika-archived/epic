use crate::error::Result;
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
    let result = client.get_accounts().await?;

    if args.json {
        println!("{}", RpcResult::from(result).to_json())
    } else {
        for account in result.into_iter() {
            println!("{}", account);
        }
    };

    Ok(())
}
