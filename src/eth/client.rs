use crate::error::{Error, Result};
use crate::eth::entity;
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

    pub fn get_chain_id(&self) -> impl Future<Output = RpcResult<String>> {
        self.0.call_method("eth_chainId", "String", ())
    }

    pub fn get_fee_history_by_int<'a>(
        &self,
        block_count: u16,
        newest_block: u128,
        reward_percentiles: Vec<f32>,
    ) -> impl Future<Output = RpcResult<entity::FeeHistory>> + 'a {
        self.0.call_method(
            "eth_feeHistory",
            "FeeHistory",
            (block_count, newest_block, reward_percentiles),
        )
    }

    pub fn get_fee_history_by_str<'a>(
        &self,
        block_count: u16,
        newest_block: String,
        reward_percentiles: Vec<f32>,
    ) -> impl Future<Output = RpcResult<entity::FeeHistory>> + 'a {
        self.0.call_method(
            "eth_feeHistory",
            "FeeHistory",
            (block_count, newest_block, reward_percentiles),
        )
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
