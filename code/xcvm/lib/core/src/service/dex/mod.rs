use crate::{prelude::*, NetworkId};

pub mod osmosis_std;

pub type ExchangeId = crate::shared::Displayed<u128>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ExchangeType {
	OsmosisCrossChainSwap(String),

	PabloPrecompile(String),
}

/// allows to execute Exchange instruction
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
pub struct ExchangeItem {
	pub network_id: NetworkId,
	pub exchange: ExchangeType,
}
