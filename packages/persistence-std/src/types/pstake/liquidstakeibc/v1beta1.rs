use persistence_std_derive::CosmwasmExt;
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.Params")]
pub struct Params {
    /// protocol admin address
    #[prost(string, tag = "1")]
    pub admin_address: ::prost::alloc::string::String,
    /// protocol fee address
    #[prost(string, tag = "2")]
    pub fee_address: ::prost::alloc::string::String,
    /// upper limit for the c value of a host chain
    #[prost(string, tag = "3")]
    pub upper_c_value_limit: ::prost::alloc::string::String,
    /// lower limit for the c value of a host chain
    #[prost(string, tag = "4")]
    pub lower_c_value_limit: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.HostChain")]
pub struct HostChain {
    /// host chain id
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// ibc connection id
    #[prost(string, tag = "2")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    /// module params
    #[prost(message, optional, tag = "3")]
    pub params: ::core::option::Option<HostChainLsParams>,
    /// native token denom
    #[prost(string, tag = "4")]
    pub host_denom: ::prost::alloc::string::String,
    /// ibc connection channel id
    #[prost(string, tag = "5")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    /// ibc connection port id
    #[prost(string, tag = "6")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    /// delegation host account
    #[prost(message, optional, tag = "7")]
    pub delegation_account: ::core::option::Option<IcaAccount>,
    /// reward host account
    #[prost(message, optional, tag = "8")]
    pub rewards_account: ::core::option::Option<IcaAccount>,
    /// validator set
    #[prost(message, repeated, tag = "9")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    /// minimum ls amount
    #[prost(string, tag = "10")]
    pub minimum_deposit: ::prost::alloc::string::String,
    /// redemption rate
    #[prost(string, tag = "11")]
    pub c_value: ::prost::alloc::string::String,
    /// previous redemption rate
    #[prost(string, tag = "12")]
    pub last_c_value: ::prost::alloc::string::String,
    /// undelegation epoch factor
    #[prost(int64, tag = "13")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub unbonding_factor: i64,
    /// whether the chain is ready to accept delegations or not
    #[prost(bool, tag = "14")]
    pub active: bool,
    /// factor limit for auto-compounding, daily periodic rate (APY / 365s)
    #[prost(string, tag = "15")]
    pub auto_compound_factor: ::prost::alloc::string::String,
    /// host chain flags
    #[prost(message, optional, tag = "16")]
    pub flags: ::core::option::Option<HostChainFlags>,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.HostChainFlags")]
pub struct HostChainFlags {
    #[prost(bool, tag = "1")]
    pub lsm: bool,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.HostChainLSParams")]
pub struct HostChainLsParams {
    /// protocol fee in percentage
    #[prost(string, tag = "1")]
    pub deposit_fee: ::prost::alloc::string::String,
    /// protocol fee in percentage
    #[prost(string, tag = "2")]
    pub restake_fee: ::prost::alloc::string::String,
    /// protocol fee in percentage
    #[prost(string, tag = "3")]
    pub unstake_fee: ::prost::alloc::string::String,
    /// protocol fee in percentage
    #[prost(string, tag = "4")]
    pub redemption_fee: ::prost::alloc::string::String,
    /// LSM validator cap
    ///   Should be used only when HostChainFlag.Lsm == true, orelse default
    #[prost(string, tag = "6")]
    pub lsm_validator_cap: ::prost::alloc::string::String,
    /// LSM bond factor
    ///   Should be used only when HostChainFlag.Lsm == true, orelse default
    #[prost(string, tag = "7")]
    pub lsm_bond_factor: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.ICAAccount")]
pub struct IcaAccount {
    /// address of the ica on the controller chain
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// token balance of the ica
    #[prost(message, optional, tag = "2")]
    pub balance: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// owner string
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
    #[prost(enumeration = "ica_account::ChannelState", tag = "4")]
    #[serde(
        serialize_with = "ica_account::channel_state_serde::serialize",
        deserialize_with = "ica_account::channel_state_serde::deserialize"
    )]
    pub channel_state: i32,
}
/// Nested message and enum types in `ICAAccount`.
pub mod ica_account {
    use persistence_std_derive::CosmwasmExt;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    #[derive(::schemars::JsonSchema)]
    pub enum ChannelState {
        /// ICA channel is being created
        IcaChannelCreating = 0,
        /// ICA is established and the account can be used
        IcaChannelCreated = 1,
    }
    pub mod channel_state_serde {
        use super::ChannelState;
        use serde::{Deserialize, Deserializer, Serializer};
        pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
        where
            T: From<ChannelState>,
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let enum_value = ChannelState::from_str_name(&s).unwrap();
            let int_value: T = enum_value.into();
            return Ok(int_value);
        }
        pub fn serialize<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let s: ChannelState = ChannelState::from_i32(*value).unwrap();
            serializer.serialize_str(s.as_str_name())
        }
    }
    impl ChannelState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                ChannelState::IcaChannelCreating => "ICA_CHANNEL_CREATING",
                ChannelState::IcaChannelCreated => "ICA_CHANNEL_CREATED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "ICA_CHANNEL_CREATING" => Some(Self::IcaChannelCreating),
                "ICA_CHANNEL_CREATED" => Some(Self::IcaChannelCreated),
                _ => None,
            }
        }
    }
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.Validator")]
pub struct Validator {
    /// valoper address
    #[prost(string, tag = "1")]
    pub operator_address: ::prost::alloc::string::String,
    /// validator status
    #[prost(string, tag = "2")]
    pub status: ::prost::alloc::string::String,
    /// validator weight in the set
    #[prost(string, tag = "3")]
    pub weight: ::prost::alloc::string::String,
    /// amount delegated by the module to the validator
    #[prost(string, tag = "4")]
    pub delegated_amount: ::prost::alloc::string::String,
    /// the validator token exchange rate, total bonded tokens divided by total shares issued
    #[prost(string, tag = "5")]
    pub exchange_rate: ::prost::alloc::string::String,
    /// the unbonding epoch number when the validator transitioned into the state
    #[prost(int64, tag = "6")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub unbonding_epoch: i64,
    /// whether the validator can accept delegations or not, default true for non-lsm chains
    #[prost(bool, tag = "7")]
    pub delegable: bool,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.Deposit")]
pub struct Deposit {
    /// deposit target chain
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// epoch number of the deposit
    #[prost(int64, tag = "3")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub epoch: i64,
    /// state
    #[prost(enumeration = "deposit::DepositState", tag = "4")]
    #[serde(
        serialize_with = "deposit::deposit_state_serde::serialize",
        deserialize_with = "deposit::deposit_state_serde::deserialize"
    )]
    pub state: i32,
    /// sequence id of the ibc transaction
    #[prost(string, tag = "5")]
    #[serde(alias = "ibc_sequenceID")]
    pub ibc_sequence_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `Deposit`.
