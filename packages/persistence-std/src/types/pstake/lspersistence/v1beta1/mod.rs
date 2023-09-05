
use persistence_std_derive::CosmwasmExt;
use pbjson;
/// Params defines the set of params for the liquidstaking module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lspersistence.v1beta1.Params")]
pub struct Params {
    /// LiquidBondDenom specifies the denomination of the token receiving after LiquidStaking, The value is calculated
    /// through NetAmount.
    #[prost(string, tag = "1")]
    pub liquid_bond_denom: ::prost::alloc::string::String,
    /// WhitelistedValidators specifies the validators elected to become Active Liquid Validators.
    #[prost(message, repeated, tag = "2")]
    pub whitelisted_validators: ::prost::alloc::vec::Vec<WhitelistedValidator>,
    #[prost(string, tag = "3")]
    pub stake_fee_rate: ::prost::alloc::string::String,
    /// UnstakeFeeRate specifies the fee rate when liquid unstake is requested, unbonded by subtracting it from
    /// unbondingAmount
    #[prost(string, tag = "4")]
    pub unstake_fee_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub redemption_fee_rate: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub restake_fee_rate: ::prost::alloc::string::String,
    /// MinLiquidStakingAmount specifies the minimum number of coins to be staked to the active liquid validators on liquid
    /// staking to minimize decimal loss and consider gas efficiency.
    #[prost(string, tag = "7")]
    pub min_liquid_staking_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub admin_address: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub fee_address: ::prost::alloc::string::String,
}
/// WhitelistedValidator consists of the validator operator address and the target weight, which is a value for
/// calculating the real weight to be derived according to the active status. In the case of inactive, it is calculated
/// as zero.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lspersistence.v1beta1.WhitelistedValidator")]
pub struct WhitelistedValidator {
    /// validator_address defines the bech32-encoded address that whitelisted validator
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    /// target_weight specifies the target weight for liquid staking, unstaking amount, which is a value for calculating
    /// the real weight to be derived according to the active status
    #[prost(string, tag = "2")]
    pub target_weight: ::prost::alloc::string::String,
}
/// LiquidValidator defines a Validator that can be the target of LiquidStaking and LiquidUnstaking, Active, Weight, etc.
/// fields are derived as functions to deal with by maintaining consistency with the state of the staking module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lspersistence.v1beta1.LiquidValidator")]
pub struct LiquidValidator {
    /// operator_address defines the address of the validator's operator; bech encoded in JSON.
    #[prost(string, tag = "1")]
    pub operator_address: ::prost::alloc::string::String,
}
/// LiquidValidatorState is type LiquidValidator with state added to return to query results.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lspersistence.v1beta1.LiquidValidatorState")]
pub struct LiquidValidatorState {
    /// operator_address defines the address of the validator's operator; bech encoded in JSON.
    #[prost(string, tag = "1")]
    pub operator_address: ::prost::alloc::string::String,
    /// weight specifies the weight for liquid staking, unstaking amount
    #[prost(string, tag = "2")]
    pub weight: ::prost::alloc::string::String,
    /// status is the liquid validator status
    #[prost(enumeration = "ValidatorStatus", tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub status: i32,
    /// del_shares define the delegation shares of the validator
    #[prost(string, tag = "4")]
    pub del_shares: ::prost::alloc::string::String,
    /// liquid_tokens define the token amount worth of delegation shares of the validator (slashing applied amount)
    #[prost(string, tag = "5")]
    pub liquid_tokens: ::prost::alloc::string::String,
}
/// NetAmountState is type for net amount raw data and mint rate, This is a value that depends on the several module
/// state every time, so it is used only for calculation and query and is not stored in kv.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lspersistence.v1beta1.NetAmountState")]
pub struct NetAmountState {
    /// mint_rate is bTokenTotalSupply / NetAmount
    #[prost(string, tag = "1")]
    pub mint_rate: ::prost::alloc::string::String,
    /// btoken_total_supply returns the total supply of btoken(liquid_bond_denom)
    #[prost(string, tag = "2")]
    pub btoken_total_supply: ::prost::alloc::string::String,
    /// net_amount is proxy account's native token balance + total liquid tokens + total remaining rewards + total
    /// unbonding balance
    #[prost(string, tag = "3")]
    pub net_amount: ::prost::alloc::string::String,
    /// total_del_shares define the delegation shares of all liquid validators
    #[prost(string, tag = "4")]
    pub total_del_shares: ::prost::alloc::string::String,
    /// total_liquid_tokens define the token amount worth of delegation shares of all liquid validator (slashing applied
    /// amount)
    #[prost(string, tag = "5")]
    pub total_liquid_tokens: ::prost::alloc::string::String,
    /// total_remaining_rewards define the sum of remaining rewards of proxy account by all liquid validators
    #[prost(string, tag = "6")]
    pub total_remaining_rewards: ::prost::alloc::string::String,
    /// total_unbonding_balance define the unbonding balance of proxy account by all liquid validator (slashing applied
    /// amount)
    #[prost(string, tag = "7")]
    pub total_unbonding_balance: ::prost::alloc::string::String,
    /// proxy_acc_balance define the balance of proxy account for the native token
    #[prost(string, tag = "8")]
    pub proxy_acc_balance: ::prost::alloc::string::String,
}
/// VotingPower is type for current voting power of the voter including staking module's voting power and liquid staking
/// module's voting power, It depends on the amount of delegation of staking module, the bonded state of the delegated
/// validator, the value of btoken(liquid_bond_denom), and the pool coin and farming position containing btoken..
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lspersistence.v1beta1.VotingPower")]
pub struct VotingPower {
    /// voter defines the address of the voter; bech encoded in JSON.
    #[prost(string, tag = "1")]
    pub voter: ::prost::alloc::string::String,
    /// staking_voting_power return the voting power of staking that can be exercised.
    #[prost(string, tag = "2")]
    pub staking_voting_power: ::prost::alloc::string::String,
    /// liquid_staking_voting_power return the voting power of liquid staking that can be exercised.
    #[prost(string, tag = "3")]
    pub liquid_staking_voting_power: ::prost::alloc::string::String,
    /// validator_voting_power return the voting power of the validator if the voter is the validator operator that can be
    /// exercised.
    #[prost(string, tag = "4")]
    pub validator_voting_power: ::prost::alloc::string::String,
}
/// ValidatorStatus enumerates the status of a liquid validator.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
#[derive(::schemars::JsonSchema)]
pub enum ValidatorStatus {
    /// VALIDATOR_STATUS_UNSPECIFIED defines the unspecified invalid status.
    Unspecified = 0,
    /// VALIDATOR_STATUS_ACTIVE defines the active, valid status
    Active = 1,
    /// VALIDATOR_STATUS_INACTIVE defines the inactive, invalid status
    Inactive = 2,
}
impl ValidatorStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ValidatorStatus::Unspecified => "VALIDATOR_STATUS_UNSPECIFIED",
            ValidatorStatus::Active => "VALIDATOR_STATUS_ACTIVE",
            ValidatorStatus::Inactive => "VALIDATOR_STATUS_INACTIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "VALIDATOR_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "VALIDATOR_STATUS_ACTIVE" => Some(Self::Active),
            "VALIDATOR_STATUS_INACTIVE" => Some(Self::Inactive),
            _ => None,
        }
    }
}
/// GenesisState defines the liquidstaking module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lspersistence.v1beta1.GenesisState")]
pub struct GenesisState {
    /// params defines all the parameters for the liquidstaking module
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub liquid_validators: ::prost::alloc::vec::Vec<LiquidValidator>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lspersistence.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/pstake.lspersistence.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lspersistence.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryLiquidValidatorsRequest is the request type for the Query/LiquidValidators RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lspersistence.v1beta1.QueryLiquidValidatorsRequest")]
#[proto_query(
    path = "/pstake.lspersistence.v1beta1.Query/LiquidValidators",
    response_type = QueryLiquidValidatorsResponse
)]
pub struct QueryLiquidValidatorsRequest {}
/// QueryLiquidValidatorsResponse is the response type for the Query/LiquidValidators RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/pstake.lspersistence.v1beta1.QueryLiquidValidatorsResponse"
)]
pub struct QueryLiquidValidatorsResponse {
    #[prost(message, repeated, tag = "1")]
    pub liquid_validators: ::prost::alloc::vec::Vec<LiquidValidatorState>,
}
/// QueryStatesRequest is the request type for the Query/States RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lspersistence.v1beta1.QueryStatesRequest")]
#[proto_query(
    path = "/pstake.lspersistence.v1beta1.Query/States",
    response_type = QueryStatesResponse
)]
pub struct QueryStatesRequest {}
/// QueryStatesResponse is the response type for the Query/States RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lspersistence.v1beta1.QueryStatesResponse")]
pub struct QueryStatesResponse {
    #[prost(message, optional, tag = "1")]
    pub net_amount_state: ::core::option::Option<NetAmountState>,
}
/// MsgLiquidStake defines a SDK message for performing a liquid stake of coins
/// from a delegator to whitelisted validators.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lspersistence.v1beta1.MsgLiquidStake")]
pub struct MsgLiquidStake {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgLiquidStakeResponse defines the Msg/LiquidStake response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lspersistence.v1beta1.MsgLiquidStakeResponse")]
pub struct MsgLiquidStakeResponse {}
/// MsgLiquidUnstake defines a SDK message for performing an undelegation of liquid staking from a
/// delegate.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lspersistence.v1beta1.MsgLiquidUnstake")]
pub struct MsgLiquidUnstake {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
/// MsgLiquidUnstakeResponse defines the Msg/LiquidUnstake response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lspersistence.v1beta1.MsgLiquidUnstakeResponse")]
pub struct MsgLiquidUnstakeResponse {
    #[prost(message, optional, tag = "1")]
    pub completion_time: ::core::option::Option<crate::shim::Timestamp>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lspersistence.v1beta1.MsgUpdateParams")]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/auth parameters to update.
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
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lspersistence.v1beta1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
include!("serde.rs");
pub struct LspersistenceQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> LspersistenceQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn liquid_validators(
        &self,
    ) -> Result<QueryLiquidValidatorsResponse, cosmwasm_std::StdError> {
        QueryLiquidValidatorsRequest {}.query(self.querier)
    }
    pub fn states(&self) -> Result<QueryStatesResponse, cosmwasm_std::StdError> {
        QueryStatesRequest {}.query(self.querier)
    }
}
