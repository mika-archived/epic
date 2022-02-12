use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Args(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    RpcError(#[from] jsonrpc_core_client::RpcError),

    #[error(transparent)]
    SerdeJsonSerializeError(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
