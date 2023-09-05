
use persistence_std_derive::CosmwasmExt;
use pbjson;
/// ConfigRequest defines the request structure for the Config gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.base.node.v1beta1.ConfigRequest")]
pub struct ConfigRequest {}
/// ConfigResponse defines the response structure for the Config gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/cosmos.base.node.v1beta1.ConfigResponse")]
pub struct ConfigResponse {
    #[prost(string, tag = "1")]
    pub minimum_gas_price: ::prost::alloc::string::String,
}
include!("serde.rs");