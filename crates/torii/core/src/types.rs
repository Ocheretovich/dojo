use core::fmt;
use std::{path::PathBuf, str::FromStr};

use chrono::{DateTime, Utc};
use dojo_types::schema::Ty;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use starknet::core::types::Felt;

#[derive(Debug, Serialize, Deserialize)]
pub struct SQLFelt(pub Felt);

impl From<SQLFelt> for Felt {
    fn from(field_element: SQLFelt) -> Self {
        field_element.0
    }
}

impl TryFrom<String> for SQLFelt {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(SQLFelt(Felt::from_hex(&value)?))
    }
}

impl fmt::LowerHex for SQLFelt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(FromRow, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Entity {
    pub id: String,
    pub keys: String,
    pub event_id: String,
    pub executed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // if updated_model is None, then the entity has been deleted
    #[sqlx(skip)]
    pub updated_model: Option<Ty>,
}

#[derive(FromRow, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EventMessage {
    pub id: String,
    pub keys: String,
    pub event_id: String,
    pub executed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    // this should never be None. as a EventMessage cannot be deleted
    #[sqlx(skip)]
    pub updated_model: Option<Ty>,
}

#[derive(FromRow, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Model {
    pub id: String,
    pub namespace: String,
    pub name: String,
    pub class_hash: String,
    pub contract_address: String,
    pub transaction_hash: String,
    pub executed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(FromRow, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    pub keys: String,
    pub data: String,
    pub transaction_hash: String,
    pub executed_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Default, Deserialize, Debug, Clone)]
pub struct ToriiConfig {
    /// ERC contract addresses to index
    pub erc_contracts: Vec<ErcContract>,
}

impl ToriiConfig {
    pub fn load_from_path(path: &PathBuf) -> Result<Self, anyhow::Error> {
        let config = std::fs::read_to_string(path)?;
        let config: Self = toml::from_str(&config)?;
        Ok(config)
    }
}

#[derive(Default, Deserialize, Debug, Clone)]
pub struct ErcContract {
    pub contract_address: Felt,
    pub start_block: u64,
    pub r#type: ErcType,
}

#[derive(Default, Deserialize, Debug, Clone)]
pub enum ErcType {
    #[default]
    ERC20,
    ERC721,
}

impl FromStr for ErcType {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input.to_lowercase().as_str() {
            "erc20" | "Er20" | "ERC20" => Ok(ErcType::ERC20),
            "erc721" | "Er721" | "ERC721" => Ok(ErcType::ERC721),
            _ => Err(anyhow::anyhow!("Invalid ERC type: {}", input)),
        }
    }
}

impl ToString for ErcType {
    fn to_string(&self) -> String {
        match self {
            ErcType::ERC20 => "ERC20",
            ErcType::ERC721 => "ERC721",
        }
        .to_string()
    }
}
