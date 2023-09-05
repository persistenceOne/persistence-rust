
use persistence_std_derive::CosmwasmExt;
use pbjson;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/persistence.interchainquery.v1beta1.Query")]
pub struct Query {
    #[prost(string, tag = "1")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub query_type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "5")]
    pub request: ::prost::alloc::vec::Vec<u8>,
    /// change these to uint64 in v0.5.0
    #[prost(string, tag = "6")]
    pub period: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub last_height: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    #[serde(alias = "callbackID")]
    pub callback_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "9")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub ttl: u64,
    #[prost(string, tag = "10")]
    pub last_emission: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/persistence.interchainquery.v1beta1.DataPoint")]
pub struct DataPoint {
    #[prost(string, tag = "1")]
    #[serde(alias = "ID")]
    pub id: ::prost::alloc::string::String,
    /// change these to uint64 in v0.5.0
    #[prost(string, tag = "2")]
    pub remote_height: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub local_height: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub value: ::prost::alloc::vec::Vec<u8>,
}
/// GenesisState defines the epochs module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/persistence.interchainquery.v1beta1.GenesisState")]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub queries: ::prost::alloc::vec::Vec<Query>,
}
/// NOTE: The following type is not implemented due to current limitations of code generator
/// which currently has issue with tendermint_proto.
/// This will be fixed in the upcoming release.
#[allow(dead_code)]
struct MsgSubmitQueryResponse {}
/// MsgSubmitQueryResponseResponse defines the MsgSubmitQueryResponse response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/persistence.interchainquery.v1beta1.MsgSubmitQueryResponseResponse"
)]
pub struct MsgSubmitQueryResponseResponse {}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/persistence.interchainquery.v1beta1.QueryRequestsRequest")]
pub struct QueryRequestsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageRequest,
    >,
    #[prost(string, tag = "2")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/persistence.interchainquery.v1beta1.QueryRequestsResponse")]
pub struct QueryRequestsResponse {
    /// params defines the parameters of the module.
    #[prost(message, repeated, tag = "1")]
    pub queries: ::prost::alloc::vec::Vec<Query>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<
        super::super::super::cosmos::base::query::v1beta1::PageResponse,
    >,
}
/// NOTE: The following type is not implemented due to current limitations of code generator
/// which currently has issue with tendermint_proto.
/// This will be fixed in the upcoming release.
#[allow(dead_code)]
struct GetTxWithProofResponse {}
include!("serde.rs");