pub mod deposit {
    use persistence_std_derive::CosmwasmExt;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    #[derive(::schemars::JsonSchema)]
    pub enum DepositState {
        /// no action has been initiated on the deposit
        DepositPending = 0,
        /// deposit sent to the host chain delegator address
        DepositSent = 1,
        /// deposit received by the host chain delegator address
        DepositReceived = 2,
        /// delegation submitted for the deposit on the host chain
        DepositDelegating = 3,
    }
    pub mod deposit_state_serde {
        use super::DepositState;
        use serde::{Deserialize, Deserializer, Serializer};
        pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
        where
            T: From<DepositState>,
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let enum_value = DepositState::from_str_name(&s).unwrap();
            let int_value: T = enum_value.into();
            return Ok(int_value);
        }
        pub fn serialize<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let s: DepositState = DepositState::from_i32(*value).unwrap();
            serializer.serialize_str(s.as_str_name())
        }
    }
    impl DepositState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                DepositState::DepositPending => "DEPOSIT_PENDING",
                DepositState::DepositSent => "DEPOSIT_SENT",
                DepositState::DepositReceived => "DEPOSIT_RECEIVED",
                DepositState::DepositDelegating => "DEPOSIT_DELEGATING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEPOSIT_PENDING" => Some(Self::DepositPending),
                "DEPOSIT_SENT" => Some(Self::DepositSent),
                "DEPOSIT_RECEIVED" => Some(Self::DepositReceived),
                "DEPOSIT_DELEGATING" => Some(Self::DepositDelegating),
                _ => None,
            }
        }
    }
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.LSMDeposit")]
pub struct LsmDeposit {
    /// deposit target chain
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// this is calculated when liquid staking [lsm_shares * validator_exchange_rate]
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    /// LSM token shares, they are mapped 1:1 with the delegator shares that are tokenized
    /// <https://github.com/iqlusioninc/cosmos-sdk/pull/19>
    #[prost(string, tag = "3")]
    pub shares: ::prost::alloc::string::String,
    /// LSM token denom
    #[prost(string, tag = "4")]
    pub denom: ::prost::alloc::string::String,
    /// LSM token ibc denom
    #[prost(string, tag = "5")]
    pub ibc_denom: ::prost::alloc::string::String,
    /// address of the delegator
    #[prost(string, tag = "6")]
    pub delegator_address: ::prost::alloc::string::String,
    /// state o the deposit
    #[prost(enumeration = "lsm_deposit::LsmDepositState", tag = "7")]
    #[serde(
        serialize_with = "lsm_deposit::lsm_deposit_state_serde::serialize",
        deserialize_with = "lsm_deposit::lsm_deposit_state_serde::deserialize"
    )]
    pub state: i32,
    /// sequence id of the ibc transaction
    #[prost(string, tag = "8")]
    #[serde(alias = "ibc_sequenceID")]
    pub ibc_sequence_id: ::prost::alloc::string::String,
}
/// Nested message and enum types in `LSMDeposit`.
pub mod lsm_deposit {
    use persistence_std_derive::CosmwasmExt;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    #[derive(::schemars::JsonSchema)]
    pub enum LsmDepositState {
        /// no action has been initiated on the deposit
        DepositPending = 0,
        /// deposit sent to the host chain delegator address
        DepositSent = 1,
        /// deposit received by the host chain delegator address
        DepositReceived = 2,
        /// deposit started the untokenization process
        DepositUntokenizing = 3,
    }
    pub mod lsm_deposit_state_serde {
        use super::LsmDepositState;
        use serde::{Deserialize, Deserializer, Serializer};
        pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
        where
            T: From<LsmDepositState>,
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let enum_value = LsmDepositState::from_str_name(&s).unwrap();
            let int_value: T = enum_value.into();
            return Ok(int_value);
        }
        pub fn serialize<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let s: LsmDepositState = LsmDepositState::from_i32(*value).unwrap();
            serializer.serialize_str(s.as_str_name())
        }
    }
    impl LsmDepositState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                LsmDepositState::DepositPending => "DEPOSIT_PENDING",
                LsmDepositState::DepositSent => "DEPOSIT_SENT",
                LsmDepositState::DepositReceived => "DEPOSIT_RECEIVED",
                LsmDepositState::DepositUntokenizing => "DEPOSIT_UNTOKENIZING",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "DEPOSIT_PENDING" => Some(Self::DepositPending),
                "DEPOSIT_SENT" => Some(Self::DepositSent),
                "DEPOSIT_RECEIVED" => Some(Self::DepositReceived),
                "DEPOSIT_UNTOKENIZING" => Some(Self::DepositUntokenizing),
                _ => None,
            }
        }
    }
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.Unbonding")]
pub struct Unbonding {
    /// unbonding target chain
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// epoch number of the unbonding record
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub epoch_number: i64,
    /// time when the unbonding matures and can be collected
    #[prost(message, optional, tag = "3")]
    pub mature_time: ::core::option::Option<crate::shim::Timestamp>,
    /// stk token amount that is burned with the unbonding
    #[prost(message, optional, tag = "4")]
    pub burn_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// host token amount that is being unbonded
    #[prost(message, optional, tag = "5")]
    pub unbond_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// sequence id of the ibc transaction
    #[prost(string, tag = "6")]
    #[serde(alias = "ibc_sequenceID")]
    pub ibc_sequence_id: ::prost::alloc::string::String,
    /// state of the unbonding during the process
    #[prost(enumeration = "unbonding::UnbondingState", tag = "7")]
    #[serde(
        serialize_with = "unbonding::unbonding_state_serde::serialize",
        deserialize_with = "unbonding::unbonding_state_serde::deserialize"
    )]
    pub state: i32,
}
/// Nested message and enum types in `Unbonding`.
pub mod unbonding {
    use persistence_std_derive::CosmwasmExt;
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    #[derive(::schemars::JsonSchema)]
    pub enum UnbondingState {
        /// no action has been initiated on the unbonding
        UnbondingPending = 0,
        /// unbonding action has been sent to the host chain
        UnbondingInitiated = 1,
        /// unbonding is waiting for the maturing period of the host chain
        UnbondingMaturing = 2,
        /// unbonding has matured and is ready to transfer from the host chain
        UnbondingMatured = 3,
        /// unbonding is on the persistence chain and can be claimed
        UnbondingClaimable = 4,
        /// unbonding has failed
        UnbondingFailed = 5,
    }
    pub mod unbonding_state_serde {
        use super::UnbondingState;
        use serde::{Deserialize, Deserializer, Serializer};
        pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
        where
            T: From<UnbondingState>,
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let enum_value = UnbondingState::from_str_name(&s).unwrap();
            let int_value: T = enum_value.into();
            return Ok(int_value);
        }
        pub fn serialize<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            let s: UnbondingState = UnbondingState::from_i32(*value).unwrap();
            serializer.serialize_str(s.as_str_name())
        }
    }
    impl UnbondingState {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                UnbondingState::UnbondingPending => "UNBONDING_PENDING",
                UnbondingState::UnbondingInitiated => "UNBONDING_INITIATED",
                UnbondingState::UnbondingMaturing => "UNBONDING_MATURING",
                UnbondingState::UnbondingMatured => "UNBONDING_MATURED",
                UnbondingState::UnbondingClaimable => "UNBONDING_CLAIMABLE",
                UnbondingState::UnbondingFailed => "UNBONDING_FAILED",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "UNBONDING_PENDING" => Some(Self::UnbondingPending),
                "UNBONDING_INITIATED" => Some(Self::UnbondingInitiated),
                "UNBONDING_MATURING" => Some(Self::UnbondingMaturing),
                "UNBONDING_MATURED" => Some(Self::UnbondingMatured),
                "UNBONDING_CLAIMABLE" => Some(Self::UnbondingClaimable),
                "UNBONDING_FAILED" => Some(Self::UnbondingFailed),
                _ => None,
            }
        }
    }
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.UserUnbonding")]
pub struct UserUnbonding {
    /// unbonding target chain
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// epoch when the unbonding started
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub epoch_number: i64,
    /// address which requested the unbonding
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
    /// stk token amount that is being unbonded
    #[prost(message, optional, tag = "4")]
    pub stk_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// host token amount that is being unbonded
    #[prost(message, optional, tag = "5")]
    pub unbond_amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.ValidatorUnbonding")]
pub struct ValidatorUnbonding {
    /// unbonding target chain
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    /// epoch when the unbonding started
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub epoch_number: i64,
    /// time when the unbonding matures and can be collected
    #[prost(message, optional, tag = "3")]
    pub mature_time: ::core::option::Option<crate::shim::Timestamp>,
    /// address of the validator that is being unbonded
    #[prost(string, tag = "4")]
    pub validator_address: ::prost::alloc::string::String,
    /// amount unbonded from the validator
    #[prost(message, optional, tag = "5")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
    /// sequence id of the ibc transaction
    #[prost(string, tag = "6")]
    #[serde(alias = "ibc_sequenceID")]
    pub ibc_sequence_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.KVUpdate")]
pub struct KvUpdate {
    #[prost(string, tag = "1")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub value: ::prost::alloc::string::String,
}
/// GenesisState defines the liquidstakeibc module's genesis state.
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.GenesisState")]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// initial host chain list
    #[prost(message, repeated, tag = "2")]
    pub host_chains: ::prost::alloc::vec::Vec<HostChain>,
    /// initial deposit list
    #[prost(message, repeated, tag = "3")]
    pub deposits: ::prost::alloc::vec::Vec<Deposit>,
    /// initial unbondings
    #[prost(message, repeated, tag = "4")]
    pub unbondings: ::prost::alloc::vec::Vec<Unbonding>,
    /// initial user unbondings
    #[prost(message, repeated, tag = "5")]
    pub user_unbondings: ::prost::alloc::vec::Vec<UserUnbonding>,
    /// validator unbondings
    #[prost(message, repeated, tag = "6")]
    pub validator_unbondings: ::prost::alloc::vec::Vec<ValidatorUnbonding>,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.MsgRegisterHostChain")]
pub struct MsgRegisterHostChain {
    /// authority is the address of the governance account
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "connectionID")]
    pub connection_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub deposit_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub restake_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub unstake_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub redemption_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    #[serde(alias = "channelID")]
    pub channel_id: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    #[serde(alias = "portID")]
    pub port_id: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub host_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub minimum_deposit: ::prost::alloc::string::String,
    #[prost(int64, tag = "11")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub unbonding_factor: i64,
    #[prost(int64, tag = "12")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub auto_compound_factor: i64,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.MsgRegisterHostChainResponse")]
pub struct MsgRegisterHostChainResponse {}
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.MsgUpdateHostChain")]
pub struct MsgUpdateHostChain {
    /// authority is the address of the governance account
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub updates: ::prost::alloc::vec::Vec<KvUpdate>,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.MsgUpdateHostChainResponse")]
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.MsgLiquidStake")]
pub struct MsgLiquidStake {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.MsgLiquidStakeResponse")]
pub struct MsgLiquidStakeResponse {}
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.MsgLiquidStakeLSM")]
pub struct MsgLiquidStakeLsm {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub delegations: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.MsgLiquidStakeLSMResponse")]
pub struct MsgLiquidStakeLsmResponse {}
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.MsgLiquidUnstake")]
pub struct MsgLiquidUnstake {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.MsgLiquidUnstakeResponse")]
pub struct MsgLiquidUnstakeResponse {}
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.MsgRedeem")]
pub struct MsgRedeem {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.MsgRedeemResponse")]
pub struct MsgRedeemResponse {}
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.MsgUpdateParams")]
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.MsgUpdateParamsResponse")]
pub struct MsgUpdateParamsResponse {}
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryParamsRequest")]
#[proto_query(
    path = "/pstake.liquidstakeibc.v1beta1.Query/Params",
    response_type = QueryParamsResponse
)]
pub struct QueryParamsRequest {}
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryParamsResponse")]
pub struct QueryParamsResponse {
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryHostChainRequest")]
#[proto_query(
    path = "/pstake.liquidstakeibc.v1beta1.Query/HostChain",
    response_type = QueryHostChainResponse
)]
pub struct QueryHostChainRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryHostChainResponse")]
pub struct QueryHostChainResponse {
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryHostChainsRequest")]
#[proto_query(
    path = "/pstake.liquidstakeibc.v1beta1.Query/HostChains",
    response_type = QueryHostChainsResponse
)]
pub struct QueryHostChainsRequest {}
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryHostChainsResponse")]
pub struct QueryHostChainsResponse {
    #[prost(message, repeated, tag = "1")]
    pub host_chains: ::prost::alloc::vec::Vec<HostChain>,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryDepositsRequest")]
#[proto_query(
    path = "/pstake.liquidstakeibc.v1beta1.Query/Deposits",
    response_type = QueryDepositsResponse
)]
pub struct QueryDepositsRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryDepositsResponse")]
pub struct QueryDepositsResponse {
    #[prost(message, repeated, tag = "1")]
    pub deposits: ::prost::alloc::vec::Vec<Deposit>,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryLSMDepositsRequest")]
#[proto_query(
    path = "/pstake.liquidstakeibc.v1beta1.Query/LSMDeposits",
    response_type = QueryLsmDepositsResponse
)]
pub struct QueryLsmDepositsRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryLSMDepositsResponse")]
pub struct QueryLsmDepositsResponse {
    #[prost(message, repeated, tag = "1")]
    pub deposits: ::prost::alloc::vec::Vec<LsmDeposit>,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryUnbondingsRequest")]
#[proto_query(
    path = "/pstake.liquidstakeibc.v1beta1.Query/Unbondings",
    response_type = QueryUnbondingsResponse
)]
pub struct QueryUnbondingsRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryUnbondingsResponse")]
pub struct QueryUnbondingsResponse {
    #[prost(message, repeated, tag = "1")]
    pub unbondings: ::prost::alloc::vec::Vec<Unbonding>,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryUnbondingRequest")]
#[proto_query(
    path = "/pstake.liquidstakeibc.v1beta1.Query/Unbonding",
    response_type = QueryUnbondingResponse
)]
pub struct QueryUnbondingRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    #[serde(
        serialize_with = "crate::serde::as_str::serialize",
        deserialize_with = "crate::serde::as_str::deserialize"
    )]
    pub epoch: i64,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryUnbondingResponse")]
