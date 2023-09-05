use persistence_std_derive::CosmwasmExt;
use pbjson;
/// Module is the config object of the feegrant module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.feegrant.module.v1.Module")]
pub struct Module {}
include!("serde.rs");
