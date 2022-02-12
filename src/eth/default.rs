use serde::Serialize;

pub static LOCALHOST_RPC: &str = "http://localhost:8545";

#[derive(Serialize, Debug, Clone)]
pub struct RpcResult<T: Serialize> {
    result: T,
}

impl<T> RpcResult<T>
where
    T: Serialize,
{
    pub fn from(result: T) -> Self {
        RpcResult { result }
    }

    pub fn to_json(&self) -> String {
        let buffer = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"  ");
        let mut serializer = serde_json::Serializer::with_formatter(buffer, formatter);

        self.serialize(&mut serializer).unwrap();

        return String::from_utf8(serializer.into_inner()).unwrap();
    }
}
