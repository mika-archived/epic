use crate::error::{Error, Result};
use jsonrpc_core_client::futures::Future;
use jsonrpc_core_client::{RpcChannel, RpcResult, TypedClient};

#[derive(Clone)]
pub struct EthereumClient(TypedClient);

impl From<RpcChannel> for EthereumClient {
    fn from(channel: RpcChannel) -> Self {
        EthereumClient(channel.into())
    }
}

impl EthereumClient {
    pub fn get_accounts(&self) -> impl Future<Output = RpcResult<Vec<String>>> {
        self.0.call_method("eth_accounts", "Vec<String>", ())
    }

    pub fn get_block_number(&self) -> impl Future<Output = RpcResult<String>> {
        self.0.call_method("eth_blockNumber", "String", ())
    }
}

pub async fn get_client(endpoint: Option<String>) -> Result<EthereumClient> {
    let endpoint = endpoint.unwrap_or_else(|| super::default::LOCALHOST_RPC.to_owned());
    let client =
        jsonrpc_core_client::transports::http::connect::<EthereumClient>(endpoint.as_str()).await;
    let client = match client {
        Ok(client) => client,
        Err(err) => return Err(Error::RpcError(err)),
    };

    return Ok(client);
}
