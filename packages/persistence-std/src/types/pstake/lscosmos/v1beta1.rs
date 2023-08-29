use persistence_std_derive::CosmwasmExt;
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.Params")]
pub struct Params {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.AllowListedValidators")]
pub struct AllowListedValidators {
    #[prost(message, repeated, tag = "1")]
    pub allow_listed_validators: ::prost::alloc::vec::Vec<AllowListedValidator>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.AllowListedValidator")]
pub struct AllowListedValidator {
    /// validator_address defines the bech32-encoded address the allowlisted
    /// validator
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    /// target_weight specifies the target weight for liquid staking, unstaking
    /// amount, which is a value for calculating the real weight to be derived
    /// according to the active status
    #[prost(string, tag = "2")]
    pub target_weight: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.PstakeParams")]
pub struct PstakeParams {
    /// protocol fee in percentage
    #[prost(string, tag = "1")]
    pub pstake_deposit_fee: ::prost::alloc::string::String,
    /// protocol fee in percentage
    #[prost(string, tag = "2")]
    pub pstake_restake_fee: ::prost::alloc::string::String,
    /// protocol fee in percentage
    #[prost(string, tag = "3")]
    pub pstake_unstake_fee: ::prost::alloc::string::String,
    /// protocol fee in percentage
    #[prost(string, tag = "4")]
    pub pstake_redemption_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub pstake_fee_address: ::prost::alloc::string::String,
}
/// HostChainParams go into the DB
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.HostChainParams")]
pub struct HostChainParams {
    #[prost(string, tag = "1")]
    pub chain_i_d: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub connection_i_d: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub transfer_channel: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub transfer_port: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub base_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub mint_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub min_deposit: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "8")]
    pub pstake_params: ::core::option::Option<PstakeParams>,
}
/// DelegationState stores module account balance, ica account balance,
/// delegation state, undelegation state
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.DelegationState")]
pub struct DelegationState {
    /// This field is necessary as the address of not blocked for send coins,
    /// we only should care about funds that have come via proper channels.
    #[prost(message, repeated, tag = "1")]
    pub host_delegation_account_balance: ::prost::alloc::vec::Vec<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
    #[prost(string, tag = "2")]
    pub host_chain_delegation_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub host_account_delegations: ::prost::alloc::vec::Vec<HostAccountDelegation>,
    #[prost(message, repeated, tag = "4")]
    pub host_account_undelegations: ::prost::alloc::vec::Vec<HostAccountUndelegation>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.HostAccountDelegation")]
pub struct HostAccountDelegation {
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.HostAccountUndelegation")]
pub struct HostAccountUndelegation {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub epoch_number: i64,
    #[prost(message, optional, tag = "2")]
    pub total_undelegation_amount: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
    #[prost(message, optional, tag = "3")]
    pub completion_time: ::core::option::Option<crate::shim::Timestamp>,
    #[prost(message, repeated, tag = "4")]
    pub undelegation_entries: ::prost::alloc::vec::Vec<UndelegationEntry>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.UndelegationEntry")]
pub struct UndelegationEntry {
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.HostChainRewardAddress")]
pub struct HostChainRewardAddress {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.IBCAmountTransientStore")]
pub struct IbcAmountTransientStore {
    /// ibc_transfer stores only tokens which have ibc denoms "ibc/HEXHASH"
    #[prost(message, repeated, tag = "1")]
    pub i_b_c_transfer: ::prost::alloc::vec::Vec<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
    /// ica_delegate stores only token which has staking baseDenom
    #[prost(message, optional, tag = "2")]
    pub i_c_a_delegate: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
    #[prost(message, repeated, tag = "3")]
    pub undelegaton_complete_i_b_c_transfer: ::prost::alloc::vec::Vec<
        TransientUndelegationTransfer,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.TransientUndelegationTransfer")]
pub struct TransientUndelegationTransfer {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub epoch_number: i64,
    #[prost(message, optional, tag = "2")]
    pub amount_unbonded: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.UnbondingEpochCValue")]
pub struct UnbondingEpochCValue {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub epoch_number: i64,
    /// c_value = stk_burn.Amount/amount_unbonded.Amount
    #[prost(message, optional, tag = "2")]
    pub s_t_k_burn: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
    #[prost(message, optional, tag = "3")]
    pub amount_unbonded: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
    #[prost(bool, tag = "4")]
    pub is_matured: bool,
    #[prost(bool, tag = "5")]
    pub is_failed: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.DelegatorUnbondingEpochEntry")]
pub struct DelegatorUnbondingEpochEntry {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub epoch_number: i64,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.HostAccounts")]
pub struct HostAccounts {
    #[prost(string, tag = "1")]
    pub delegator_account_owner_i_d: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub rewards_account_owner_i_d: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.MinDepositAndFeeChangeProposal")]
#[deprecated]
pub struct MinDepositAndFeeChangeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub min_deposit: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub pstake_deposit_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub pstake_restake_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub pstake_unstake_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub pstake_redemption_fee: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.PstakeFeeAddressChangeProposal")]
#[deprecated]
pub struct PstakeFeeAddressChangeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub pstake_fee_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/pstake.lscosmos.v1beta1.AllowListedValidatorSetChangeProposal"
)]
#[deprecated]
pub struct AllowListedValidatorSetChangeProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub allow_listed_validators: ::core::option::Option<AllowListedValidators>,
}
/// GenesisState defines the lscosmos module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.GenesisState")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// this line is used by starport scaffolding # genesis/proto/state
    #[prost(bool, tag = "2")]
    pub module_enabled: bool,
    #[prost(message, optional, tag = "3")]
    pub host_chain_params: ::core::option::Option<HostChainParams>,
    #[prost(message, optional, tag = "4")]
    pub allow_listed_validators: ::core::option::Option<AllowListedValidators>,
    #[prost(message, optional, tag = "5")]
    pub delegation_state: ::core::option::Option<DelegationState>,
    #[prost(message, optional, tag = "6")]
    pub host_chain_reward_address: ::core::option::Option<HostChainRewardAddress>,
    #[prost(message, optional, tag = "7")]
    pub i_b_c_amount_transient_store: ::core::option::Option<IbcAmountTransientStore>,
    #[prost(message, repeated, tag = "8")]
    pub unbonding_epoch_c_values: ::prost::alloc::vec::Vec<UnbondingEpochCValue>,
    #[prost(message, repeated, tag = "9")]
    pub delegator_unbonding_epoch_entries: ::prost::alloc::vec::Vec<
        DelegatorUnbondingEpochEntry,
    >,
    #[prost(message, optional, tag = "10")]
    pub host_accounts: ::core::option::Option<HostAccounts>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.MsgLiquidStake")]
pub struct MsgLiquidStake {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.MsgLiquidStakeResponse")]
pub struct MsgLiquidStakeResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.MsgLiquidUnstake")]
pub struct MsgLiquidUnstake {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.MsgLiquidUnstakeResponse")]
pub struct MsgLiquidUnstakeResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.MsgRedeem")]
pub struct MsgRedeem {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.MsgRedeemResponse")]
pub struct MsgRedeemResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.MsgClaim")]
pub struct MsgClaim {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.MsgClaimResponse")]
pub struct MsgClaimResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.MsgRecreateICA")]
pub struct MsgRecreateIca {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.MsgRecreateICAResponse")]
pub struct MsgRecreateIcaResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.MsgJumpStart")]
pub struct MsgJumpStart {
    #[prost(string, tag = "1")]
    pub pstake_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub chain_i_d: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub connection_i_d: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub transfer_channel: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub transfer_port: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub base_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub mint_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub min_deposit: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "9")]
    pub allow_listed_validators: ::core::option::Option<AllowListedValidators>,
    #[prost(message, optional, tag = "10")]
    pub pstake_params: ::core::option::Option<PstakeParams>,
    #[prost(message, optional, tag = "11")]
    pub host_accounts: ::core::option::Option<HostAccounts>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.MsgJumpStartResponse")]
pub struct MsgJumpStartResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.MsgChangeModuleState")]
pub struct MsgChangeModuleState {
    #[prost(string, tag = "1")]
    pub pstake_address: ::prost::alloc::string::String,
    #[prost(bool, tag = "2")]
    pub module_state: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.MsgChangeModuleStateResponse")]
pub struct MsgChangeModuleStateResponse {}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.MsgReportSlashing")]
pub struct MsgReportSlashing {
    #[prost(string, tag = "1")]
    pub pstake_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.MsgReportSlashingResponse")]
pub struct MsgReportSlashingResponse {}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/pstake.lscosmos.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryAllStateRequest is request type for the Query/AllState RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryAllStateRequest")]
#[proto_query(
    path = "/pstake.lscosmos.v1beta1.Query/AllState",
    response_type = QueryAllStateResponse
)]
pub struct QueryAllStateRequest {}
/// QueryAllStateResponse is response type for the Query/AllState RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryAllStateResponse")]
pub struct QueryAllStateResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub genesis: ::core::option::Option<GenesisState>,
}
/// QueryHostChainParamsRequest is request for the Ouery/HostChainParams methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryHostChainParamsRequest")]
#[proto_query(
    path = "/pstake.lscosmos.v1beta1.Query/HostChainParams",
    response_type = QueryHostChainParamsResponse
)]
pub struct QueryHostChainParamsRequest {}
/// QueryHostChainParamsResponse is response for the Ouery/HostChainParams
/// methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryHostChainParamsResponse")]
pub struct QueryHostChainParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub host_chain_params: ::core::option::Option<HostChainParams>,
}
/// QueryDelegationStateRequest is request for the Ouery/DelegationState methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryDelegationStateRequest")]
#[proto_query(
    path = "/pstake.lscosmos.v1beta1.Query/DelegationState",
    response_type = QueryDelegationStateResponse
)]
pub struct QueryDelegationStateRequest {}
/// QueryDelegationStateResponse is response for the Ouery/DelegationState
/// methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryDelegationStateResponse")]
pub struct QueryDelegationStateResponse {
    #[prost(message, optional, tag = "1")]
    pub delegation_state: ::core::option::Option<DelegationState>,
}
/// QueryListedValidatorsRequest is a request for the Query/AllowListedValidators
/// methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryAllowListedValidatorsRequest")]
#[proto_query(
    path = "/pstake.lscosmos.v1beta1.Query/AllowListedValidators",
    response_type = QueryAllowListedValidatorsResponse
)]
pub struct QueryAllowListedValidatorsRequest {}
/// QueryListedValidatorsResponse is a response for the
/// Query/AllowListedValidators methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/pstake.lscosmos.v1beta1.QueryAllowListedValidatorsResponse"
)]
pub struct QueryAllowListedValidatorsResponse {
    #[prost(message, optional, tag = "1")]
    pub allow_listed_validators: ::core::option::Option<AllowListedValidators>,
}
/// QueryCValueRequest is a request for the Query/CValue methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryCValueRequest")]
#[proto_query(
    path = "/pstake.lscosmos.v1beta1.Query/CValue",
    response_type = QueryCValueResponse
)]
pub struct QueryCValueRequest {}
/// QueryCValueRequest is a response for the Query/CValue methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryCValueResponse")]
pub struct QueryCValueResponse {
    #[prost(string, tag = "1")]
    pub c_value: ::prost::alloc::string::String,
}
/// QueryModuleStateRequest is a request for the Query/ModuleState methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryModuleStateRequest")]
#[proto_query(
    path = "/pstake.lscosmos.v1beta1.Query/ModuleState",
    response_type = QueryModuleStateResponse
)]
pub struct QueryModuleStateRequest {}
/// QueryModuleStateRequest is a response for the Query/ModuleState methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryModuleStateResponse")]
pub struct QueryModuleStateResponse {
    #[prost(bool, tag = "1")]
    pub module_state: bool,
}
/// QueryIBCTransientStoreRequest is a request for the Query/IBCTransientStore
/// methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryIBCTransientStoreRequest")]
#[proto_query(
    path = "/pstake.lscosmos.v1beta1.Query/IBCTransientStore",
    response_type = QueryIbcTransientStoreResponse
)]
pub struct QueryIbcTransientStoreRequest {}
/// QueryIBCTransientStoreRequest is a response for the Query/IBCTransientStore
/// methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryIBCTransientStoreResponse")]
pub struct QueryIbcTransientStoreResponse {
    #[prost(message, optional, tag = "1")]
    pub i_b_c_transient_store: ::core::option::Option<IbcAmountTransientStore>,
}
/// QueryUnclaimedRequest is a request for the Query/Unclaimed methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryUnclaimedRequest")]
#[proto_query(
    path = "/pstake.lscosmos.v1beta1.Query/Unclaimed",
    response_type = QueryUnclaimedResponse
)]
pub struct QueryUnclaimedRequest {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
}
/// QueryUnclaimedResponse is a response for the Query/Unclaimed methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryUnclaimedResponse")]
pub struct QueryUnclaimedResponse {
    #[prost(message, repeated, tag = "1")]
    pub unclaimed: ::prost::alloc::vec::Vec<UnbondingEpochCValue>,
}
/// QueryFailedUnbondingsRequest is a request for the Query/FailedUnbondings
/// methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryFailedUnbondingsRequest")]
#[proto_query(
    path = "/pstake.lscosmos.v1beta1.Query/FailedUnbondings",
    response_type = QueryFailedUnbondingsResponse
)]
pub struct QueryFailedUnbondingsRequest {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
}
/// QueryFailedUnbondingsResponse a response for the Query/FailedUnbondings
/// methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryFailedUnbondingsResponse")]
pub struct QueryFailedUnbondingsResponse {
    #[prost(message, repeated, tag = "1")]
    pub failed_unbondings: ::prost::alloc::vec::Vec<UnbondingEpochCValue>,
}
/// QueryPendingUnbondingsRequest is a request for the Query/PendingUnbondings
/// methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryPendingUnbondingsRequest")]
#[proto_query(
    path = "/pstake.lscosmos.v1beta1.Query/PendingUnbondings",
    response_type = QueryPendingUnbondingsResponse
)]
pub struct QueryPendingUnbondingsRequest {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
}
/// QueryPendingUnbondingsResponse is a response for the Query/PendingUnbondings
/// methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryPendingUnbondingsResponse")]
pub struct QueryPendingUnbondingsResponse {
    #[prost(message, repeated, tag = "1")]
    pub pending_unbondings: ::prost::alloc::vec::Vec<UnbondingEpochCValue>,
}
/// QueryUnbondingEpochCValueRequest is a request for the
/// Query/UnbondingEpochCValue methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryUnbondingEpochCValueRequest")]
#[proto_query(
    path = "/pstake.lscosmos.v1beta1.Query/UnbondingEpochCValue",
    response_type = QueryUnbondingEpochCValueResponse
)]
pub struct QueryUnbondingEpochCValueRequest {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub epoch_number: i64,
}
/// QueryUnbondingEpochCValueResponse is a response for the
/// Query/UnbondingEpochCValue methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryUnbondingEpochCValueResponse")]
pub struct QueryUnbondingEpochCValueResponse {
    #[prost(message, optional, tag = "1")]
    pub unbonding_epoch_c_value: ::core::option::Option<UnbondingEpochCValue>,
}
/// QueryHostAccountUndelegationRequest is a request for the
/// Query/HostAccountUndelegation methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/pstake.lscosmos.v1beta1.QueryHostAccountUndelegationRequest"
)]
#[proto_query(
    path = "/pstake.lscosmos.v1beta1.Query/HostAccountUndelegation",
    response_type = QueryHostAccountUndelegationResponse
)]
pub struct QueryHostAccountUndelegationRequest {
    #[prost(int64, tag = "1")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub epoch_number: i64,
}
/// QueryHostAccountUndelegationResponse is a response for the
/// Query/HostAccountUndelegation methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/pstake.lscosmos.v1beta1.QueryHostAccountUndelegationResponse"
)]
pub struct QueryHostAccountUndelegationResponse {
    #[prost(message, optional, tag = "1")]
    pub host_account_undelegation: ::core::option::Option<HostAccountUndelegation>,
}
/// QueryDelegatorUnbondingEpochEntryRequest is a request for the
/// Query/DelegatorUnbondingEpochEntry methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/pstake.lscosmos.v1beta1.QueryDelegatorUnbondingEpochEntryRequest"
)]
#[proto_query(
    path = "/pstake.lscosmos.v1beta1.Query/DelegatorUnbondingEpochEntry",
    response_type = QueryDelegatorUnbondingEpochEntryResponse
)]
pub struct QueryDelegatorUnbondingEpochEntryRequest {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub epoch_number: i64,
}
/// QueryDelegatorUnbondingEpochEntryResponse is a response for the
/// Query/DelegatorUnbondingEpochEntry methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/pstake.lscosmos.v1beta1.QueryDelegatorUnbondingEpochEntryResponse"
)]
pub struct QueryDelegatorUnbondingEpochEntryResponse {
    #[prost(message, optional, tag = "1")]
    pub delegator_unboding_epoch_entry: ::core::option::Option<
        DelegatorUnbondingEpochEntry,
    >,
}
/// QueryHostAccountsRequest is a request for the Query/HostAccounts methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryHostAccountsRequest")]
#[proto_query(
    path = "/pstake.lscosmos.v1beta1.Query/HostAccounts",
    response_type = QueryHostAccountsResponse
)]
pub struct QueryHostAccountsRequest {}
/// QueryHostAccountsResponse is a response for the Query/HostAccounts methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryHostAccountsResponse")]
pub struct QueryHostAccountsResponse {
    #[prost(message, optional, tag = "1")]
    pub host_accounts: ::core::option::Option<HostAccounts>,
}
/// QueryDepositModuleAccountRequest is a request for the
/// Query/DepositModuleAccount methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryDepositModuleAccountRequest")]
#[proto_query(
    path = "/pstake.lscosmos.v1beta1.Query/DepositModuleAccount",
    response_type = QueryDepositModuleAccountResponse
)]
pub struct QueryDepositModuleAccountRequest {}
/// QueryDepositModuleAccountResponse is a response for the
/// Query/DepositModuleAccount methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(type_url = "/pstake.lscosmos.v1beta1.QueryDepositModuleAccountResponse")]
pub struct QueryDepositModuleAccountResponse {
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<
        super::super::super::cosmos::base::v1beta1::Coin,
    >,
}
/// QueryAllDelegatorUnbondingEpochEntriesRequest is a request for the
/// Query/DelegatorUnbondingEpochEntries methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/pstake.lscosmos.v1beta1.QueryAllDelegatorUnbondingEpochEntriesRequest"
)]
#[proto_query(
    path = "/pstake.lscosmos.v1beta1.Query/DelegatorUnbondingEpochEntries",
    response_type = QueryAllDelegatorUnbondingEpochEntriesResponse
)]
pub struct QueryAllDelegatorUnbondingEpochEntriesRequest {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
}
/// QueryAllDelegatorUnbondingEpochEntriesResponse is a response for the
/// Query/DelegatorUnbondingEpochEntries methods.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
#[derive(::serde::Serialize, ::serde::Deserialize, ::schemars::JsonSchema, CosmwasmExt)]
#[proto_message(
    type_url = "/pstake.lscosmos.v1beta1.QueryAllDelegatorUnbondingEpochEntriesResponse"
)]
pub struct QueryAllDelegatorUnbondingEpochEntriesResponse {
    #[prost(message, repeated, tag = "1")]
    pub delegator_unbonding_epoch_entries: ::prost::alloc::vec::Vec<
        DelegatorUnbondingEpochEntry,
    >,
}
pub struct LscosmosQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> LscosmosQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn all_state(&self) -> Result<QueryAllStateResponse, cosmwasm_std::StdError> {
        QueryAllStateRequest {}.query(self.querier)
    }
    pub fn host_chain_params(
        &self,
    ) -> Result<QueryHostChainParamsResponse, cosmwasm_std::StdError> {
        QueryHostChainParamsRequest {}.query(self.querier)
    }
    pub fn delegation_state(
        &self,
    ) -> Result<QueryDelegationStateResponse, cosmwasm_std::StdError> {
        QueryDelegationStateRequest {}.query(self.querier)
    }
    pub fn allow_listed_validators(
        &self,
    ) -> Result<QueryAllowListedValidatorsResponse, cosmwasm_std::StdError> {
        QueryAllowListedValidatorsRequest {
        }
            .query(self.querier)
    }
    pub fn c_value(&self) -> Result<QueryCValueResponse, cosmwasm_std::StdError> {
        QueryCValueRequest {}.query(self.querier)
    }
    pub fn module_state(
        &self,
    ) -> Result<QueryModuleStateResponse, cosmwasm_std::StdError> {
        QueryModuleStateRequest {}.query(self.querier)
    }
    pub fn ibc_transient_store(
        &self,
    ) -> Result<QueryIbcTransientStoreResponse, cosmwasm_std::StdError> {
        QueryIbcTransientStoreRequest {}.query(self.querier)
    }
    pub fn unclaimed(
        &self,
        delegator_address: ::prost::alloc::string::String,
    ) -> Result<QueryUnclaimedResponse, cosmwasm_std::StdError> {
        QueryUnclaimedRequest {
            delegator_address,
        }
            .query(self.querier)
    }
    pub fn failed_unbondings(
        &self,
        delegator_address: ::prost::alloc::string::String,
    ) -> Result<QueryFailedUnbondingsResponse, cosmwasm_std::StdError> {
        QueryFailedUnbondingsRequest {
            delegator_address,
        }
            .query(self.querier)
    }
    pub fn pending_unbondings(
        &self,
        delegator_address: ::prost::alloc::string::String,
    ) -> Result<QueryPendingUnbondingsResponse, cosmwasm_std::StdError> {
        QueryPendingUnbondingsRequest {
            delegator_address,
        }
            .query(self.querier)
    }
    pub fn unbonding_epoch_c_value(
        &self,
        epoch_number: i64,
    ) -> Result<QueryUnbondingEpochCValueResponse, cosmwasm_std::StdError> {
        QueryUnbondingEpochCValueRequest {
            epoch_number,
        }
            .query(self.querier)
    }
    pub fn host_account_undelegation(
        &self,
        epoch_number: i64,
    ) -> Result<QueryHostAccountUndelegationResponse, cosmwasm_std::StdError> {
        QueryHostAccountUndelegationRequest {
            epoch_number,
        }
            .query(self.querier)
    }
    pub fn delegator_unbonding_epoch_entry(
        &self,
        delegator_address: ::prost::alloc::string::String,
        epoch_number: i64,
    ) -> Result<QueryDelegatorUnbondingEpochEntryResponse, cosmwasm_std::StdError> {
        QueryDelegatorUnbondingEpochEntryRequest {
            delegator_address,
            epoch_number,
        }
            .query(self.querier)
    }
    pub fn host_accounts(
        &self,
    ) -> Result<QueryHostAccountsResponse, cosmwasm_std::StdError> {
        QueryHostAccountsRequest {}.query(self.querier)
    }
    pub fn deposit_module_account(
        &self,
    ) -> Result<QueryDepositModuleAccountResponse, cosmwasm_std::StdError> {
        QueryDepositModuleAccountRequest {
        }
            .query(self.querier)
    }
    pub fn delegator_unbonding_epoch_entries(
        &self,
        delegator_address: ::prost::alloc::string::String,
    ) -> Result<QueryAllDelegatorUnbondingEpochEntriesResponse, cosmwasm_std::StdError> {
        QueryAllDelegatorUnbondingEpochEntriesRequest {
            delegator_address,
        }
            .query(self.querier)
    }
}
