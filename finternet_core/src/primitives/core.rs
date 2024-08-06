/// Core Types
use serde::{Deserialize, Serialize};
use ssi_caips::caip2::ChainId;
use std::ops::Deref;
use std::str::FromStr;

/// Fundamental Identifier for anything on the Finternet
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FinternetUID([u8; 32]);

/// Public Key or Signing Key for the User
// __NOTE__
// This should be generalized to support multiple key types or eliptic curves
// The current implementation is for the sake of simplicity and to get the ball rolling
// It doesn't  explicitly say anything about the key type
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PublicKey([u8; 32]);

/// Type wrapper for ChainId
#[derive(Debug, Clone)]
pub struct FinternetChainID(ChainId);

// impl all the necessary traits for FinternetChainID
impl Deref for FinternetChainID {
    type Target = ChainId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Serialize for FinternetChainID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serde_json::Value::String(self.0.to_string()).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for FinternetChainID {
    fn deserialize<D>(deserializer: D) -> Result<FinternetChainID, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let chain_id_str = String::deserialize(deserializer)?;
        let chain_id = ChainId::from_str(&chain_id_str).map_err(serde::de::Error::custom)?;
        Ok(FinternetChainID(chain_id))
    }
}