pub struct QueryUnbondingResponse {
    #[prost(message, optional, tag = "1")]
    pub unbonding: ::core::option::Option<Unbonding>,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryUserUnbondingsRequest")]
#[proto_query(
    path = "/pstake.liquidstakeibc.v1beta1.Query/UserUnbondings",
    response_type = QueryUserUnbondingsResponse
)]
pub struct QueryUserUnbondingsRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryUserUnbondingsResponse")]
pub struct QueryUserUnbondingsResponse {
    #[prost(message, repeated, tag = "1")]
    pub user_unbondings: ::prost::alloc::vec::Vec<UserUnbonding>,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryValidatorUnbondingRequest")]
#[proto_query(
    path = "/pstake.liquidstakeibc.v1beta1.Query/ValidatorUnbondings",
    response_type = QueryValidatorUnbondingResponse
)]
pub struct QueryValidatorUnbondingRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryValidatorUnbondingResponse")]
pub struct QueryValidatorUnbondingResponse {
    #[prost(message, repeated, tag = "1")]
    pub validator_unbondings: ::prost::alloc::vec::Vec<ValidatorUnbonding>,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryDepositAccountBalanceRequest")]
#[proto_query(
    path = "/pstake.liquidstakeibc.v1beta1.Query/DepositAccountBalance",
    response_type = QueryDepositAccountBalanceResponse
)]
pub struct QueryDepositAccountBalanceRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryDepositAccountBalanceResponse")]
pub struct QueryDepositAccountBalanceResponse {
    #[prost(message, optional, tag = "1")]
    pub balance: ::core::option::Option<super::super::super::cosmos::base::v1beta1::Coin>,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryExchangeRateRequest")]
#[proto_query(
    path = "/pstake.liquidstakeibc.v1beta1.Query/ExchangeRate",
    response_type = QueryExchangeRateResponse
)]
pub struct QueryExchangeRateRequest {
    #[prost(string, tag = "1")]
    #[serde(alias = "chainID")]
    pub chain_id: ::prost::alloc::string::String,
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
#[proto_message(type_url = "/pstake.liquidstakeibc.v1beta1.QueryExchangeRateResponse")]
pub struct QueryExchangeRateResponse {
    #[prost(string, tag = "1")]
    pub rate: ::prost::alloc::string::String,
}
pub struct LiquidstakeibcQuerier<'a, Q: cosmwasm_std::CustomQuery> {
    querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>,
}
impl<'a, Q: cosmwasm_std::CustomQuery> LiquidstakeibcQuerier<'a, Q> {
    pub fn new(querier: &'a cosmwasm_std::QuerierWrapper<'a, Q>) -> Self {
        Self { querier }
    }
    pub fn params(&self) -> Result<QueryParamsResponse, cosmwasm_std::StdError> {
        QueryParamsRequest {}.query(self.querier)
    }
    pub fn host_chain(
        &self,
        chain_id: ::prost::alloc::string::String,
    ) -> Result<QueryHostChainResponse, cosmwasm_std::StdError> {
        QueryHostChainRequest { chain_id }.query(self.querier)
    }
    pub fn host_chains(&self) -> Result<QueryHostChainsResponse, cosmwasm_std::StdError> {
        QueryHostChainsRequest {}.query(self.querier)
    }
    pub fn deposits(
        &self,
        chain_id: ::prost::alloc::string::String,
    ) -> Result<QueryDepositsResponse, cosmwasm_std::StdError> {
        QueryDepositsRequest { chain_id }.query(self.querier)
    }
    pub fn lsm_deposits(
        &self,
        chain_id: ::prost::alloc::string::String,
    ) -> Result<QueryLsmDepositsResponse, cosmwasm_std::StdError> {
        QueryLsmDepositsRequest { chain_id }.query(self.querier)
    }
    pub fn unbondings(
        &self,
        chain_id: ::prost::alloc::string::String,
    ) -> Result<QueryUnbondingsResponse, cosmwasm_std::StdError> {
        QueryUnbondingsRequest { chain_id }.query(self.querier)
    }
    pub fn unbonding(
        &self,
        chain_id: ::prost::alloc::string::String,
        epoch: i64,
    ) -> Result<QueryUnbondingResponse, cosmwasm_std::StdError> {
        QueryUnbondingRequest { chain_id, epoch }.query(self.querier)
    }
    pub fn user_unbondings(
        &self,
        address: ::prost::alloc::string::String,
    ) -> Result<QueryUserUnbondingsResponse, cosmwasm_std::StdError> {
        QueryUserUnbondingsRequest { address }.query(self.querier)
    }
    pub fn validator_unbondings(
        &self,
        chain_id: ::prost::alloc::string::String,
    ) -> Result<QueryValidatorUnbondingResponse, cosmwasm_std::StdError> {
        QueryValidatorUnbondingRequest { chain_id }.query(self.querier)
    }
    pub fn deposit_account_balance(
        &self,
        chain_id: ::prost::alloc::string::String,
    ) -> Result<QueryDepositAccountBalanceResponse, cosmwasm_std::StdError> {
        QueryDepositAccountBalanceRequest { chain_id }.query(self.querier)
    }
    pub fn exchange_rate(
        &self,
        chain_id: ::prost::alloc::string::String,
    ) -> Result<QueryExchangeRateResponse, cosmwasm_std::StdError> {
        QueryExchangeRateRequest { chain_id }.query(self.querier)
    }
}
