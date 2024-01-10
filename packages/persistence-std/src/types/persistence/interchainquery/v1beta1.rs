use persistence_std_derive::CosmwasmExt;
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
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
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
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
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/persistence.interchainquery.v1beta1.GenesisState")]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub queries: ::prost::alloc::vec::Vec<Query>,
}
/// MsgSubmitQueryResponse represents a message type to fulfil a query request.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/persistence.interchainquery.v1beta1.MsgSubmitQueryResponse")]
pub struct MsgSubmitQueryResponse {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "queryID")]
    pub query_id: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub result: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "4")]
    pub proof_ops: ::core::option::Option<super::super::super::tendermint::crypto::ProofOps>,
    #[prost(int64, tag = "5")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
    #[prost(string, tag = "6")]
    pub from_address: ::prost::alloc::string::String,
}
/// MsgSubmitQueryResponseResponse defines the MsgSubmitQueryResponse response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/persistence.interchainquery.v1beta1.MsgSubmitQueryResponseResponse")]
pub struct MsgSubmitQueryResponseResponse {}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/persistence.interchainquery.v1beta1.QueryRequestsRequest")]
pub struct QueryRequestsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
    #[prost(string, tag = "2")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/persistence.interchainquery.v1beta1.QueryRequestsResponse")]
pub struct QueryRequestsResponse {
    /// params defines the parameters of the module.
    #[prost(message, repeated, tag = "1")]
    pub queries: ::prost::alloc::vec::Vec<Query>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// GetTxResponse is the response type for the Service.GetTx method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(
    Clone,
    PartialEq,
    Eq,
    ::prost::Message,
    ::serde::Serialize,
    ::serde::Deserialize,
    ::schemars::JsonSchema,
    CosmwasmExt,
)]
#[proto_message(type_url = "/persistence.interchainquery.v1beta1.GetTxWithProofResponse")]
pub struct GetTxWithProofResponse {
    /// tx is the queried transaction.
    #[prost(message, optional, tag = "1")]
    pub tx: ::core::option::Option<super::super::super::cosmos::tx::v1beta1::Tx>,
    /// tx_response is the queried TxResponses.
    #[prost(message, optional, tag = "2")]
    pub tx_response:
        ::core::option::Option<super::super::super::cosmos::base::abci::v1beta1::TxResponse>,
    /// proof is the tmproto.TxProof for the queried tx
    #[prost(message, optional, tag = "3")]
    pub proof: ::core::option::Option<super::super::super::tendermint::types::TxProof>,
    /// ibc-go header to validate txs
    #[prost(message, optional, tag = "4")]
    pub header:
        ::core::option::Option<super::super::super::ibc::lightclients::tendermint::v1::Header>,
}
