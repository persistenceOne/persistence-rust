use persistence_std_derive::CosmwasmExt;
/// StakeAuthorization defines authorization for delegate/undelegate/redelegate.
///
/// Since: cosmos-sdk 0.43
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.StakeAuthorization")]
pub struct StakeAuthorization {
    /// max_tokens specifies the maximum amount of tokens can be delegate to a validator. If it is
    /// empty, there is no spend limit and any amount of coins can be delegated.
    #[prost(message, optional, tag = "1")]
    pub max_tokens: ::core::option::Option<super::super::base::v1beta1::Coin>,
    /// authorization_type defines one of AuthorizationType.
    #[prost(enumeration = "AuthorizationType", tag = "4")]
    #[serde(
        serialize_with = "authorization_type_serde::serialize",
        deserialize_with = "authorization_type_serde::deserialize"
    )]
    pub authorization_type: i32,
    /// validators is the oneof that represents either allow_list or deny_list
    #[prost(oneof = "stake_authorization::Validators", tags = "2, 3")]
    pub validators: ::core::option::Option<stake_authorization::Validators>,
}
/// Nested message and enum types in `StakeAuthorization`.
pub mod stake_authorization {
    use persistence_std_derive::CosmwasmExt;
    /// Validators defines list of validator addresses.
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
    #[proto_message(type_url = "/cosmos.staking.v1beta1.")]
    pub struct Validators_ {
        #[prost(string, repeated, tag = "1")]
        pub address: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    }
    /// validators is the oneof that represents either allow_list or deny_list
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(
        Clone,
        PartialEq,
        Eq,
        ::prost::Oneof,
        ::schemars::JsonSchema,
        ::serde::Serialize,
        ::serde::Deserialize,
    )]
    pub enum Validators {
        /// allow_list specifies list of validator addresses to whom grantee can delegate tokens on behalf of granter's
        /// account.
        #[prost(message, tag = "2")]
        AllowList(Validators_),
        /// deny_list specifies list of validator addresses to whom grantee can not delegate tokens.
        #[prost(message, tag = "3")]
        DenyList(Validators_),
    }
}
/// AuthorizationType defines the type of staking module authorization type
///
/// Since: cosmos-sdk 0.43
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::schemars::JsonSchema)]
pub enum AuthorizationType {
    /// AUTHORIZATION_TYPE_UNSPECIFIED specifies an unknown authorization type
    Unspecified = 0,
    /// AUTHORIZATION_TYPE_DELEGATE defines an authorization type for Msg/Delegate
    Delegate = 1,
    /// AUTHORIZATION_TYPE_UNDELEGATE defines an authorization type for Msg/Undelegate
    Undelegate = 2,
    /// AUTHORIZATION_TYPE_REDELEGATE defines an authorization type for Msg/BeginRedelegate
    Redelegate = 3,
}
pub mod authorization_type_serde {
    use super::AuthorizationType;
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn deserialize<'de, T, D>(deserializer: D) -> std::result::Result<T, D::Error>
    where
        T: From<AuthorizationType>,
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let enum_value = AuthorizationType::from_str_name(&s).unwrap();
        let int_value: T = enum_value.into();
        return Ok(int_value);
    }
    pub fn serialize<S>(value: &i32, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s: AuthorizationType = AuthorizationType::from_i32(*value).unwrap();
        serializer.serialize_str(s.as_str_name())
    }
}
impl AuthorizationType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AuthorizationType::Unspecified => "AUTHORIZATION_TYPE_UNSPECIFIED",
            AuthorizationType::Delegate => "AUTHORIZATION_TYPE_DELEGATE",
            AuthorizationType::Undelegate => "AUTHORIZATION_TYPE_UNDELEGATE",
            AuthorizationType::Redelegate => "AUTHORIZATION_TYPE_REDELEGATE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUTHORIZATION_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "AUTHORIZATION_TYPE_DELEGATE" => Some(Self::Delegate),
            "AUTHORIZATION_TYPE_UNDELEGATE" => Some(Self::Undelegate),
            "AUTHORIZATION_TYPE_REDELEGATE" => Some(Self::Redelegate),
            _ => None,
        }
    }
}
/// HistoricalInfo contains header and validator information for a given block.
/// It is stored as part of staking module's state, which persists the `n` most
/// recent HistoricalInfo
/// (`n` is set by the staking module's `historical_entries` parameter).
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.HistoricalInfo")]
pub struct HistoricalInfo {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<super::super::super::tendermint::types::Header>,
    #[prost(message, repeated, tag = "2")]
    pub valset: ::prost::alloc::vec::Vec<Validator>,
}
/// CommissionRates defines the initial commission rates to be used for creating
/// a validator.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.CommissionRates")]
pub struct CommissionRates {
    /// rate is the commission rate charged to delegators, as a fraction.
    #[prost(string, tag = "1")]
    pub rate: ::prost::alloc::string::String,
    /// max_rate defines the maximum commission rate which validator can ever charge, as a fraction.
    #[prost(string, tag = "2")]
    pub max_rate: ::prost::alloc::string::String,
    /// max_change_rate defines the maximum daily increase of the validator commission, as a fraction.
    #[prost(string, tag = "3")]
    pub max_change_rate: ::prost::alloc::string::String,
}
/// Commission defines commission parameters for a given validator.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.Commission")]
pub struct Commission {
    /// commission_rates defines the initial commission rates to be used for creating a validator.
    #[prost(message, optional, tag = "1")]
    pub commission_rates: ::core::option::Option<CommissionRates>,
    /// update_time is the last time the commission rate was changed.
    #[prost(message, optional, tag = "2")]
    pub update_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// Description defines a validator description.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.Description")]
pub struct Description {
    /// moniker defines a human-readable name for the validator.
    #[prost(string, tag = "1")]
    pub moniker: ::prost::alloc::string::String,
    /// identity defines an optional identity signature (ex. UPort or Keybase).
    #[prost(string, tag = "2")]
    pub identity: ::prost::alloc::string::String,
    /// website defines an optional website link.
    #[prost(string, tag = "3")]
    pub website: ::prost::alloc::string::String,
    /// security_contact defines an optional email for security contact.
    #[prost(string, tag = "4")]
    pub security_contact: ::prost::alloc::string::String,
    /// details define other optional details.
    #[prost(string, tag = "5")]
    pub details: ::prost::alloc::string::String,
}
/// Validator defines a validator, together with the total amount of the
/// Validator's bond shares and their exchange rate to coins. Slashing results in
/// a decrease in the exchange rate, allowing correct calculation of future
/// undelegations without iterating over delegators. When coins are delegated to
/// this validator, the validator is credited with a delegation whose number of
/// bond shares is based on the amount of coins delegated divided by the current
/// exchange rate. Voting power can be calculated as total bonded shares
/// multiplied by exchange rate.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.Validator")]
pub struct Validator {
    /// operator_address defines the address of the validator's operator; bech encoded in JSON.
    #[prost(string, tag = "1")]
    pub operator_address: ::prost::alloc::string::String,
    /// consensus_pubkey is the consensus public key of the validator, as a Protobuf Any.
    #[prost(message, optional, tag = "2")]
    pub consensus_pubkey: ::core::option::Option<crate::shim::Any>,
    /// jailed defined whether the validator has been jailed from bonded status or not.
    #[prost(bool, tag = "3")]
    pub jailed: bool,
    /// status is the validator status (bonded/unbonding/unbonded).
    #[prost(enumeration = "BondStatus", tag = "4")]
    #[serde(
        serialize_with = "bond_status_serde::serialize",
        deserialize_with = "bond_status_serde::deserialize"
    )]
    pub status: i32,
    /// tokens define the delegated tokens (incl. self-delegation).
    #[prost(string, tag = "5")]
    pub tokens: ::prost::alloc::string::String,
    /// delegator_shares defines total shares issued to a validator's delegators.
    #[prost(string, tag = "6")]
    pub delegator_shares: ::prost::alloc::string::String,
    /// description defines the description terms for the validator.
    #[prost(message, optional, tag = "7")]
    pub description: ::core::option::Option<Description>,
    /// unbonding_height defines, if unbonding, the height at which this validator has begun unbonding.
    #[prost(int64, tag = "8")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub unbonding_height: i64,
    /// unbonding_time defines, if unbonding, the min time for the validator to complete unbonding.
    #[prost(message, optional, tag = "9")]
    pub unbonding_time: ::core::option::Option<crate::shim::Timestamp>,
    /// commission defines the commission parameters.
    #[prost(message, optional, tag = "10")]
    pub commission: ::core::option::Option<Commission>,
    /// min_self_delegation is the validator's self declared minimum self delegation.
    #[deprecated]
    #[prost(string, tag = "11")]
    pub min_self_delegation: ::prost::alloc::string::String,
    /// strictly positive if this validator's unbonding has been stopped by external modules
    ///
    /// Since: cosmos-sdk 0.47
    #[prost(int64, tag = "12")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub unbonding_on_hold_ref_count: i64,
    /// list of unbonding ids, each uniquely identifing an unbonding of this validator
    ///
    /// Since: cosmos-sdk 0.47
    #[prost(uint64, repeated, tag = "13")]
    #[serde(alias = "unbondingIDs")]
    pub unbonding_ids: ::prost::alloc::vec::Vec<u64>,
    /// validator_bond_shares is the number of shares self bonded from the validator.
    ///
    /// Since: cosmos-sdk 0.47-lsm
    #[prost(string, tag = "14")]
    pub validator_bond_shares: ::prost::alloc::string::String,
    /// liquid_shares is the number of shares either tokenized or owned by a liquid staking provider.
    ///
    /// Since: cosmos-sdk 0.47-lsm
    #[prost(string, tag = "15")]
    pub liquid_shares: ::prost::alloc::string::String,
}
/// ValAddresses defines a repeated set of validator addresses.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.ValAddresses")]
pub struct ValAddresses {
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// DVPair is struct that just has a delegator-validator pair with no other data.
/// It is intended to be used as a marshalable pointer. For example, a DVPair can
/// be used to construct the key to getting an UnbondingDelegation from state.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.DVPair")]
pub struct DvPair {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
}
/// DVPairs defines an array of DVPair objects.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.DVPairs")]
pub struct DvPairs {
    #[prost(message, repeated, tag = "1")]
    pub pairs: ::prost::alloc::vec::Vec<DvPair>,
}
/// DVVTriplet is struct that just has a delegator-validator-validator triplet
/// with no other data. It is intended to be used as a marshalable pointer. For
/// example, a DVVTriplet can be used to construct the key to getting a
/// Redelegation from state.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.DVVTriplet")]
pub struct DvvTriplet {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_src_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub validator_dst_address: ::prost::alloc::string::String,
}
/// DVVTriplets defines an array of DVVTriplet objects.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.DVVTriplets")]
pub struct DvvTriplets {
    #[prost(message, repeated, tag = "1")]
    pub triplets: ::prost::alloc::vec::Vec<DvvTriplet>,
}
/// Delegation represents the bond with tokens held by an account. It is
/// owned by one delegator, and is associated with the voting power of one
/// validator.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.Delegation")]
pub struct Delegation {
    /// delegator_address is the bech32-encoded address of the delegator.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    /// validator_address is the bech32-encoded address of the validator.
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    /// shares define the delegation shares received.
    #[prost(string, tag = "3")]
    pub shares: ::prost::alloc::string::String,
    /// has this delegation been marked as a validator self bond.
    ///
    /// Since: cosmos-sdk 0.47-lsm
    #[prost(bool, tag = "4")]
    pub validator_bond: bool,
}
/// UnbondingDelegation stores all of a single delegator's unbonding bonds
/// for a single validator in an time-ordered list.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.UnbondingDelegation")]
pub struct UnbondingDelegation {
    /// delegator_address is the bech32-encoded address of the delegator.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    /// validator_address is the bech32-encoded address of the validator.
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    /// entries are the unbonding delegation entries.
    ///
    /// unbonding delegation entries
    #[prost(message, repeated, tag = "3")]
    pub entries: ::prost::alloc::vec::Vec<UnbondingDelegationEntry>,
}
/// UnbondingDelegationEntry defines an unbonding object with relevant metadata.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.UnbondingDelegationEntry")]
pub struct UnbondingDelegationEntry {
    /// creation_height is the height which the unbonding took place.
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub creation_height: i64,
    /// completion_time is the unix time for unbonding completion.
    #[prost(message, optional, tag = "2")]
    pub completion_time: ::core::option::Option<crate::shim::Timestamp>,
    /// initial_balance defines the tokens initially scheduled to receive at completion.
    #[prost(string, tag = "3")]
    pub initial_balance: ::prost::alloc::string::String,
    /// balance defines the tokens to receive at completion.
    #[prost(string, tag = "4")]
    pub balance: ::prost::alloc::string::String,
    /// Incrementing id that uniquely identifies this entry
    #[prost(uint64, tag = "5")]
    #[serde(alias = "unbondingID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub unbonding_id: u64,
    /// Strictly positive if this entry's unbonding has been stopped by external modules
    #[prost(int64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub unbonding_on_hold_ref_count: i64,
    /// validator_bond_factor is required for tokenize share and undelegation check for network safety
    ///
    /// Since: cosmos-sdk 0.47-lsm
    #[prost(string, tag = "7")]
    pub validator_bond_factor: ::prost::alloc::string::String,
    /// global_liquid_staking_cap represents a cap on the portion of stake that
    /// comes from liquid staking providers
    ///
    /// Since: cosmos-sdk 0.47-lsm
    #[prost(string, tag = "8")]
    pub global_liquid_staking_cap: ::prost::alloc::string::String,
}
/// RedelegationEntry defines a redelegation object with relevant metadata.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.RedelegationEntry")]
pub struct RedelegationEntry {
    /// creation_height  defines the height which the redelegation took place.
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub creation_height: i64,
    /// completion_time defines the unix time for redelegation completion.
    #[prost(message, optional, tag = "2")]
    pub completion_time: ::core::option::Option<crate::shim::Timestamp>,
    /// initial_balance defines the initial balance when redelegation started.
    #[prost(string, tag = "3")]
    pub initial_balance: ::prost::alloc::string::String,
    /// shares_dst is the amount of destination-validator shares created by redelegation.
    #[prost(string, tag = "4")]
    pub shares_dst: ::prost::alloc::string::String,
    /// Incrementing id that uniquely identifies this entry
    #[prost(uint64, tag = "5")]
    #[serde(alias = "unbondingID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub unbonding_id: u64,
    /// Strictly positive if this entry's unbonding has been stopped by external modules
    #[prost(int64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub unbonding_on_hold_ref_count: i64,
}
/// Redelegation contains the list of a particular delegator's redelegating bonds
/// from a particular source validator to a particular destination validator.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.Redelegation")]
pub struct Redelegation {
    /// delegator_address is the bech32-encoded address of the delegator.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    /// validator_src_address is the validator redelegation source operator address.
    #[prost(string, tag = "2")]
    pub validator_src_address: ::prost::alloc::string::String,
    /// validator_dst_address is the validator redelegation destination operator address.
    #[prost(string, tag = "3")]
    pub validator_dst_address: ::prost::alloc::string::String,
    /// entries are the redelegation entries.
    ///
    /// redelegation entries
    #[prost(message, repeated, tag = "4")]
    pub entries: ::prost::alloc::vec::Vec<RedelegationEntry>,
}
/// Params defines the parameters for the x/staking module.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.Params")]
pub struct Params {
    /// unbonding_time is the time duration of unbonding.
    #[prost(message, optional, tag = "1")]
    pub unbonding_time: ::core::option::Option<crate::shim::Duration>,
    /// max_validators is the maximum number of validators.
    #[prost(uint32, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_validators: u32,
    /// max_entries is the max entries for either unbonding delegation or redelegation (per pair/trio).
    #[prost(uint32, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub max_entries: u32,
    /// historical_entries is the number of historical entries to persist.
    #[prost(uint32, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub historical_entries: u32,
    /// bond_denom defines the bondable coin denomination.
    #[prost(string, tag = "5")]
    pub bond_denom: ::prost::alloc::string::String,
    /// min_commission_rate is the chain-wide minimum commission rate that a validator can charge their delegators
    #[prost(string, tag = "6")]
    pub min_commission_rate: ::prost::alloc::string::String,
    /// validator_bond_factor is required as a safety check for tokenizing shares and
    /// delegations from liquid staking providers
    #[prost(string, tag = "7")]
    pub validator_bond_factor: ::prost::alloc::string::String,
    /// global_liquid_staking_cap represents a cap on the portion of stake that
    /// comes from liquid staking providers
    #[prost(string, tag = "8")]
    pub global_liquid_staking_cap: ::prost::alloc::string::String,
    /// validator_liquid_staking_cap represents a cap on the portion of stake that
    /// comes from liquid staking providers for a specific validator
    #[prost(string, tag = "9")]
    pub validator_liquid_staking_cap: ::prost::alloc::string::String,
}
/// DelegationResponse is equivalent to Delegation except that it contains a
/// balance in addition to shares which is more suitable for client responses.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.DelegationResponse")]
pub struct DelegationResponse {
    #[prost(message, optional, tag = "1")]
    pub delegation: ::core::option::Option<Delegation>,
    #[prost(message, optional, tag = "2")]
    pub balance: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// RedelegationEntryResponse is equivalent to a RedelegationEntry except that it
/// contains a balance in addition to shares which is more suitable for client
/// responses.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.RedelegationEntryResponse")]
pub struct RedelegationEntryResponse {
    #[prost(message, optional, tag = "1")]
    pub redelegation_entry: ::core::option::Option<RedelegationEntry>,
    #[prost(string, tag = "4")]
    pub balance: ::prost::alloc::string::String,
}
/// RedelegationResponse is equivalent to a Redelegation except that its entries
/// contain a balance in addition to shares which is more suitable for client
/// responses.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.RedelegationResponse")]
pub struct RedelegationResponse {
    #[prost(message, optional, tag = "1")]
    pub redelegation: ::core::option::Option<Redelegation>,
    #[prost(message, repeated, tag = "2")]
    pub entries: ::prost::alloc::vec::Vec<RedelegationEntryResponse>,
}
/// Pool is used for tracking bonded and not-bonded token supply of the bond
/// denomination.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.Pool")]
pub struct Pool {
    #[prost(string, tag = "1")]
    pub not_bonded_tokens: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub bonded_tokens: ::prost::alloc::string::String,
}
/// ValidatorUpdates defines an array of abci.ValidatorUpdate objects.
/// TODO: explore moving this to proto/cosmos/base to separate modules from tendermint dependence
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.ValidatorUpdates")]
pub struct ValidatorUpdates {
    #[prost(message, repeated, tag = "1")]
    pub updates: ::prost::alloc::vec::Vec<super::super::super::tendermint::abci::ValidatorUpdate>,
}
/// TokenizeShareRecord represents a tokenized delegation.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.TokenizeShareRecord")]
pub struct TokenizeShareRecord {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// module account take the role of delegator
    #[prost(string, tag = "3")]
    pub module_account: ::prost::alloc::string::String,
    /// validator delegated to for tokenize share record creation
    #[prost(string, tag = "4")]
    pub validator: ::prost::alloc::string::String,
}
/// PendingTokenizeShareAuthorizations stores a list of addresses that have their
/// tokenize share enablement in progress.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.PendingTokenizeShareAuthorizations")]
pub struct PendingTokenizeShareAuthorizations {
    #[prost(string, repeated, tag = "1")]
    pub addresses: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// BondStatus is the status of a validator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::schemars::JsonSchema)]
pub enum BondStatus {
    /// UNSPECIFIED defines an invalid validator status.
    Unspecified = 0,
    /// UNBONDED defines a validator that is not bonded.
    Unbonded = 1,
    /// UNBONDING defines a validator that is unbonding.
    Unbonding = 2,
    /// BONDED defines a validator that is bonded.
    Bonded = 3,
}
pub mod bond_status_serde {
    use super::BondStatus;
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn deserialize<'de, T, D>(deserializer: D) -> std::result::Result<T, D::Error>
    where
        T: From<BondStatus>,
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let enum_value = BondStatus::from_str_name(&s).unwrap();
        let int_value: T = enum_value.into();
        return Ok(int_value);
    }
    pub fn serialize<S>(value: &i32, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s: BondStatus = BondStatus::from_i32(*value).unwrap();
        serializer.serialize_str(s.as_str_name())
    }
}
impl BondStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BondStatus::Unspecified => "BOND_STATUS_UNSPECIFIED",
            BondStatus::Unbonded => "BOND_STATUS_UNBONDED",
            BondStatus::Unbonding => "BOND_STATUS_UNBONDING",
            BondStatus::Bonded => "BOND_STATUS_BONDED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BOND_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "BOND_STATUS_UNBONDED" => Some(Self::Unbonded),
            "BOND_STATUS_UNBONDING" => Some(Self::Unbonding),
            "BOND_STATUS_BONDED" => Some(Self::Bonded),
            _ => None,
        }
    }
}
/// Infraction indicates the infraction a validator commited.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::schemars::JsonSchema)]
pub enum Infraction {
    /// UNSPECIFIED defines an empty infraction.
    Unspecified = 0,
    /// DOUBLE_SIGN defines a validator that double-signs a block.
    DoubleSign = 1,
    /// DOWNTIME defines a validator that missed signing too many blocks.
    Downtime = 2,
}
pub mod infraction_serde {
    use super::Infraction;
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn deserialize<'de, T, D>(deserializer: D) -> std::result::Result<T, D::Error>
    where
        T: From<Infraction>,
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let enum_value = Infraction::from_str_name(&s).unwrap();
        let int_value: T = enum_value.into();
        return Ok(int_value);
    }
    pub fn serialize<S>(value: &i32, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s: Infraction = Infraction::from_i32(*value).unwrap();
        serializer.serialize_str(s.as_str_name())
    }
}
impl Infraction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Infraction::Unspecified => "INFRACTION_UNSPECIFIED",
            Infraction::DoubleSign => "INFRACTION_DOUBLE_SIGN",
            Infraction::Downtime => "INFRACTION_DOWNTIME",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INFRACTION_UNSPECIFIED" => Some(Self::Unspecified),
            "INFRACTION_DOUBLE_SIGN" => Some(Self::DoubleSign),
            "INFRACTION_DOWNTIME" => Some(Self::Downtime),
            _ => None,
        }
    }
}
/// TokenizeShareLockStatus represents status of an account's tokenize share lock.
///
/// Since: cosmos-sdk 0.47-lsm
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::schemars::JsonSchema)]
pub enum TokenizeShareLockStatus {
    /// An empty value is not allowed.
    Unspecified = 0,
    /// Status means cannot tokenize shares.
    Locked = 1,
    /// Status means cannot tokenize shares.
    Unlocked = 2,
    /// Status when lock is queued for unlocking.
    LockExpiring = 3,
}
pub mod tokenize_share_lock_status_serde {
    use super::TokenizeShareLockStatus;
    use serde::{Deserialize, Deserializer, Serializer};
    pub fn deserialize<'de, T, D>(deserializer: D) -> std::result::Result<T, D::Error>
    where
        T: From<TokenizeShareLockStatus>,
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let enum_value = TokenizeShareLockStatus::from_str_name(&s).unwrap();
        let int_value: T = enum_value.into();
        return Ok(int_value);
    }
    pub fn serialize<S>(value: &i32, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s: TokenizeShareLockStatus = TokenizeShareLockStatus::from_i32(*value).unwrap();
        serializer.serialize_str(s.as_str_name())
    }
}
impl TokenizeShareLockStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TokenizeShareLockStatus::Unspecified => "TOKENIZE_SHARE_LOCK_STATUS_UNSPECIFIED",
            TokenizeShareLockStatus::Locked => "TOKENIZE_SHARE_LOCK_STATUS_LOCKED",
            TokenizeShareLockStatus::Unlocked => "TOKENIZE_SHARE_LOCK_STATUS_UNLOCKED",
            TokenizeShareLockStatus::LockExpiring => "TOKENIZE_SHARE_LOCK_STATUS_LOCK_EXPIRING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "TOKENIZE_SHARE_LOCK_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "TOKENIZE_SHARE_LOCK_STATUS_LOCKED" => Some(Self::Locked),
            "TOKENIZE_SHARE_LOCK_STATUS_UNLOCKED" => Some(Self::Unlocked),
            "TOKENIZE_SHARE_LOCK_STATUS_LOCK_EXPIRING" => Some(Self::LockExpiring),
            _ => None,
        }
    }
}
/// GenesisState defines the staking module's genesis state.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters of related to deposit.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// last_total_power tracks the total amounts of bonded tokens recorded during
    /// the previous end block.
    #[prost(bytes = "vec", tag = "2")]
    pub last_total_power: ::prost::alloc::vec::Vec<u8>,
    /// last_validator_powers is a special index that provides a historical list
    /// of the last-block's bonded validators.
    #[prost(message, repeated, tag = "3")]
    pub last_validator_powers: ::prost::alloc::vec::Vec<LastValidatorPower>,
    /// delegations defines the validator set at genesis.
    #[prost(message, repeated, tag = "4")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    /// delegations defines the delegations active at genesis.
    #[prost(message, repeated, tag = "5")]
    pub delegations: ::prost::alloc::vec::Vec<Delegation>,
    /// unbonding_delegations defines the unbonding delegations active at genesis.
    #[prost(message, repeated, tag = "6")]
    pub unbonding_delegations: ::prost::alloc::vec::Vec<UnbondingDelegation>,
    /// redelegations defines the redelegations active at genesis.
    #[prost(message, repeated, tag = "7")]
    pub redelegations: ::prost::alloc::vec::Vec<Redelegation>,
    #[prost(bool, tag = "8")]
    pub exported: bool,
    /// store tokenize share records to provide reward to record owners.
    ///
    /// Since: cosmos-sdk 0.47-lsm
    #[prost(message, repeated, tag = "9")]
    pub tokenize_share_records: ::prost::alloc::vec::Vec<TokenizeShareRecord>,
    /// last tokenize share record id, used for next share record id calculation.
    ///
    /// Since: cosmos-sdk 0.47-lsm
    #[prost(uint64, tag = "10")]
    #[serde(alias = "last_tokenize_share_recordID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub last_tokenize_share_record_id: u64,
    /// total number of liquid staked tokens at genesis.
    ///
    /// Since: cosmos-sdk 0.47-lsm
    #[prost(bytes = "vec", tag = "11")]
    pub total_liquid_staked_tokens: ::prost::alloc::vec::Vec<u8>,
    /// tokenize shares locks at genesis.
    ///
    /// Since: cosmos-sdk 0.47-lsm
    #[prost(message, repeated, tag = "12")]
    pub tokenize_share_locks: ::prost::alloc::vec::Vec<TokenizeShareLock>,
}
/// TokenizeSharesLock required for specifying account locks at genesis.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.TokenizeShareLock")]
pub struct TokenizeShareLock {
    /// Address of the account that is locked.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// Status of the lock (LOCKED or LOCK_EXPIRING)
    #[prost(string, tag = "2")]
    pub status: ::prost::alloc::string::String,
    /// Completion time if the lock is expiring.
    #[prost(message, optional, tag = "3")]
    pub completion_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// LastValidatorPower required for validator set update logic.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.LastValidatorPower")]
pub struct LastValidatorPower {
    /// address is the address of the validator.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// power defines the power of the validator.
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub power: i64,
}
/// QueryValidatorsRequest is request type for Query/Validators RPC method.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryValidatorsRequest")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/Validators",
    response_type = QueryValidatorsResponse
)]
pub struct QueryValidatorsRequest {
    /// status enables to query for validators matching a given status.
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryValidatorsResponse is response type for the Query/Validators RPC method
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryValidatorsResponse")]
pub struct QueryValidatorsResponse {
    /// validators contains all the queried validators.
    #[prost(message, repeated, tag = "1")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryValidatorRequest is response type for the Query/Validator RPC method
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryValidatorRequest")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/Validator",
    response_type = QueryValidatorResponse
)]
pub struct QueryValidatorRequest {
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryValidatorResponse is response type for the Query/Validator RPC method
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryValidatorResponse")]
pub struct QueryValidatorResponse {
    /// validator defines the validator info.
    #[prost(message, optional, tag = "1")]
    pub validator: ::core::option::Option<Validator>,
}
/// QueryValidatorDelegationsRequest is request type for the
/// Query/ValidatorDelegations RPC method
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryValidatorDelegationsRequest")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/ValidatorDelegations",
    response_type = QueryValidatorDelegationsResponse
)]
pub struct QueryValidatorDelegationsRequest {
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryValidatorDelegationsResponse is response type for the
/// Query/ValidatorDelegations RPC method
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryValidatorDelegationsResponse")]
pub struct QueryValidatorDelegationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub delegation_responses: ::prost::alloc::vec::Vec<DelegationResponse>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryValidatorUnbondingDelegationsRequest is required type for the
/// Query/ValidatorUnbondingDelegations RPC method
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryValidatorUnbondingDelegationsRequest")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/ValidatorUnbondingDelegations",
    response_type = QueryValidatorUnbondingDelegationsResponse
)]
pub struct QueryValidatorUnbondingDelegationsRequest {
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_addr: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryValidatorUnbondingDelegationsResponse is response type for the
/// Query/ValidatorUnbondingDelegations RPC method.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryValidatorUnbondingDelegationsResponse")]
pub struct QueryValidatorUnbondingDelegationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub unbonding_responses: ::prost::alloc::vec::Vec<UnbondingDelegation>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryDelegationRequest is request type for the Query/Delegation RPC method.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryDelegationRequest")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/Delegation",
    response_type = QueryDelegationResponse
)]
pub struct QueryDelegationRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag = "2")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryDelegationResponse is response type for the Query/Delegation RPC method.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryDelegationResponse")]
pub struct QueryDelegationResponse {
    /// delegation_responses defines the delegation info of a delegation.
    #[prost(message, optional, tag = "1")]
    pub delegation_response: ::core::option::Option<DelegationResponse>,
}
/// QueryUnbondingDelegationRequest is request type for the
/// Query/UnbondingDelegation RPC method.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryUnbondingDelegationRequest")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/UnbondingDelegation",
    response_type = QueryUnbondingDelegationResponse
)]
pub struct QueryUnbondingDelegationRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag = "2")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryDelegationResponse is response type for the Query/UnbondingDelegation
/// RPC method.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryUnbondingDelegationResponse")]
pub struct QueryUnbondingDelegationResponse {
    /// unbond defines the unbonding information of a delegation.
    #[prost(message, optional, tag = "1")]
    pub unbond: ::core::option::Option<UnbondingDelegation>,
}
/// QueryDelegatorDelegationsRequest is request type for the
/// Query/DelegatorDelegations RPC method.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryDelegatorDelegationsRequest")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/DelegatorDelegations",
    response_type = QueryDelegatorDelegationsResponse
)]
pub struct QueryDelegatorDelegationsRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryDelegatorDelegationsResponse is response type for the
/// Query/DelegatorDelegations RPC method.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryDelegatorDelegationsResponse")]
pub struct QueryDelegatorDelegationsResponse {
    /// delegation_responses defines all the delegations' info of a delegator.
    #[prost(message, repeated, tag = "1")]
    pub delegation_responses: ::prost::alloc::vec::Vec<DelegationResponse>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryDelegatorUnbondingDelegationsRequest is request type for the
/// Query/DelegatorUnbondingDelegations RPC method.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryDelegatorUnbondingDelegationsRequest")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/DelegatorUnbondingDelegations",
    response_type = QueryDelegatorUnbondingDelegationsResponse
)]
pub struct QueryDelegatorUnbondingDelegationsRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryUnbondingDelegatorDelegationsResponse is response type for the
/// Query/UnbondingDelegatorDelegations RPC method.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryDelegatorUnbondingDelegationsResponse")]
pub struct QueryDelegatorUnbondingDelegationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub unbonding_responses: ::prost::alloc::vec::Vec<UnbondingDelegation>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryRedelegationsRequest is request type for the Query/Redelegations RPC
/// method.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryRedelegationsRequest")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/Redelegations",
    response_type = QueryRedelegationsResponse
)]
pub struct QueryRedelegationsRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// src_validator_addr defines the validator address to redelegate from.
    #[prost(string, tag = "2")]
    pub src_validator_addr: ::prost::alloc::string::String,
    /// dst_validator_addr defines the validator address to redelegate to.
    #[prost(string, tag = "3")]
    pub dst_validator_addr: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "4")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryRedelegationsResponse is response type for the Query/Redelegations RPC
