use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};
use strum_macros::EnumString;
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Node {
    pub name: String,
    pub path: String,
    pub chain: Chain,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            name: String::from("selendra"),
            path: String::from("/var/lib/selendra"),
            chain: Chain::default(),
        }
    }
}

impl Node {
    pub fn new(name: &str, path: &str, chain: &Chain) -> Self {
        Self {
            name: name.to_string().to_case(Case::Snake),
            path: path.to_string().to_case(Case::Snake),
            chain: chain.to_owned(),
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Chain {
    Mainnet,
    Testnet,
    Custom(String),
}

impl Default for Chain {
    fn default() -> Self {
        Self::Testnet
    }
}
