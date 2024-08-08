/// Traits and types for Runtime
use crate::primitives::core::FinternetUID;
use serde::de::Deserializer;

/// Trait for loading the data into memory
// // __NOTE__: Firm up the return type, impl Deserializer is lazy
pub trait Loader {
    fn load(&self, id: FinternetUID) -> Result<impl Deserializer, String>;
}