/// method.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryRedelegationsResponse")]
pub struct QueryRedelegationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub redelegation_responses: ::prost::alloc::vec::Vec<RedelegationResponse>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryDelegatorValidatorsRequest is request type for the
/// Query/DelegatorValidators RPC method.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryDelegatorValidatorsRequest")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/DelegatorValidators",
    response_type = QueryDelegatorValidatorsResponse
)]
pub struct QueryDelegatorValidatorsRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryDelegatorValidatorsResponse is response type for the
/// Query/DelegatorValidators RPC method.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryDelegatorValidatorsResponse")]
pub struct QueryDelegatorValidatorsResponse {
    /// validators defines the validators' info of a delegator.
    #[prost(message, repeated, tag = "1")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryDelegatorValidatorRequest is request type for the
/// Query/DelegatorValidator RPC method.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryDelegatorValidatorRequest")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/DelegatorValidator",
    response_type = QueryDelegatorValidatorResponse
)]
pub struct QueryDelegatorValidatorRequest {
    /// delegator_addr defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_addr: ::prost::alloc::string::String,
    /// validator_addr defines the validator address to query for.
    #[prost(string, tag = "2")]
    pub validator_addr: ::prost::alloc::string::String,
}
/// QueryDelegatorValidatorResponse response type for the
/// Query/DelegatorValidator RPC method.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryDelegatorValidatorResponse")]
pub struct QueryDelegatorValidatorResponse {
    /// validator defines the validator info.
    #[prost(message, optional, tag = "1")]
    pub validator: ::core::option::Option<Validator>,
}
/// QueryHistoricalInfoRequest is request type for the Query/HistoricalInfo RPC
/// method.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryHistoricalInfoRequest")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/HistoricalInfo",
    response_type = QueryHistoricalInfoResponse
)]
pub struct QueryHistoricalInfoRequest {
    /// height defines at which height to query the historical info.
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub height: i64,
}
/// QueryHistoricalInfoResponse is response type for the Query/HistoricalInfo RPC
/// method.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryHistoricalInfoResponse")]
pub struct QueryHistoricalInfoResponse {
    /// hist defines the historical info at the given height.
    #[prost(message, optional, tag = "1")]
    pub hist: ::core::option::Option<HistoricalInfo>,
}
/// QueryPoolRequest is request type for the Query/Pool RPC method.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryPoolRequest")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/Pool",
    response_type = QueryPoolResponse
)]
pub struct QueryPoolRequest {}
/// QueryPoolResponse is response type for the Query/Pool RPC method.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryPoolResponse")]
pub struct QueryPoolResponse {
    /// pool defines the pool info.
    #[prost(message, optional, tag = "1")]
    pub pool: ::core::option::Option<Pool>,
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/Params",
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryTokenizeShareRecordByIdRequest is request type for the
/// Query/QueryTokenizeShareRecordById RPC method.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryTokenizeShareRecordByIdRequest")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/TokenizeShareRecordById",
    response_type = QueryTokenizeShareRecordByIdResponse
)]
pub struct QueryTokenizeShareRecordByIdRequest {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
}
/// QueryTokenizeShareRecordByIdRequest is response type for the
/// Query/QueryTokenizeShareRecordById RPC method.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryTokenizeShareRecordByIdResponse")]
pub struct QueryTokenizeShareRecordByIdResponse {
    #[prost(message, optional, tag = "1")]
    pub record: ::core::option::Option<TokenizeShareRecord>,
}
/// QueryTokenizeShareRecordByDenomRequest is request type for the
/// Query/QueryTokenizeShareRecordByDenom RPC method.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryTokenizeShareRecordByDenomRequest")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/TokenizeShareRecordByDenom",
    response_type = QueryTokenizeShareRecordByDenomResponse
)]
pub struct QueryTokenizeShareRecordByDenomRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryTokenizeShareRecordByDenomResponse is response type for the
/// Query/QueryTokenizeShareRecordByDenom RPC method.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryTokenizeShareRecordByDenomResponse")]
pub struct QueryTokenizeShareRecordByDenomResponse {
    #[prost(message, optional, tag = "1")]
    pub record: ::core::option::Option<TokenizeShareRecord>,
}
/// QueryTokenizeShareRecordsOwnedRequest is request type for the
/// Query/QueryTokenizeShareRecordsOwned RPC method.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryTokenizeShareRecordsOwnedRequest")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/TokenizeShareRecordsOwned",
    response_type = QueryTokenizeShareRecordsOwnedResponse
)]
pub struct QueryTokenizeShareRecordsOwnedRequest {
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
/// QueryTokenizeShareRecordsOwnedResponse is response type for the
/// Query/QueryTokenizeShareRecordsOwned RPC method.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryTokenizeShareRecordsOwnedResponse")]
pub struct QueryTokenizeShareRecordsOwnedResponse {
    #[prost(message, repeated, tag = "1")]
    pub records: ::prost::alloc::vec::Vec<TokenizeShareRecord>,
}
/// QueryTotalTokenizeSharedAssetsRequest is request type for the
/// Query/QueryTotalTokenizeSharedAssets RPC method.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryAllTokenizeShareRecordsRequest")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/AllTokenizeShareRecords",
    response_type = QueryAllTokenizeShareRecordsResponse
)]
pub struct QueryAllTokenizeShareRecordsRequest {}
/// QueryTotalTokenizeSharedAssetsResponse is response type for the
/// Query/QueryTotalTokenizeSharedAssets RPC method.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryAllTokenizeShareRecordsResponse")]
pub struct QueryAllTokenizeShareRecordsResponse {
    #[prost(message, repeated, tag = "1")]
    pub records: ::prost::alloc::vec::Vec<TokenizeShareRecord>,
}
/// QueryLastTokenizeShareRecordIdRequest
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryLastTokenizeShareRecordIdRequest")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/LastTokenizeShareRecordId",
    response_type = QueryLastTokenizeShareRecordIdResponse
)]
pub struct QueryLastTokenizeShareRecordIdRequest {}
/// QueryLastTokenizeShareRecordIdResponse
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryLastTokenizeShareRecordIdResponse")]
pub struct QueryLastTokenizeShareRecordIdResponse {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "ID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub id: u64,
}
/// QueryTotalTokenizeSharedAssetsRequest is request type for the
/// Query/QueryTotalTokenizeSharedAssets RPC method.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryTotalTokenizeSharedAssetsRequest")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/TotalTokenizeSharedAssets",
    response_type = QueryTotalTokenizeSharedAssetsResponse
)]
pub struct QueryTotalTokenizeSharedAssetsRequest {}
/// QueryTotalTokenizeSharedAssetsResponse is response type for the
/// Query/QueryTotalTokenizeSharedAssets RPC method.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryTotalTokenizeSharedAssetsResponse")]
pub struct QueryTotalTokenizeSharedAssetsResponse {
    #[prost(message, optional, tag = "1")]
    pub value: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// QueryQueryTotalLiquidStakedRequest is request type for the
/// Query/QueryQueryTotalLiquidStaked RPC method.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryTotalLiquidStaked")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/TotalLiquidStaked",
    response_type = QueryTotalLiquidStakedResponse
)]
pub struct QueryTotalLiquidStaked {}
/// QueryQueryTotalLiquidStakedResponse is response type for the
/// Query/QueryQueryTotalLiquidStaked RPC method.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryTotalLiquidStakedResponse")]
pub struct QueryTotalLiquidStakedResponse {
    #[prost(string, tag = "1")]
    pub tokens: ::prost::alloc::string::String,
}
/// QueryTokenizeShareLockInfo queries the tokenize share lock information
/// associated with given account.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryTokenizeShareLockInfo")]
#[proto_query(
    path = "/cosmos.staking.v1beta1.Query/TokenizeShareLockInfo",
    response_type = QueryTokenizeShareLockInfoResponse
)]
pub struct QueryTokenizeShareLockInfo {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryTokenizeShareLockInfoResponse is the response from the
/// QueryTokenizeShareLockInfo query.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.QueryTokenizeShareLockInfoResponse")]
pub struct QueryTokenizeShareLockInfoResponse {
    #[prost(string, tag = "1")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub expiration_time: ::prost::alloc::string::String,
}
/// MsgCreateValidator defines a SDK message for creating a new validator.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgCreateValidator")]
pub struct MsgCreateValidator {
    #[prost(message, optional, tag = "1")]
    pub description: ::core::option::Option<Description>,
    #[prost(message, optional, tag = "2")]
    pub commission: ::core::option::Option<CommissionRates>,
    #[deprecated]
    #[prost(string, tag = "3")]
    pub min_self_delegation: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub pubkey: ::core::option::Option<crate::shim::Any>,
    #[prost(message, optional, tag = "7")]
    pub value: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// MsgCreateValidatorResponse defines the Msg/CreateValidator response type.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgCreateValidatorResponse")]
pub struct MsgCreateValidatorResponse {}
/// MsgEditValidator defines a SDK message for editing an existing validator.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgEditValidator")]
pub struct MsgEditValidator {
    #[prost(message, optional, tag = "1")]
    pub description: ::core::option::Option<Description>,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    /// We pass a reference to the new commission rate and min self delegation as
    /// it's not mandatory to update. If not updated, the deserialized rate will be
    /// zero with no way to distinguish if an update was intended.
    /// REF: #2373
    #[prost(string, tag = "3")]
    pub commission_rate: ::prost::alloc::string::String,
    #[deprecated]
    #[prost(string, tag = "4")]
    pub min_self_delegation: ::prost::alloc::string::String,
}
/// MsgEditValidatorResponse defines the Msg/EditValidator response type.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgEditValidatorResponse")]
pub struct MsgEditValidatorResponse {}
/// MsgDelegate defines a SDK message for performing a delegation of coins
/// from a delegator to a validator.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgDelegate")]
pub struct MsgDelegate {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// MsgDelegateResponse defines the Msg/Delegate response type.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgDelegateResponse")]
pub struct MsgDelegateResponse {}
/// MsgBeginRedelegate defines a SDK message for performing a redelegation
/// of coins from a delegator and source validator to a destination validator.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgBeginRedelegate")]
pub struct MsgBeginRedelegate {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_src_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub validator_dst_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub amount: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// MsgBeginRedelegateResponse defines the Msg/BeginRedelegate response type.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgBeginRedelegateResponse")]
pub struct MsgBeginRedelegateResponse {
    #[prost(message, optional, tag = "1")]
    pub completion_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// MsgUndelegate defines a SDK message for performing an undelegation from a
/// delegate and a validator.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgUndelegate")]
pub struct MsgUndelegate {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// MsgUndelegateResponse defines the Msg/Undelegate response type.
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgUndelegateResponse")]
pub struct MsgUndelegateResponse {
    #[prost(message, optional, tag = "1")]
    pub completion_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// MsgCancelUnbondingDelegation defines the SDK message for performing a cancel unbonding delegation for delegator
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgCancelUnbondingDelegation")]
pub struct MsgCancelUnbondingDelegation {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    /// amount is always less than or equal to unbonding delegation entry balance
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::base::v1beta1::Coin>,
    /// creation_height is the height which the unbonding took place.
    #[prost(int64, tag = "4")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub creation_height: i64,
}
/// MsgCancelUnbondingDelegationResponse
///
/// Since: cosmos-sdk 0.46
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgCancelUnbondingDelegationResponse")]
pub struct MsgCancelUnbondingDelegationResponse {}
/// MsgUpdateParams is the Msg/UpdateParams request type.
///
/// Since: cosmos-sdk 0.47
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/staking parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
///
/// Since: cosmos-sdk 0.47
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
/// MsgUnbondValidator defines a method for performing the status transition for
/// a validator from bonded to unbonding.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgUnbondValidator")]
pub struct MsgUnbondValidator {
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
}
/// MsgUnbondValidatorResponse defines the MsgUnbondValidator response type.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgUnbondValidatorResponse")]
pub struct MsgUnbondValidatorResponse {}
/// MsgTokenizeShares tokenizes a delegation.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgTokenizeShares")]
pub struct MsgTokenizeShares {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::base::v1beta1::Coin>,
    #[prost(string, tag = "4")]
    pub tokenized_share_owner: ::prost::alloc::string::String,
}
/// MsgTokenizeSharesResponse defines the MsgTokenizeShares response type.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgTokenizeSharesResponse")]
pub struct MsgTokenizeSharesResponse {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// MsgRedeemTokensForShares redeems a tokenized share back into a native delegation.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgRedeemTokensForShares")]
pub struct MsgRedeemTokensForShares {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// MsgRedeemTokensForSharesResponse defines the MsgRedeemTokensForShares response type.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgRedeemTokensForSharesResponse")]
pub struct MsgRedeemTokensForSharesResponse {
    #[prost(message, optional, tag = "1")]
    pub amount: ::core::option::Option<super::super::base::v1beta1::Coin>,
}
/// MsgTransferTokenizeShareRecord transfer a tokenize share record.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgTransferTokenizeShareRecord")]
pub struct MsgTransferTokenizeShareRecord {
    #[prost(uint64, tag = "1")]
    #[serde(alias = "tokenize_share_recordID")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub tokenize_share_record_id: u64,
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub new_owner: ::prost::alloc::string::String,
}
/// MsgTransferTokenizeShareRecordResponse defines the MsgTransferTokenizeShareRecord response type.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgTransferTokenizeShareRecordResponse")]
pub struct MsgTransferTokenizeShareRecordResponse {}
/// MsgDisableTokenizeShares prevents the tokenization of shares for a given address.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgDisableTokenizeShares")]
pub struct MsgDisableTokenizeShares {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
}
/// MsgDisableTokenizeSharesResponse defines the /DisableTokenizeShares response type.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgDisableTokenizeSharesResponse")]
pub struct MsgDisableTokenizeSharesResponse {}
/// MsgEnableTokenizeShares re-enables tokenization of shares for a given address.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgEnableTokenizeShares")]
pub struct MsgEnableTokenizeShares {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
}
/// MsgEnableTokenizeSharesResponse defines the EnableTokenizeShares response type.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgEnableTokenizeSharesResponse")]
pub struct MsgEnableTokenizeSharesResponse {
    #[prost(message, optional, tag = "1")]
    pub completion_time: ::core::option::Option<crate::shim::Timestamp>,
}
/// MsgValidatorBond defines a SDK message for performing validator self-bond of delegated coins
/// from a delegator to a validator.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgValidatorBond")]
pub struct MsgValidatorBond {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
}
/// MsgValidatorBondResponse defines the ValidatorBond response type.
///
/// Since: cosmos-sdk 0.47-lsm
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
#[proto_message(type_url = "/cosmos.staking.v1beta1.MsgValidatorBondResponse")]
pub struct MsgValidatorBondResponse {}
pub struct StakingQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> StakingQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn validators(
        &self,
        status: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryValidatorsResponse, cosmwasm_std::StdError> {
        QueryValidatorsRequest { status, pagination }.query(self.querier)
    }
    pub fn validator(
        &self,
        validator_addr: ::prost::alloc::string::String,
    ) -> Result<QueryValidatorResponse, cosmwasm_std::StdError> {
        QueryValidatorRequest { validator_addr }.query(self.querier)
    }
    pub fn validator_delegations(
        &self,
        validator_addr: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryValidatorDelegationsResponse, cosmwasm_std::StdError> {
        QueryValidatorDelegationsRequest {
            validator_addr,
            pagination,
        }
        .query(self.querier)
    }
    pub fn validator_unbonding_delegations(
        &self,
        validator_addr: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryValidatorUnbondingDelegationsResponse, cosmwasm_std::StdError> {
        QueryValidatorUnbondingDelegationsRequest {
            validator_addr,
            pagination,
        }
        .query(self.querier)
    }
    pub fn delegation(
        &self,
        delegator_addr: ::prost::alloc::string::String,
        validator_addr: ::prost::alloc::string::String,
    ) -> Result<QueryDelegationResponse, cosmwasm_std::StdError> {
        QueryDelegationRequest {
            delegator_addr,
            validator_addr,
        }
        .query(self.querier)
    }
    pub fn unbonding_delegation(
        &self,
        delegator_addr: ::prost::alloc::string::String,
        validator_addr: ::prost::alloc::string::String,
    ) -> Result<QueryUnbondingDelegationResponse, cosmwasm_std::StdError> {
        QueryUnbondingDelegationRequest {
            delegator_addr,
            validator_addr,
        }
        .query(self.querier)
    }
    pub fn delegator_delegations(
        &self,
        delegator_addr: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryDelegatorDelegationsResponse, cosmwasm_std::StdError> {
        QueryDelegatorDelegationsRequest {
            delegator_addr,
            pagination,
        }
        .query(self.querier)
    }
    pub fn delegator_unbonding_delegations(
        &self,
        delegator_addr: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryDelegatorUnbondingDelegationsResponse, cosmwasm_std::StdError> {
        QueryDelegatorUnbondingDelegationsRequest {
            delegator_addr,
            pagination,
        }
        .query(self.querier)
    }
    pub fn redelegations(
        &self,
        delegator_addr: ::prost::alloc::string::String,
        src_validator_addr: ::prost::alloc::string::String,
        dst_validator_addr: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryRedelegationsResponse, cosmwasm_std::StdError> {
        QueryRedelegationsRequest {
            delegator_addr,
            src_validator_addr,
            dst_validator_addr,
            pagination,
        }
        .query(self.querier)
    }
    pub fn delegator_validators(
        &self,
        delegator_addr: ::prost::alloc::string::String,
        pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
    ) -> Result<QueryDelegatorValidatorsResponse, cosmwasm_std::StdError> {
        QueryDelegatorValidatorsRequest {
            delegator_addr,
            pagination,
        }
        .query(self.querier)
    }
    pub fn delegator_validator(
        &self,
        delegator_addr: ::prost::alloc::string::String,
        validator_addr: ::prost::alloc::string::String,
    ) -> Result<QueryDelegatorValidatorResponse, cosmwasm_std::StdError> {
        QueryDelegatorValidatorRequest {
            delegator_addr,
            validator_addr,
        }
        .query(self.querier)
    }
    pub fn historical_info(
        &self,
        height: i64,
    ) -> Result<QueryHistoricalInfoResponse, cosmwasm_std::StdError> {
        QueryHistoricalInfoRequest { height }.query(self.querier)
    }
    pub fn pool(&self) -> Result<QueryPoolResponse, cosmwasm_std::StdError> {
        QueryPoolRequest {}.query(self.querier)
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn tokenize_share_record_by_id(
        &self,
        id: u64,
    ) -> Result<QueryTokenizeShareRecordByIdResponse, cosmwasm_std::StdError> {
        QueryTokenizeShareRecordByIdRequest { id }.query(self.querier)
    }
    pub fn tokenize_share_record_by_denom(
        &self,
        denom: ::prost::alloc::string::String,
    ) -> Result<QueryTokenizeShareRecordByDenomResponse, cosmwasm_std::StdError> {
        QueryTokenizeShareRecordByDenomRequest { denom }.query(self.querier)
    }
    pub fn tokenize_share_records_owned(
        &self,
        owner: ::prost::alloc::string::String,
    ) -> Result<QueryTokenizeShareRecordsOwnedResponse, cosmwasm_std::StdError> {
        QueryTokenizeShareRecordsOwnedRequest { owner }.query(self.querier)
    }
    pub fn all_tokenize_share_records(
        &self,
    ) -> Result<QueryAllTokenizeShareRecordsResponse, cosmwasm_std::StdError> {
        QueryAllTokenizeShareRecordsRequest {}.query(self.querier)
    }
    pub fn last_tokenize_share_record_id(
        &self,
    ) -> Result<QueryLastTokenizeShareRecordIdResponse, cosmwasm_std::StdError> {
        QueryLastTokenizeShareRecordIdRequest {}.query(self.querier)
    }
    pub fn total_tokenize_shared_assets(
        &self,
    ) -> Result<QueryTotalTokenizeSharedAssetsResponse, cosmwasm_std::StdError> {
        QueryTotalTokenizeSharedAssetsRequest {}.query(self.querier)
    }
    pub fn total_liquid_staked(
        &self,
    ) -> Result<QueryTotalLiquidStakedResponse, cosmwasm_std::StdError> {
        QueryTotalLiquidStaked {}.query(self.querier)
    }
    pub fn tokenize_share_lock_info(
        &self,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryTokenizeShareLockInfoResponse, cosmwasm_std::StdError> {
        QueryTokenizeShareLockInfo { address }.query(self.querier)
    }
}
