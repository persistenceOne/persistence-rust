use persistence_std_derive::CosmwasmExt;
/// msg blob for instantiate contract.
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.InstantiateLiquidStakeRateContract")]
pub struct InstantiateLiquidStakeRateContract {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
}
/// wrapper for liquidstakerate as wasm msg should be marshalled as encodedMsg = { wasmMsg: { wasm MsgDetails } }
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.ExecuteLiquidStakeRate")]
pub struct ExecuteLiquidStakeRate {
    #[prost(message, optional, tag = "1")]
    pub liquid_stake_rate: ::core::option::Option<LiquidStakeRate>,
}
/// msg blob for execute contract.
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.LiquidStakeRate")]
pub struct LiquidStakeRate {
    #[prost(string, tag = "1")]
    pub default_bond_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub stk_denom: ::prost::alloc::string::String,
    /// cvalue = default_bond_denom_price/stk_denom_price
    /// cvalue = stk_denom_supply/default_bond_denom_supply
    #[prost(string, tag = "3")]
    pub c_value: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub controller_chain_time: i64,
}
/// Params defines the parameters for the module.
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.Params")]
pub struct Params {
    #[prost(string, tag = "1")]
    pub admin: ::prost::alloc::string::String,
}
/// HostChain defines the ratesync module's HostChain state.
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.HostChain")]
pub struct HostChain {
    /// unique id
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub i_d: u64,
    /// not really required, just easier readability
    #[prost(string, tag = "2")]
    pub chain_i_d: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub connection_i_d: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub i_c_a_account: ::core::option::Option<super::super::liquidstakeibc::v1beta1::IcaAccount>,
    #[prost(message, optional, tag = "5")]
    pub features: ::core::option::Option<Feature>,
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.Feature")]
pub struct Feature {
    /// triggers on hooks
    #[prost(message, optional, tag = "1")]
    pub liquid_stake_i_b_c: ::core::option::Option<LiquidStake>,
    /// triggers on hour epoch
    #[prost(message, optional, tag = "2")]
    pub liquid_stake: ::core::option::Option<LiquidStake>,
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.LiquidStake")]
pub struct LiquidStake {
    #[prost(enumeration = "FeatureType", tag = "1")]
    #[serde(
        serialize_with = "feature_type_serde::serialize",
        deserialize_with = "feature_type_serde::deserialize"
    )]
    pub feature_type: i32,
    /// needs to be uploaded before hand
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub code_i_d: u64,
    /// state of instantiation, do not support gov based instantiation. (need ICA to be at least admin)
    #[prost(enumeration = "InstantiationState", tag = "3")]
    #[serde(
        serialize_with = "instantiation_state_serde::serialize",
        deserialize_with = "instantiation_state_serde::deserialize"
    )]
    pub instantiation: i32,
    /// address of instantiated contract.
    #[prost(string, tag = "4")]
    pub contract_address: ::prost::alloc::string::String,
    /// allow * as default for all denoms in case of lsibc, or default bond denom in case of ls.
    #[prost(string, repeated, tag = "5")]
    pub denoms: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "6")]
    pub enabled: bool,
}
/// aim to keep this smaller than 256 MaxCharLen in ICA memo.
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.ICAMemo")]
pub struct IcaMemo {
    #[prost(enumeration = "FeatureType", tag = "1")]
    #[serde(
        serialize_with = "feature_type_serde::serialize",
        deserialize_with = "feature_type_serde::deserialize"
    )]
    pub feature_type: i32,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub host_chain_i_d: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::schemars::JsonSchema)]
