/// Traits and types for Runtime
use crate::primitives::core::FinternetUID;
use serde::de::Deserializer;

/// Trait for loading the data into memory
pub trait Loader {
    fn load(&self, id: FinternetUID) -> Result<impl Deserializer, String>;
}

/// Trait for saving the data from memory
pub trait Saver<'a> {
    fn save(&self, id: FinternetUID, data: impl Deserializer<'a>) -> Result<(), String>;
}

/// Trait to execute the transaction
pub trait Executor {
    fn execute(&self, transaction_bytes: Vec<u8>) -> Result<(), String>;
}