pub enum InstantiationState {
    /// Not Initiated
    InstantiationNotInitiated = 0,
    /// Initiated
    InstantiationInitiated = 1,
    /// we should have an address
    InstantiationCompleted = 2,
}
pub mod instantiation_state_serde {
    use super::InstantiationState;
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn deserialize<'de, T, D>(deserializer: D) -> std::result::Result<T, D::Error>
    where
        T: From<InstantiationState>,
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let enum_value = InstantiationState::from_str_name(&s).unwrap();
        let int_value: T = enum_value.into();
        return Ok(int_value);
    }
    pub fn serialize<S>(value: &i32, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s: InstantiationState = InstantiationState::from_i32(*value).unwrap();
        serializer.serialize_str(s.as_str_name())
    }
}
impl InstantiationState {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InstantiationState::InstantiationNotInitiated => "INSTANTIATION_NOT_INITIATED",
            InstantiationState::InstantiationInitiated => "INSTANTIATION_INITIATED",
            InstantiationState::InstantiationCompleted => "INSTANTIATION_COMPLETED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INSTANTIATION_NOT_INITIATED" => Some(Self::InstantiationNotInitiated),
            "INSTANTIATION_INITIATED" => Some(Self::InstantiationInitiated),
            "INSTANTIATION_COMPLETED" => Some(Self::InstantiationCompleted),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::schemars::JsonSchema)]
pub enum FeatureType {
    LiquidStakeIbc = 0,
    LiquidStake = 1,
}
pub mod feature_type_serde {
    use super::FeatureType;
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn deserialize<'de, T, D>(deserializer: D) -> std::result::Result<T, D::Error>
    where
        T: From<FeatureType>,
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let enum_value = FeatureType::from_str_name(&s).unwrap();
        let int_value: T = enum_value.into();
        return Ok(int_value);
    }
    pub fn serialize<S>(value: &i32, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s: FeatureType = FeatureType::from_i32(*value).unwrap();
        serializer.serialize_str(s.as_str_name())
    }
}
impl FeatureType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            FeatureType::LiquidStakeIbc => "LIQUID_STAKE_IBC",
            FeatureType::LiquidStake => "LIQUID_STAKE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LIQUID_STAKE_IBC" => Some(Self::LiquidStakeIbc),
            "LIQUID_STAKE" => Some(Self::LiquidStake),
            _ => None,
        }
    }
}
/// GenesisState defines the ratesync module's genesis state.
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.GenesisState")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub host_chains: ::prost::alloc::vec::Vec<HostChain>,
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/pstake.ratesync.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.QueryGetHostChainRequest")]
#[proto_query(
    path = "/pstake.ratesync.v1beta1.Query/HostChain",
    response_type = QueryGetHostChainResponse
)]
pub struct QueryGetHostChainRequest {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub i_d: u64,
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.QueryGetHostChainResponse")]
pub struct QueryGetHostChainResponse {
    #[prost(message, optional, tag = "1")]
    pub host_chain: ::core::option::Option<HostChain>,
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.QueryAllHostChainsRequest")]
#[proto_query(
    path = "/pstake.ratesync.v1beta1.Query/AllHostChains",
    response_type = QueryAllHostChainsResponse
)]
pub struct QueryAllHostChainsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.QueryAllHostChainsResponse")]
pub struct QueryAllHostChainsResponse {
    #[prost(message, repeated, tag = "1")]
    pub host_chains: ::prost::alloc::vec::Vec<HostChain>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.MsgCreateHostChain")]
pub struct MsgCreateHostChain {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub host_chain: ::core::option::Option<HostChain>,
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.MsgCreateHostChainResponse")]
pub struct MsgCreateHostChainResponse {
    #[prost(uint64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub i_d: u64,
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.MsgUpdateHostChain")]
pub struct MsgUpdateHostChain {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub host_chain: ::core::option::Option<HostChain>,
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.MsgUpdateHostChainResponse")]
pub struct MsgUpdateHostChainResponse {}
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.MsgDeleteHostChain")]
pub struct MsgDeleteHostChain {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub i_d: u64,
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.MsgDeleteHostChainResponse")]
pub struct MsgDeleteHostChainResponse {}
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
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
#[proto_message(type_url = "/pstake.ratesync.v1beta1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
pub struct RatesyncQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> RatesyncQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn host_chain(
        &self,
        i_d: u64,
    ) -> Result<QueryGetHostChainResponse, cosmwasm_std::StdError> {
        QueryGetHostChainRequest { i_d }.query(self.querier)
    }
    pub fn all_host_chains(
        &self,
        pagination: ::core::option::Option<
            super::super::super::cosmos::base::query::v1beta1::PageRequest,
        >,
    ) -> Result<QueryAllHostChainsResponse, cosmwasm_std::StdError> {
        QueryAllHostChainsRequest { pagination }.query(self.querier)
    }
}
