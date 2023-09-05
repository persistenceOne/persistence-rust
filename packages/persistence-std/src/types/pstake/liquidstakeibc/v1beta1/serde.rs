impl serde::Serialize for Deposit {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_id.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        if self.epoch != 0 {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        if !self.ibc_sequence_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.liquidstakeibc.v1beta1.Deposit", len)?;
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        if self.epoch != 0 {
            struct_ser
                .serialize_field("epoch", ToString::to_string(&self.epoch).as_str())?;
        }
        if self.state != 0 {
            let v = deposit::DepositState::from_i32(self.state)
                .ok_or_else(|| serde::ser::Error::custom(
                    format!("Invalid variant {}", self.state),
                ))?;
            struct_ser.serialize_field("state", &v)?;
        }
        if !self.ibc_sequence_id.is_empty() {
            struct_ser.serialize_field("ibcSequenceId", &self.ibc_sequence_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Deposit {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "amount",
            "epoch",
            "state",
            "ibc_sequence_id",
            "ibcSequenceId",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            Amount,
            Epoch,
            State,
            IbcSequenceId,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "amount" => Ok(GeneratedField::Amount),
                            "epoch" => Ok(GeneratedField::Epoch),
                            "state" => Ok(GeneratedField::State),
                            "ibcSequenceId" | "ibc_sequence_id" => {
                                Ok(GeneratedField::IbcSequenceId)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Deposit;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.liquidstakeibc.v1beta1.Deposit")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Deposit, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut amount__ = None;
                let mut epoch__ = None;
                let mut state__ = None;
                let mut ibc_sequence_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map.next_value()?;
                        }
                        GeneratedField::Epoch => {
                            if epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            epoch__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(
                                map.next_value::<deposit::DepositState>()? as i32,
                            );
                        }
                        GeneratedField::IbcSequenceId => {
                            if ibc_sequence_id__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("ibcSequenceId"),
                                );
                            }
                            ibc_sequence_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Deposit {
                    chain_id: chain_id__.unwrap_or_default(),
                    amount: amount__,
                    epoch: epoch__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                    ibc_sequence_id: ibc_sequence_id__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.Deposit",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for deposit::DepositState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::DepositPending => "DEPOSIT_PENDING",
            Self::DepositSent => "DEPOSIT_SENT",
            Self::DepositReceived => "DEPOSIT_RECEIVED",
            Self::DepositDelegating => "DEPOSIT_DELEGATING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for deposit::DepositState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "DEPOSIT_PENDING",
            "DEPOSIT_SENT",
            "DEPOSIT_RECEIVED",
            "DEPOSIT_DELEGATING",
        ];
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = deposit::DepositState;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", & FIELDS)
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(deposit::DepositState::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(
                            serde::de::Unexpected::Signed(v),
                            &self,
                        )
                    })
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(deposit::DepositState::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(
                            serde::de::Unexpected::Unsigned(v),
                            &self,
                        )
                    })
            }
            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "DEPOSIT_PENDING" => Ok(deposit::DepositState::DepositPending),
                    "DEPOSIT_SENT" => Ok(deposit::DepositState::DepositSent),
                    "DEPOSIT_RECEIVED" => Ok(deposit::DepositState::DepositReceived),
                    "DEPOSIT_DELEGATING" => Ok(deposit::DepositState::DepositDelegating),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for GenesisState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.params.is_some() {
            len += 1;
        }
        if !self.host_chains.is_empty() {
            len += 1;
        }
        if !self.deposits.is_empty() {
            len += 1;
        }
        if !self.unbondings.is_empty() {
            len += 1;
        }
        if !self.user_unbondings.is_empty() {
            len += 1;
        }
        if !self.validator_unbondings.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.liquidstakeibc.v1beta1.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.host_chains.is_empty() {
            struct_ser.serialize_field("hostChains", &self.host_chains)?;
        }
        if !self.deposits.is_empty() {
            struct_ser.serialize_field("deposits", &self.deposits)?;
        }
        if !self.unbondings.is_empty() {
            struct_ser.serialize_field("unbondings", &self.unbondings)?;
        }
        if !self.user_unbondings.is_empty() {
            struct_ser.serialize_field("userUnbondings", &self.user_unbondings)?;
        }
        if !self.validator_unbondings.is_empty() {
            struct_ser
                .serialize_field("validatorUnbondings", &self.validator_unbondings)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GenesisState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "params",
            "host_chains",
            "hostChains",
            "deposits",
            "unbondings",
            "user_unbondings",
            "userUnbondings",
            "validator_unbondings",
            "validatorUnbondings",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            HostChains,
            Deposits,
            Unbondings,
            UserUnbondings,
            ValidatorUnbondings,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "params" => Ok(GeneratedField::Params),
                            "hostChains" | "host_chains" => {
                                Ok(GeneratedField::HostChains)
                            }
                            "deposits" => Ok(GeneratedField::Deposits),
                            "unbondings" => Ok(GeneratedField::Unbondings),
                            "userUnbondings" | "user_unbondings" => {
                                Ok(GeneratedField::UserUnbondings)
                            }
                            "validatorUnbondings" | "validator_unbondings" => {
                                Ok(GeneratedField::ValidatorUnbondings)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GenesisState;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.liquidstakeibc.v1beta1.GenesisState")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut host_chains__ = None;
                let mut deposits__ = None;
                let mut unbondings__ = None;
                let mut user_unbondings__ = None;
                let mut validator_unbondings__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                        GeneratedField::HostChains => {
                            if host_chains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostChains"));
                            }
                            host_chains__ = Some(map.next_value()?);
                        }
                        GeneratedField::Deposits => {
                            if deposits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deposits"));
                            }
                            deposits__ = Some(map.next_value()?);
                        }
                        GeneratedField::Unbondings => {
                            if unbondings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unbondings"));
                            }
                            unbondings__ = Some(map.next_value()?);
                        }
                        GeneratedField::UserUnbondings => {
                            if user_unbondings__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("userUnbondings"),
                                );
                            }
                            user_unbondings__ = Some(map.next_value()?);
                        }
                        GeneratedField::ValidatorUnbondings => {
                            if validator_unbondings__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorUnbondings"),
                                );
                            }
                            validator_unbondings__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    host_chains: host_chains__.unwrap_or_default(),
                    deposits: deposits__.unwrap_or_default(),
                    unbondings: unbondings__.unwrap_or_default(),
                    user_unbondings: user_unbondings__.unwrap_or_default(),
                    validator_unbondings: validator_unbondings__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.GenesisState",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for HostChain {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_id.is_empty() {
            len += 1;
        }
        if !self.connection_id.is_empty() {
            len += 1;
        }
        if self.params.is_some() {
            len += 1;
        }
        if !self.host_denom.is_empty() {
            len += 1;
        }
        if !self.channel_id.is_empty() {
            len += 1;
        }
        if !self.port_id.is_empty() {
            len += 1;
        }
        if self.delegation_account.is_some() {
            len += 1;
        }
        if self.rewards_account.is_some() {
            len += 1;
        }
        if !self.validators.is_empty() {
            len += 1;
        }
        if !self.minimum_deposit.is_empty() {
            len += 1;
        }
        if !self.c_value.is_empty() {
            len += 1;
        }
        if !self.last_c_value.is_empty() {
            len += 1;
        }
        if self.unbonding_factor != 0 {
            len += 1;
        }
        if self.active {
            len += 1;
        }
        if !self.auto_compound_factor.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.liquidstakeibc.v1beta1.HostChain", len)?;
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        if !self.connection_id.is_empty() {
            struct_ser.serialize_field("connectionId", &self.connection_id)?;
        }
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.host_denom.is_empty() {
            struct_ser.serialize_field("hostDenom", &self.host_denom)?;
        }
        if !self.channel_id.is_empty() {
            struct_ser.serialize_field("channelId", &self.channel_id)?;
        }
        if !self.port_id.is_empty() {
            struct_ser.serialize_field("portId", &self.port_id)?;
        }
        if let Some(v) = self.delegation_account.as_ref() {
            struct_ser.serialize_field("delegationAccount", v)?;
        }
        if let Some(v) = self.rewards_account.as_ref() {
            struct_ser.serialize_field("rewardsAccount", v)?;
        }
        if !self.validators.is_empty() {
            struct_ser.serialize_field("validators", &self.validators)?;
        }
        if !self.minimum_deposit.is_empty() {
            struct_ser.serialize_field("minimumDeposit", &self.minimum_deposit)?;
        }
        if !self.c_value.is_empty() {
            struct_ser.serialize_field("cValue", &self.c_value)?;
        }
        if !self.last_c_value.is_empty() {
            struct_ser.serialize_field("lastCValue", &self.last_c_value)?;
        }
        if self.unbonding_factor != 0 {
            struct_ser
                .serialize_field(
                    "unbondingFactor",
                    ToString::to_string(&self.unbonding_factor).as_str(),
                )?;
        }
        if self.active {
            struct_ser.serialize_field("active", &self.active)?;
        }
        if !self.auto_compound_factor.is_empty() {
            struct_ser
                .serialize_field("autoCompoundFactor", &self.auto_compound_factor)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostChain {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "connection_id",
            "connectionId",
            "params",
            "host_denom",
            "hostDenom",
            "channel_id",
            "channelId",
            "port_id",
            "portId",
            "delegation_account",
            "delegationAccount",
            "rewards_account",
            "rewardsAccount",
            "validators",
            "minimum_deposit",
            "minimumDeposit",
            "c_value",
            "cValue",
            "last_c_value",
            "lastCValue",
            "unbonding_factor",
            "unbondingFactor",
            "active",
            "auto_compound_factor",
            "autoCompoundFactor",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            ConnectionId,
            Params,
            HostDenom,
            ChannelId,
            PortId,
            DelegationAccount,
            RewardsAccount,
            Validators,
            MinimumDeposit,
            CValue,
            LastCValue,
            UnbondingFactor,
            Active,
            AutoCompoundFactor,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "connectionId" | "connection_id" => {
                                Ok(GeneratedField::ConnectionId)
                            }
                            "params" => Ok(GeneratedField::Params),
                            "hostDenom" | "host_denom" => Ok(GeneratedField::HostDenom),
                            "channelId" | "channel_id" => Ok(GeneratedField::ChannelId),
                            "portId" | "port_id" => Ok(GeneratedField::PortId),
                            "delegationAccount" | "delegation_account" => {
                                Ok(GeneratedField::DelegationAccount)
                            }
                            "rewardsAccount" | "rewards_account" => {
                                Ok(GeneratedField::RewardsAccount)
                            }
                            "validators" => Ok(GeneratedField::Validators),
                            "minimumDeposit" | "minimum_deposit" => {
                                Ok(GeneratedField::MinimumDeposit)
                            }
                            "cValue" | "c_value" => Ok(GeneratedField::CValue),
                            "lastCValue" | "last_c_value" => {
                                Ok(GeneratedField::LastCValue)
                            }
                            "unbondingFactor" | "unbonding_factor" => {
                                Ok(GeneratedField::UnbondingFactor)
                            }
                            "active" => Ok(GeneratedField::Active),
                            "autoCompoundFactor" | "auto_compound_factor" => {
                                Ok(GeneratedField::AutoCompoundFactor)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostChain;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.liquidstakeibc.v1beta1.HostChain")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<HostChain, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut connection_id__ = None;
                let mut params__ = None;
                let mut host_denom__ = None;
                let mut channel_id__ = None;
                let mut port_id__ = None;
                let mut delegation_account__ = None;
                let mut rewards_account__ = None;
                let mut validators__ = None;
                let mut minimum_deposit__ = None;
                let mut c_value__ = None;
                let mut last_c_value__ = None;
                let mut unbonding_factor__ = None;
                let mut active__ = None;
                let mut auto_compound_factor__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ConnectionId => {
                            if connection_id__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("connectionId"),
                                );
                            }
                            connection_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                        GeneratedField::HostDenom => {
                            if host_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostDenom"));
                            }
                            host_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChannelId => {
                            if channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelId"));
                            }
                            channel_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::PortId => {
                            if port_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("portId"));
                            }
                            port_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::DelegationAccount => {
                            if delegation_account__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("delegationAccount"),
                                );
                            }
                            delegation_account__ = map.next_value()?;
                        }
                        GeneratedField::RewardsAccount => {
                            if rewards_account__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("rewardsAccount"),
                                );
                            }
                            rewards_account__ = map.next_value()?;
                        }
                        GeneratedField::Validators => {
                            if validators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validators"));
                            }
                            validators__ = Some(map.next_value()?);
                        }
                        GeneratedField::MinimumDeposit => {
                            if minimum_deposit__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("minimumDeposit"),
                                );
                            }
                            minimum_deposit__ = Some(map.next_value()?);
                        }
                        GeneratedField::CValue => {
                            if c_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cValue"));
                            }
                            c_value__ = Some(map.next_value()?);
                        }
                        GeneratedField::LastCValue => {
                            if last_c_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastCValue"));
                            }
                            last_c_value__ = Some(map.next_value()?);
                        }
                        GeneratedField::UnbondingFactor => {
                            if unbonding_factor__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("unbondingFactor"),
                                );
                            }
                            unbonding_factor__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Active => {
                            if active__.is_some() {
                                return Err(serde::de::Error::duplicate_field("active"));
                            }
                            active__ = Some(map.next_value()?);
                        }
                        GeneratedField::AutoCompoundFactor => {
                            if auto_compound_factor__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("autoCompoundFactor"),
                                );
                            }
                            auto_compound_factor__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostChain {
                    chain_id: chain_id__.unwrap_or_default(),
                    connection_id: connection_id__.unwrap_or_default(),
                    params: params__,
                    host_denom: host_denom__.unwrap_or_default(),
                    channel_id: channel_id__.unwrap_or_default(),
                    port_id: port_id__.unwrap_or_default(),
                    delegation_account: delegation_account__,
                    rewards_account: rewards_account__,
                    validators: validators__.unwrap_or_default(),
                    minimum_deposit: minimum_deposit__.unwrap_or_default(),
                    c_value: c_value__.unwrap_or_default(),
                    last_c_value: last_c_value__.unwrap_or_default(),
                    unbonding_factor: unbonding_factor__.unwrap_or_default(),
                    active: active__.unwrap_or_default(),
                    auto_compound_factor: auto_compound_factor__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.HostChain",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for HostChainLsParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.deposit_fee.is_empty() {
            len += 1;
        }
        if !self.restake_fee.is_empty() {
            len += 1;
        }
        if !self.unstake_fee.is_empty() {
            len += 1;
        }
        if !self.redemption_fee.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.liquidstakeibc.v1beta1.HostChainLSParams", len)?;
        if !self.deposit_fee.is_empty() {
            struct_ser.serialize_field("depositFee", &self.deposit_fee)?;
        }
        if !self.restake_fee.is_empty() {
            struct_ser.serialize_field("restakeFee", &self.restake_fee)?;
        }
        if !self.unstake_fee.is_empty() {
            struct_ser.serialize_field("unstakeFee", &self.unstake_fee)?;
        }
        if !self.redemption_fee.is_empty() {
            struct_ser.serialize_field("redemptionFee", &self.redemption_fee)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostChainLsParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "deposit_fee",
            "depositFee",
            "restake_fee",
            "restakeFee",
            "unstake_fee",
            "unstakeFee",
            "redemption_fee",
            "redemptionFee",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DepositFee,
            RestakeFee,
            UnstakeFee,
            RedemptionFee,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "depositFee" | "deposit_fee" => {
                                Ok(GeneratedField::DepositFee)
                            }
                            "restakeFee" | "restake_fee" => {
                                Ok(GeneratedField::RestakeFee)
                            }
                            "unstakeFee" | "unstake_fee" => {
                                Ok(GeneratedField::UnstakeFee)
                            }
                            "redemptionFee" | "redemption_fee" => {
                                Ok(GeneratedField::RedemptionFee)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = HostChainLsParams;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.liquidstakeibc.v1beta1.HostChainLSParams")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<HostChainLsParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut deposit_fee__ = None;
                let mut restake_fee__ = None;
                let mut unstake_fee__ = None;
                let mut redemption_fee__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DepositFee => {
                            if deposit_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("depositFee"));
                            }
                            deposit_fee__ = Some(map.next_value()?);
                        }
                        GeneratedField::RestakeFee => {
                            if restake_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restakeFee"));
                            }
                            restake_fee__ = Some(map.next_value()?);
                        }
                        GeneratedField::UnstakeFee => {
                            if unstake_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unstakeFee"));
                            }
                            unstake_fee__ = Some(map.next_value()?);
                        }
                        GeneratedField::RedemptionFee => {
                            if redemption_fee__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("redemptionFee"),
                                );
                            }
                            redemption_fee__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostChainLsParams {
                    deposit_fee: deposit_fee__.unwrap_or_default(),
                    restake_fee: restake_fee__.unwrap_or_default(),
                    unstake_fee: unstake_fee__.unwrap_or_default(),
                    redemption_fee: redemption_fee__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.HostChainLSParams",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for IcaAccount {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        if self.balance.is_some() {
            len += 1;
        }
        if !self.owner.is_empty() {
            len += 1;
        }
        if self.channel_state != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.liquidstakeibc.v1beta1.ICAAccount", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if let Some(v) = self.balance.as_ref() {
            struct_ser.serialize_field("balance", v)?;
        }
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if self.channel_state != 0 {
            let v = ica_account::ChannelState::from_i32(self.channel_state)
                .ok_or_else(|| serde::ser::Error::custom(
                    format!("Invalid variant {}", self.channel_state),
                ))?;
            struct_ser.serialize_field("channelState", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IcaAccount {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "balance",
            "owner",
            "channel_state",
            "channelState",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Balance,
            Owner,
            ChannelState,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "address" => Ok(GeneratedField::Address),
                            "balance" => Ok(GeneratedField::Balance),
                            "owner" => Ok(GeneratedField::Owner),
                            "channelState" | "channel_state" => {
                                Ok(GeneratedField::ChannelState)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IcaAccount;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.liquidstakeibc.v1beta1.ICAAccount")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<IcaAccount, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut balance__ = None;
                let mut owner__ = None;
                let mut channel_state__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map.next_value()?);
                        }
                        GeneratedField::Balance => {
                            if balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balance"));
                            }
                            balance__ = map.next_value()?;
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChannelState => {
                            if channel_state__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("channelState"),
                                );
                            }
                            channel_state__ = Some(
                                map.next_value::<ica_account::ChannelState>()? as i32,
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(IcaAccount {
                    address: address__.unwrap_or_default(),
                    balance: balance__,
                    owner: owner__.unwrap_or_default(),
                    channel_state: channel_state__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.ICAAccount",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for ica_account::ChannelState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::IcaChannelCreating => "ICA_CHANNEL_CREATING",
            Self::IcaChannelCreated => "ICA_CHANNEL_CREATED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ica_account::ChannelState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["ICA_CHANNEL_CREATING", "ICA_CHANNEL_CREATED"];
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ica_account::ChannelState;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", & FIELDS)
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ica_account::ChannelState::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(
                            serde::de::Unexpected::Signed(v),
                            &self,
                        )
                    })
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(ica_account::ChannelState::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(
                            serde::de::Unexpected::Unsigned(v),
                            &self,
                        )
                    })
            }
            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "ICA_CHANNEL_CREATING" => {
                        Ok(ica_account::ChannelState::IcaChannelCreating)
                    }
                    "ICA_CHANNEL_CREATED" => {
                        Ok(ica_account::ChannelState::IcaChannelCreated)
                    }
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for KvUpdate {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.liquidstakeibc.v1beta1.KVUpdate", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.value.is_empty() {
            struct_ser.serialize_field("value", &self.value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for KvUpdate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["key", "value"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Value,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "key" => Ok(GeneratedField::Key),
                            "value" => Ok(GeneratedField::Value),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = KvUpdate;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.liquidstakeibc.v1beta1.KVUpdate")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<KvUpdate, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(KvUpdate {
                    key: key__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.KVUpdate",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgLiquidStake {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.delegator_address.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.liquidstakeibc.v1beta1.MsgLiquidStake", len)?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgLiquidStake {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["delegator_address", "delegatorAddress", "amount"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
            Amount,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "delegatorAddress" | "delegator_address" => {
                                Ok(GeneratedField::DelegatorAddress)
                            }
                            "amount" => Ok(GeneratedField::Amount),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgLiquidStake;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.liquidstakeibc.v1beta1.MsgLiquidStake")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgLiquidStake, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
                let mut amount__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DelegatorAddress => {
                            if delegator_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("delegatorAddress"),
                                );
                            }
                            delegator_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgLiquidStake {
                    delegator_address: delegator_address__.unwrap_or_default(),
                    amount: amount__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.MsgLiquidStake",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgLiquidStakeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.MsgLiquidStakeResponse",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgLiquidStakeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgLiquidStakeResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.MsgLiquidStakeResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgLiquidStakeResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgLiquidStakeResponse {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.MsgLiquidStakeResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgLiquidUnstake {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.delegator_address.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.liquidstakeibc.v1beta1.MsgLiquidUnstake", len)?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgLiquidUnstake {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["delegator_address", "delegatorAddress", "amount"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
            Amount,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "delegatorAddress" | "delegator_address" => {
                                Ok(GeneratedField::DelegatorAddress)
                            }
                            "amount" => Ok(GeneratedField::Amount),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgLiquidUnstake;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.liquidstakeibc.v1beta1.MsgLiquidUnstake")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgLiquidUnstake, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
                let mut amount__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DelegatorAddress => {
                            if delegator_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("delegatorAddress"),
                                );
                            }
                            delegator_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgLiquidUnstake {
                    delegator_address: delegator_address__.unwrap_or_default(),
                    amount: amount__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.MsgLiquidUnstake",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgLiquidUnstakeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.MsgLiquidUnstakeResponse",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgLiquidUnstakeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgLiquidUnstakeResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.MsgLiquidUnstakeResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgLiquidUnstakeResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgLiquidUnstakeResponse {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.MsgLiquidUnstakeResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgRedeem {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.delegator_address.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.liquidstakeibc.v1beta1.MsgRedeem", len)?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRedeem {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["delegator_address", "delegatorAddress", "amount"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
            Amount,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "delegatorAddress" | "delegator_address" => {
                                Ok(GeneratedField::DelegatorAddress)
                            }
                            "amount" => Ok(GeneratedField::Amount),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRedeem;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.liquidstakeibc.v1beta1.MsgRedeem")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgRedeem, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
                let mut amount__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DelegatorAddress => {
                            if delegator_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("delegatorAddress"),
                                );
                            }
                            delegator_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgRedeem {
                    delegator_address: delegator_address__.unwrap_or_default(),
                    amount: amount__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.MsgRedeem",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgRedeemResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("pstake.liquidstakeibc.v1beta1.MsgRedeemResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRedeemResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRedeemResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.liquidstakeibc.v1beta1.MsgRedeemResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgRedeemResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRedeemResponse {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.MsgRedeemResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgRegisterHostChain {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if !self.connection_id.is_empty() {
            len += 1;
        }
        if !self.deposit_fee.is_empty() {
            len += 1;
        }
        if !self.restake_fee.is_empty() {
            len += 1;
        }
        if !self.unstake_fee.is_empty() {
            len += 1;
        }
        if !self.redemption_fee.is_empty() {
            len += 1;
        }
        if !self.channel_id.is_empty() {
            len += 1;
        }
        if !self.port_id.is_empty() {
            len += 1;
        }
        if !self.host_denom.is_empty() {
            len += 1;
        }
        if !self.minimum_deposit.is_empty() {
            len += 1;
        }
        if self.unbonding_factor != 0 {
            len += 1;
        }
        if self.auto_compound_factor != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.MsgRegisterHostChain",
                len,
            )?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.connection_id.is_empty() {
            struct_ser.serialize_field("connectionId", &self.connection_id)?;
        }
        if !self.deposit_fee.is_empty() {
            struct_ser.serialize_field("depositFee", &self.deposit_fee)?;
        }
        if !self.restake_fee.is_empty() {
            struct_ser.serialize_field("restakeFee", &self.restake_fee)?;
        }
        if !self.unstake_fee.is_empty() {
            struct_ser.serialize_field("unstakeFee", &self.unstake_fee)?;
        }
        if !self.redemption_fee.is_empty() {
            struct_ser.serialize_field("redemptionFee", &self.redemption_fee)?;
        }
        if !self.channel_id.is_empty() {
            struct_ser.serialize_field("channelId", &self.channel_id)?;
        }
        if !self.port_id.is_empty() {
            struct_ser.serialize_field("portId", &self.port_id)?;
        }
        if !self.host_denom.is_empty() {
            struct_ser.serialize_field("hostDenom", &self.host_denom)?;
        }
        if !self.minimum_deposit.is_empty() {
            struct_ser.serialize_field("minimumDeposit", &self.minimum_deposit)?;
        }
        if self.unbonding_factor != 0 {
            struct_ser
                .serialize_field(
                    "unbondingFactor",
                    ToString::to_string(&self.unbonding_factor).as_str(),
                )?;
        }
        if self.auto_compound_factor != 0 {
            struct_ser
                .serialize_field(
                    "autoCompoundFactor",
                    ToString::to_string(&self.auto_compound_factor).as_str(),
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRegisterHostChain {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "authority",
            "connection_id",
            "connectionId",
            "deposit_fee",
            "depositFee",
            "restake_fee",
            "restakeFee",
            "unstake_fee",
            "unstakeFee",
            "redemption_fee",
            "redemptionFee",
            "channel_id",
            "channelId",
            "port_id",
            "portId",
            "host_denom",
            "hostDenom",
            "minimum_deposit",
            "minimumDeposit",
            "unbonding_factor",
            "unbondingFactor",
            "auto_compound_factor",
            "autoCompoundFactor",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            ConnectionId,
            DepositFee,
            RestakeFee,
            UnstakeFee,
            RedemptionFee,
            ChannelId,
            PortId,
            HostDenom,
            MinimumDeposit,
            UnbondingFactor,
            AutoCompoundFactor,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authority" => Ok(GeneratedField::Authority),
                            "connectionId" | "connection_id" => {
                                Ok(GeneratedField::ConnectionId)
                            }
                            "depositFee" | "deposit_fee" => {
                                Ok(GeneratedField::DepositFee)
                            }
                            "restakeFee" | "restake_fee" => {
                                Ok(GeneratedField::RestakeFee)
                            }
                            "unstakeFee" | "unstake_fee" => {
                                Ok(GeneratedField::UnstakeFee)
                            }
                            "redemptionFee" | "redemption_fee" => {
                                Ok(GeneratedField::RedemptionFee)
                            }
                            "channelId" | "channel_id" => Ok(GeneratedField::ChannelId),
                            "portId" | "port_id" => Ok(GeneratedField::PortId),
                            "hostDenom" | "host_denom" => Ok(GeneratedField::HostDenom),
                            "minimumDeposit" | "minimum_deposit" => {
                                Ok(GeneratedField::MinimumDeposit)
                            }
                            "unbondingFactor" | "unbonding_factor" => {
                                Ok(GeneratedField::UnbondingFactor)
                            }
                            "autoCompoundFactor" | "auto_compound_factor" => {
                                Ok(GeneratedField::AutoCompoundFactor)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRegisterHostChain;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.MsgRegisterHostChain",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgRegisterHostChain, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut connection_id__ = None;
                let mut deposit_fee__ = None;
                let mut restake_fee__ = None;
                let mut unstake_fee__ = None;
                let mut redemption_fee__ = None;
                let mut channel_id__ = None;
                let mut port_id__ = None;
                let mut host_denom__ = None;
                let mut minimum_deposit__ = None;
                let mut unbonding_factor__ = None;
                let mut auto_compound_factor__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map.next_value()?);
                        }
                        GeneratedField::ConnectionId => {
                            if connection_id__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("connectionId"),
                                );
                            }
                            connection_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::DepositFee => {
                            if deposit_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("depositFee"));
                            }
                            deposit_fee__ = Some(map.next_value()?);
                        }
                        GeneratedField::RestakeFee => {
                            if restake_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("restakeFee"));
                            }
                            restake_fee__ = Some(map.next_value()?);
                        }
                        GeneratedField::UnstakeFee => {
                            if unstake_fee__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unstakeFee"));
                            }
                            unstake_fee__ = Some(map.next_value()?);
                        }
                        GeneratedField::RedemptionFee => {
                            if redemption_fee__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("redemptionFee"),
                                );
                            }
                            redemption_fee__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChannelId => {
                            if channel_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("channelId"));
                            }
                            channel_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::PortId => {
                            if port_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("portId"));
                            }
                            port_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::HostDenom => {
                            if host_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostDenom"));
                            }
                            host_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::MinimumDeposit => {
                            if minimum_deposit__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("minimumDeposit"),
                                );
                            }
                            minimum_deposit__ = Some(map.next_value()?);
                        }
                        GeneratedField::UnbondingFactor => {
                            if unbonding_factor__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("unbondingFactor"),
                                );
                            }
                            unbonding_factor__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::AutoCompoundFactor => {
                            if auto_compound_factor__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("autoCompoundFactor"),
                                );
                            }
                            auto_compound_factor__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgRegisterHostChain {
                    authority: authority__.unwrap_or_default(),
                    connection_id: connection_id__.unwrap_or_default(),
                    deposit_fee: deposit_fee__.unwrap_or_default(),
                    restake_fee: restake_fee__.unwrap_or_default(),
                    unstake_fee: unstake_fee__.unwrap_or_default(),
                    redemption_fee: redemption_fee__.unwrap_or_default(),
                    channel_id: channel_id__.unwrap_or_default(),
                    port_id: port_id__.unwrap_or_default(),
                    host_denom: host_denom__.unwrap_or_default(),
                    minimum_deposit: minimum_deposit__.unwrap_or_default(),
                    unbonding_factor: unbonding_factor__.unwrap_or_default(),
                    auto_compound_factor: auto_compound_factor__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.MsgRegisterHostChain",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgRegisterHostChainResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.MsgRegisterHostChainResponse",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRegisterHostChainResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgRegisterHostChainResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.MsgRegisterHostChainResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgRegisterHostChainResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRegisterHostChainResponse {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.MsgRegisterHostChainResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgUpdateHostChain {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if !self.chain_id.is_empty() {
            len += 1;
        }
        if !self.updates.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.liquidstakeibc.v1beta1.MsgUpdateHostChain", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        if !self.updates.is_empty() {
            struct_ser.serialize_field("updates", &self.updates)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateHostChain {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "chain_id", "chainId", "updates"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            ChainId,
            Updates,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authority" => Ok(GeneratedField::Authority),
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "updates" => Ok(GeneratedField::Updates),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateHostChain;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.liquidstakeibc.v1beta1.MsgUpdateHostChain")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgUpdateHostChain, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut chain_id__ = None;
                let mut updates__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Updates => {
                            if updates__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updates"));
                            }
                            updates__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgUpdateHostChain {
                    authority: authority__.unwrap_or_default(),
                    chain_id: chain_id__.unwrap_or_default(),
                    updates: updates__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.MsgUpdateHostChain",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgUpdateHostChainResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.MsgUpdateHostChainResponse",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateHostChainResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateHostChainResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.MsgUpdateHostChainResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgUpdateHostChainResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateHostChainResponse {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.MsgUpdateHostChainResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgUpdateParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.authority.is_empty() {
            len += 1;
        }
        if self.params.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.liquidstakeibc.v1beta1.MsgUpdateParams", len)?;
        if !self.authority.is_empty() {
            struct_ser.serialize_field("authority", &self.authority)?;
        }
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["authority", "params"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Authority,
            Params,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "authority" => Ok(GeneratedField::Authority),
                            "params" => Ok(GeneratedField::Params),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateParams;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.liquidstakeibc.v1beta1.MsgUpdateParams")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgUpdateParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut authority__ = None;
                let mut params__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Authority => {
                            if authority__.is_some() {
                                return Err(serde::de::Error::duplicate_field("authority"));
                            }
                            authority__ = Some(map.next_value()?);
                        }
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgUpdateParams {
                    authority: authority__.unwrap_or_default(),
                    params: params__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.MsgUpdateParams",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgUpdateParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.MsgUpdateParamsResponse",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUpdateParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgUpdateParamsResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.MsgUpdateParamsResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgUpdateParamsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUpdateParamsResponse {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.MsgUpdateParamsResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for Params {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.admin_address.is_empty() {
            len += 1;
        }
        if !self.fee_address.is_empty() {
            len += 1;
        }
        if !self.upper_c_value_limit.is_empty() {
            len += 1;
        }
        if !self.lower_c_value_limit.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.liquidstakeibc.v1beta1.Params", len)?;
        if !self.admin_address.is_empty() {
            struct_ser.serialize_field("adminAddress", &self.admin_address)?;
        }
        if !self.fee_address.is_empty() {
            struct_ser.serialize_field("feeAddress", &self.fee_address)?;
        }
        if !self.upper_c_value_limit.is_empty() {
            struct_ser.serialize_field("upperCValueLimit", &self.upper_c_value_limit)?;
        }
        if !self.lower_c_value_limit.is_empty() {
            struct_ser.serialize_field("lowerCValueLimit", &self.lower_c_value_limit)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Params {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "admin_address",
            "adminAddress",
            "fee_address",
            "feeAddress",
            "upper_c_value_limit",
            "upperCValueLimit",
            "lower_c_value_limit",
            "lowerCValueLimit",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AdminAddress,
            FeeAddress,
            UpperCValueLimit,
            LowerCValueLimit,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "adminAddress" | "admin_address" => {
                                Ok(GeneratedField::AdminAddress)
                            }
                            "feeAddress" | "fee_address" => {
                                Ok(GeneratedField::FeeAddress)
                            }
                            "upperCValueLimit" | "upper_c_value_limit" => {
                                Ok(GeneratedField::UpperCValueLimit)
                            }
                            "lowerCValueLimit" | "lower_c_value_limit" => {
                                Ok(GeneratedField::LowerCValueLimit)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Params;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.liquidstakeibc.v1beta1.Params")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut admin_address__ = None;
                let mut fee_address__ = None;
                let mut upper_c_value_limit__ = None;
                let mut lower_c_value_limit__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AdminAddress => {
                            if admin_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("adminAddress"),
                                );
                            }
                            admin_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::FeeAddress => {
                            if fee_address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feeAddress"));
                            }
                            fee_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::UpperCValueLimit => {
                            if upper_c_value_limit__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("upperCValueLimit"),
                                );
                            }
                            upper_c_value_limit__ = Some(map.next_value()?);
                        }
                        GeneratedField::LowerCValueLimit => {
                            if lower_c_value_limit__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("lowerCValueLimit"),
                                );
                            }
                            lower_c_value_limit__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Params {
                    admin_address: admin_address__.unwrap_or_default(),
                    fee_address: fee_address__.unwrap_or_default(),
                    upper_c_value_limit: upper_c_value_limit__.unwrap_or_default(),
                    lower_c_value_limit: lower_c_value_limit__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.Params",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryDepositAccountBalanceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryDepositAccountBalanceRequest",
                len,
            )?;
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryDepositAccountBalanceRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["chain_id", "chainId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDepositAccountBalanceRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.QueryDepositAccountBalanceRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryDepositAccountBalanceRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryDepositAccountBalanceRequest {
                    chain_id: chain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryDepositAccountBalanceRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryDepositAccountBalanceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.balance.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryDepositAccountBalanceResponse",
                len,
            )?;
        if let Some(v) = self.balance.as_ref() {
            struct_ser.serialize_field("balance", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryDepositAccountBalanceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["balance"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Balance,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "balance" => Ok(GeneratedField::Balance),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDepositAccountBalanceResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.QueryDepositAccountBalanceResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryDepositAccountBalanceResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut balance__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Balance => {
                            if balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balance"));
                            }
                            balance__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryDepositAccountBalanceResponse {
                    balance: balance__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryDepositAccountBalanceResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryDepositsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryDepositsRequest",
                len,
            )?;
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryDepositsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["chain_id", "chainId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDepositsRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.QueryDepositsRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryDepositsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryDepositsRequest {
                    chain_id: chain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryDepositsRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryDepositsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.deposits.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryDepositsResponse",
                len,
            )?;
        if !self.deposits.is_empty() {
            struct_ser.serialize_field("deposits", &self.deposits)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryDepositsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["deposits"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Deposits,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "deposits" => Ok(GeneratedField::Deposits),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryDepositsResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.QueryDepositsResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryDepositsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut deposits__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Deposits => {
                            if deposits__.is_some() {
                                return Err(serde::de::Error::duplicate_field("deposits"));
                            }
                            deposits__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryDepositsResponse {
                    deposits: deposits__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryDepositsResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryExchangeRateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryExchangeRateRequest",
                len,
            )?;
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryExchangeRateRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["chain_id", "chainId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryExchangeRateRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.QueryExchangeRateRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryExchangeRateRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryExchangeRateRequest {
                    chain_id: chain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryExchangeRateRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryExchangeRateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.rate.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryExchangeRateResponse",
                len,
            )?;
        if !self.rate.is_empty() {
            struct_ser.serialize_field("rate", &self.rate)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryExchangeRateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["rate"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rate,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "rate" => Ok(GeneratedField::Rate),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryExchangeRateResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.QueryExchangeRateResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryExchangeRateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut rate__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Rate => {
                            if rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rate"));
                            }
                            rate__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryExchangeRateResponse {
                    rate: rate__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryExchangeRateResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryHostChainRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryHostChainRequest",
                len,
            )?;
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryHostChainRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["chain_id", "chainId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryHostChainRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.QueryHostChainRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryHostChainRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryHostChainRequest {
                    chain_id: chain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryHostChainRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryHostChainResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.host_chain.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryHostChainResponse",
                len,
            )?;
        if let Some(v) = self.host_chain.as_ref() {
            struct_ser.serialize_field("hostChain", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryHostChainResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["host_chain", "hostChain"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HostChain,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "hostChain" | "host_chain" => Ok(GeneratedField::HostChain),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryHostChainResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.QueryHostChainResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryHostChainResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut host_chain__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HostChain => {
                            if host_chain__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostChain"));
                            }
                            host_chain__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryHostChainResponse {
                    host_chain: host_chain__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryHostChainResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryHostChainsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryHostChainsRequest",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryHostChainsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryHostChainsRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.QueryHostChainsRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryHostChainsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryHostChainsRequest {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryHostChainsRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryHostChainsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.host_chains.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryHostChainsResponse",
                len,
            )?;
        if !self.host_chains.is_empty() {
            struct_ser.serialize_field("hostChains", &self.host_chains)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryHostChainsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["host_chains", "hostChains"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HostChains,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "hostChains" | "host_chains" => {
                                Ok(GeneratedField::HostChains)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryHostChainsResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.QueryHostChainsResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryHostChainsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut host_chains__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HostChains => {
                            if host_chains__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hostChains"));
                            }
                            host_chains__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryHostChainsResponse {
                    host_chains: host_chains__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryHostChainsResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryParamsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("pstake.liquidstakeibc.v1beta1.QueryParamsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryParamsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        Ok(GeneratedField::__SkipField__)
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParamsRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.liquidstakeibc.v1beta1.QueryParamsRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryParamsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryParamsRequest {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryParamsRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.params.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.liquidstakeibc.v1beta1.QueryParamsResponse", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["params"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "params" => Ok(GeneratedField::Params),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryParamsResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.QueryParamsResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryParamsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryParamsResponse {
                    params: params__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryParamsResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryUnbondingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_id.is_empty() {
            len += 1;
        }
        if self.epoch != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryUnbondingRequest",
                len,
            )?;
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        if self.epoch != 0 {
            struct_ser
                .serialize_field("epoch", ToString::to_string(&self.epoch).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryUnbondingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["chain_id", "chainId", "epoch"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            Epoch,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "epoch" => Ok(GeneratedField::Epoch),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryUnbondingRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.QueryUnbondingRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryUnbondingRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut epoch__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Epoch => {
                            if epoch__.is_some() {
                                return Err(serde::de::Error::duplicate_field("epoch"));
                            }
                            epoch__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryUnbondingRequest {
                    chain_id: chain_id__.unwrap_or_default(),
                    epoch: epoch__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryUnbondingRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryUnbondingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.unbonding.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryUnbondingResponse",
                len,
            )?;
        if let Some(v) = self.unbonding.as_ref() {
            struct_ser.serialize_field("unbonding", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryUnbondingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["unbonding"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Unbonding,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "unbonding" => Ok(GeneratedField::Unbonding),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryUnbondingResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.QueryUnbondingResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryUnbondingResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut unbonding__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Unbonding => {
                            if unbonding__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unbonding"));
                            }
                            unbonding__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryUnbondingResponse {
                    unbonding: unbonding__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryUnbondingResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryUnbondingsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryUnbondingsRequest",
                len,
            )?;
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryUnbondingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["chain_id", "chainId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryUnbondingsRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.QueryUnbondingsRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryUnbondingsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryUnbondingsRequest {
                    chain_id: chain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryUnbondingsRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryUnbondingsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.unbondings.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryUnbondingsResponse",
                len,
            )?;
        if !self.unbondings.is_empty() {
            struct_ser.serialize_field("unbondings", &self.unbondings)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryUnbondingsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["unbondings"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Unbondings,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "unbondings" => Ok(GeneratedField::Unbondings),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryUnbondingsResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.QueryUnbondingsResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryUnbondingsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut unbondings__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Unbondings => {
                            if unbondings__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unbondings"));
                            }
                            unbondings__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryUnbondingsResponse {
                    unbondings: unbondings__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryUnbondingsResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryUserUnbondingsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryUserUnbondingsRequest",
                len,
            )?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryUserUnbondingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "address" => Ok(GeneratedField::Address),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryUserUnbondingsRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.QueryUserUnbondingsRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryUserUnbondingsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryUserUnbondingsRequest {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryUserUnbondingsRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryUserUnbondingsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.user_unbondings.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryUserUnbondingsResponse",
                len,
            )?;
        if !self.user_unbondings.is_empty() {
            struct_ser.serialize_field("userUnbondings", &self.user_unbondings)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryUserUnbondingsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["user_unbondings", "userUnbondings"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UserUnbondings,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "userUnbondings" | "user_unbondings" => {
                                Ok(GeneratedField::UserUnbondings)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryUserUnbondingsResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.QueryUserUnbondingsResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryUserUnbondingsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut user_unbondings__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UserUnbondings => {
                            if user_unbondings__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("userUnbondings"),
                                );
                            }
                            user_unbondings__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryUserUnbondingsResponse {
                    user_unbondings: user_unbondings__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryUserUnbondingsResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryValidatorUnbondingRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryValidatorUnbondingRequest",
                len,
            )?;
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryValidatorUnbondingRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["chain_id", "chainId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryValidatorUnbondingRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.QueryValidatorUnbondingRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryValidatorUnbondingRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryValidatorUnbondingRequest {
                    chain_id: chain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryValidatorUnbondingRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryValidatorUnbondingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.validator_unbondings.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryValidatorUnbondingResponse",
                len,
            )?;
        if !self.validator_unbondings.is_empty() {
            struct_ser
                .serialize_field("validatorUnbondings", &self.validator_unbondings)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryValidatorUnbondingResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["validator_unbondings", "validatorUnbondings"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidatorUnbondings,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "validatorUnbondings" | "validator_unbondings" => {
                                Ok(GeneratedField::ValidatorUnbondings)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryValidatorUnbondingResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.liquidstakeibc.v1beta1.QueryValidatorUnbondingResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryValidatorUnbondingResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut validator_unbondings__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ValidatorUnbondings => {
                            if validator_unbondings__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorUnbondings"),
                                );
                            }
                            validator_unbondings__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryValidatorUnbondingResponse {
                    validator_unbondings: validator_unbondings__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.QueryValidatorUnbondingResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for Unbonding {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_id.is_empty() {
            len += 1;
        }
        if self.epoch_number != 0 {
            len += 1;
        }
        if self.mature_time.is_some() {
            len += 1;
        }
        if self.burn_amount.is_some() {
            len += 1;
        }
        if self.unbond_amount.is_some() {
            len += 1;
        }
        if !self.ibc_sequence_id.is_empty() {
            len += 1;
        }
        if self.state != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.liquidstakeibc.v1beta1.Unbonding", len)?;
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        if self.epoch_number != 0 {
            struct_ser
                .serialize_field(
                    "epochNumber",
                    ToString::to_string(&self.epoch_number).as_str(),
                )?;
        }
        if let Some(v) = self.mature_time.as_ref() {
            struct_ser.serialize_field("matureTime", v)?;
        }
        if let Some(v) = self.burn_amount.as_ref() {
            struct_ser.serialize_field("burnAmount", v)?;
        }
        if let Some(v) = self.unbond_amount.as_ref() {
            struct_ser.serialize_field("unbondAmount", v)?;
        }
        if !self.ibc_sequence_id.is_empty() {
            struct_ser.serialize_field("ibcSequenceId", &self.ibc_sequence_id)?;
        }
        if self.state != 0 {
            let v = unbonding::UnbondingState::from_i32(self.state)
                .ok_or_else(|| serde::ser::Error::custom(
                    format!("Invalid variant {}", self.state),
                ))?;
            struct_ser.serialize_field("state", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Unbonding {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "epoch_number",
            "epochNumber",
            "mature_time",
            "matureTime",
            "burn_amount",
            "burnAmount",
            "unbond_amount",
            "unbondAmount",
            "ibc_sequence_id",
            "ibcSequenceId",
            "state",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            EpochNumber,
            MatureTime,
            BurnAmount,
            UnbondAmount,
            IbcSequenceId,
            State,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "epochNumber" | "epoch_number" => {
                                Ok(GeneratedField::EpochNumber)
                            }
                            "matureTime" | "mature_time" => {
                                Ok(GeneratedField::MatureTime)
                            }
                            "burnAmount" | "burn_amount" => {
                                Ok(GeneratedField::BurnAmount)
                            }
                            "unbondAmount" | "unbond_amount" => {
                                Ok(GeneratedField::UnbondAmount)
                            }
                            "ibcSequenceId" | "ibc_sequence_id" => {
                                Ok(GeneratedField::IbcSequenceId)
                            }
                            "state" => Ok(GeneratedField::State),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Unbonding;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.liquidstakeibc.v1beta1.Unbonding")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Unbonding, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut epoch_number__ = None;
                let mut mature_time__ = None;
                let mut burn_amount__ = None;
                let mut unbond_amount__ = None;
                let mut ibc_sequence_id__ = None;
                let mut state__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::EpochNumber => {
                            if epoch_number__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("epochNumber"),
                                );
                            }
                            epoch_number__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MatureTime => {
                            if mature_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matureTime"));
                            }
                            mature_time__ = map.next_value()?;
                        }
                        GeneratedField::BurnAmount => {
                            if burn_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("burnAmount"));
                            }
                            burn_amount__ = map.next_value()?;
                        }
                        GeneratedField::UnbondAmount => {
                            if unbond_amount__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("unbondAmount"),
                                );
                            }
                            unbond_amount__ = map.next_value()?;
                        }
                        GeneratedField::IbcSequenceId => {
                            if ibc_sequence_id__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("ibcSequenceId"),
                                );
                            }
                            ibc_sequence_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::State => {
                            if state__.is_some() {
                                return Err(serde::de::Error::duplicate_field("state"));
                            }
                            state__ = Some(
                                map.next_value::<unbonding::UnbondingState>()? as i32,
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Unbonding {
                    chain_id: chain_id__.unwrap_or_default(),
                    epoch_number: epoch_number__.unwrap_or_default(),
                    mature_time: mature_time__,
                    burn_amount: burn_amount__,
                    unbond_amount: unbond_amount__,
                    ibc_sequence_id: ibc_sequence_id__.unwrap_or_default(),
                    state: state__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.Unbonding",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for unbonding::UnbondingState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::UnbondingPending => "UNBONDING_PENDING",
            Self::UnbondingInitiated => "UNBONDING_INITIATED",
            Self::UnbondingMaturing => "UNBONDING_MATURING",
            Self::UnbondingMatured => "UNBONDING_MATURED",
            Self::UnbondingClaimable => "UNBONDING_CLAIMABLE",
            Self::UnbondingFailed => "UNBONDING_FAILED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for unbonding::UnbondingState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "UNBONDING_PENDING",
            "UNBONDING_INITIATED",
            "UNBONDING_MATURING",
            "UNBONDING_MATURED",
            "UNBONDING_CLAIMABLE",
            "UNBONDING_FAILED",
        ];
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = unbonding::UnbondingState;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", & FIELDS)
            }
            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(unbonding::UnbondingState::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(
                            serde::de::Unexpected::Signed(v),
                            &self,
                        )
                    })
            }
            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(unbonding::UnbondingState::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(
                            serde::de::Unexpected::Unsigned(v),
                            &self,
                        )
                    })
            }
            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "UNBONDING_PENDING" => {
                        Ok(unbonding::UnbondingState::UnbondingPending)
                    }
                    "UNBONDING_INITIATED" => {
                        Ok(unbonding::UnbondingState::UnbondingInitiated)
                    }
                    "UNBONDING_MATURING" => {
                        Ok(unbonding::UnbondingState::UnbondingMaturing)
                    }
                    "UNBONDING_MATURED" => {
                        Ok(unbonding::UnbondingState::UnbondingMatured)
                    }
                    "UNBONDING_CLAIMABLE" => {
                        Ok(unbonding::UnbondingState::UnbondingClaimable)
                    }
                    "UNBONDING_FAILED" => Ok(unbonding::UnbondingState::UnbondingFailed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for UserUnbonding {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_id.is_empty() {
            len += 1;
        }
        if self.epoch_number != 0 {
            len += 1;
        }
        if !self.address.is_empty() {
            len += 1;
        }
        if self.stk_amount.is_some() {
            len += 1;
        }
        if self.unbond_amount.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.liquidstakeibc.v1beta1.UserUnbonding", len)?;
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        if self.epoch_number != 0 {
            struct_ser
                .serialize_field(
                    "epochNumber",
                    ToString::to_string(&self.epoch_number).as_str(),
                )?;
        }
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if let Some(v) = self.stk_amount.as_ref() {
            struct_ser.serialize_field("stkAmount", v)?;
        }
        if let Some(v) = self.unbond_amount.as_ref() {
            struct_ser.serialize_field("unbondAmount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserUnbonding {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "epoch_number",
            "epochNumber",
            "address",
            "stk_amount",
            "stkAmount",
            "unbond_amount",
            "unbondAmount",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            EpochNumber,
            Address,
            StkAmount,
            UnbondAmount,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "epochNumber" | "epoch_number" => {
                                Ok(GeneratedField::EpochNumber)
                            }
                            "address" => Ok(GeneratedField::Address),
                            "stkAmount" | "stk_amount" => Ok(GeneratedField::StkAmount),
                            "unbondAmount" | "unbond_amount" => {
                                Ok(GeneratedField::UnbondAmount)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserUnbonding;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.liquidstakeibc.v1beta1.UserUnbonding")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<UserUnbonding, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut epoch_number__ = None;
                let mut address__ = None;
                let mut stk_amount__ = None;
                let mut unbond_amount__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::EpochNumber => {
                            if epoch_number__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("epochNumber"),
                                );
                            }
                            epoch_number__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map.next_value()?);
                        }
                        GeneratedField::StkAmount => {
                            if stk_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("stkAmount"));
                            }
                            stk_amount__ = map.next_value()?;
                        }
                        GeneratedField::UnbondAmount => {
                            if unbond_amount__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("unbondAmount"),
                                );
                            }
                            unbond_amount__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UserUnbonding {
                    chain_id: chain_id__.unwrap_or_default(),
                    epoch_number: epoch_number__.unwrap_or_default(),
                    address: address__.unwrap_or_default(),
                    stk_amount: stk_amount__,
                    unbond_amount: unbond_amount__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.UserUnbonding",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for Validator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.operator_address.is_empty() {
            len += 1;
        }
        if !self.status.is_empty() {
            len += 1;
        }
        if !self.weight.is_empty() {
            len += 1;
        }
        if !self.delegated_amount.is_empty() {
            len += 1;
        }
        if !self.exchange_rate.is_empty() {
            len += 1;
        }
        if self.unbonding_epoch != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.liquidstakeibc.v1beta1.Validator", len)?;
        if !self.operator_address.is_empty() {
            struct_ser.serialize_field("operatorAddress", &self.operator_address)?;
        }
        if !self.status.is_empty() {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if !self.weight.is_empty() {
            struct_ser.serialize_field("weight", &self.weight)?;
        }
        if !self.delegated_amount.is_empty() {
            struct_ser.serialize_field("delegatedAmount", &self.delegated_amount)?;
        }
        if !self.exchange_rate.is_empty() {
            struct_ser.serialize_field("exchangeRate", &self.exchange_rate)?;
        }
        if self.unbonding_epoch != 0 {
            struct_ser
                .serialize_field(
                    "unbondingEpoch",
                    ToString::to_string(&self.unbonding_epoch).as_str(),
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Validator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operator_address",
            "operatorAddress",
            "status",
            "weight",
            "delegated_amount",
            "delegatedAmount",
            "exchange_rate",
            "exchangeRate",
            "unbonding_epoch",
            "unbondingEpoch",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OperatorAddress,
            Status,
            Weight,
            DelegatedAmount,
            ExchangeRate,
            UnbondingEpoch,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "operatorAddress" | "operator_address" => {
                                Ok(GeneratedField::OperatorAddress)
                            }
                            "status" => Ok(GeneratedField::Status),
                            "weight" => Ok(GeneratedField::Weight),
                            "delegatedAmount" | "delegated_amount" => {
                                Ok(GeneratedField::DelegatedAmount)
                            }
                            "exchangeRate" | "exchange_rate" => {
                                Ok(GeneratedField::ExchangeRate)
                            }
                            "unbondingEpoch" | "unbonding_epoch" => {
                                Ok(GeneratedField::UnbondingEpoch)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Validator;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.liquidstakeibc.v1beta1.Validator")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Validator, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut operator_address__ = None;
                let mut status__ = None;
                let mut weight__ = None;
                let mut delegated_amount__ = None;
                let mut exchange_rate__ = None;
                let mut unbonding_epoch__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OperatorAddress => {
                            if operator_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("operatorAddress"),
                                );
                            }
                            operator_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map.next_value()?);
                        }
                        GeneratedField::Weight => {
                            if weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            weight__ = Some(map.next_value()?);
                        }
                        GeneratedField::DelegatedAmount => {
                            if delegated_amount__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("delegatedAmount"),
                                );
                            }
                            delegated_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExchangeRate => {
                            if exchange_rate__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("exchangeRate"),
                                );
                            }
                            exchange_rate__ = Some(map.next_value()?);
                        }
                        GeneratedField::UnbondingEpoch => {
                            if unbonding_epoch__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("unbondingEpoch"),
                                );
                            }
                            unbonding_epoch__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Validator {
                    operator_address: operator_address__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    weight: weight__.unwrap_or_default(),
                    delegated_amount: delegated_amount__.unwrap_or_default(),
                    exchange_rate: exchange_rate__.unwrap_or_default(),
                    unbonding_epoch: unbonding_epoch__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.Validator",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for ValidatorUnbonding {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_id.is_empty() {
            len += 1;
        }
        if self.epoch_number != 0 {
            len += 1;
        }
        if self.mature_time.is_some() {
            len += 1;
        }
        if !self.validator_address.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        if !self.ibc_sequence_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.liquidstakeibc.v1beta1.ValidatorUnbonding", len)?;
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        if self.epoch_number != 0 {
            struct_ser
                .serialize_field(
                    "epochNumber",
                    ToString::to_string(&self.epoch_number).as_str(),
                )?;
        }
        if let Some(v) = self.mature_time.as_ref() {
            struct_ser.serialize_field("matureTime", v)?;
        }
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        if !self.ibc_sequence_id.is_empty() {
            struct_ser.serialize_field("ibcSequenceId", &self.ibc_sequence_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValidatorUnbonding {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_id",
            "chainId",
            "epoch_number",
            "epochNumber",
            "mature_time",
            "matureTime",
            "validator_address",
            "validatorAddress",
            "amount",
            "ibc_sequence_id",
            "ibcSequenceId",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainId,
            EpochNumber,
            MatureTime,
            ValidatorAddress,
            Amount,
            IbcSequenceId,
            __SkipField__,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(
                deserializer: D,
            ) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;
                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;
                    fn expecting(
                        &self,
                        formatter: &mut std::fmt::Formatter<'_>,
                    ) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", & FIELDS)
                    }
                    #[allow(unused_variables)]
                    fn visit_str<E>(
                        self,
                        value: &str,
                    ) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "epochNumber" | "epoch_number" => {
                                Ok(GeneratedField::EpochNumber)
                            }
                            "matureTime" | "mature_time" => {
                                Ok(GeneratedField::MatureTime)
                            }
                            "validatorAddress" | "validator_address" => {
                                Ok(GeneratedField::ValidatorAddress)
                            }
                            "amount" => Ok(GeneratedField::Amount),
                            "ibcSequenceId" | "ibc_sequence_id" => {
                                Ok(GeneratedField::IbcSequenceId)
                            }
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidatorUnbonding;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.liquidstakeibc.v1beta1.ValidatorUnbonding")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<ValidatorUnbonding, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut chain_id__ = None;
                let mut epoch_number__ = None;
                let mut mature_time__ = None;
                let mut validator_address__ = None;
                let mut amount__ = None;
                let mut ibc_sequence_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::EpochNumber => {
                            if epoch_number__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("epochNumber"),
                                );
                            }
                            epoch_number__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MatureTime => {
                            if mature_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("matureTime"));
                            }
                            mature_time__ = map.next_value()?;
                        }
                        GeneratedField::ValidatorAddress => {
                            if validator_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorAddress"),
                                );
                            }
                            validator_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::Amount => {
                            if amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("amount"));
                            }
                            amount__ = map.next_value()?;
                        }
                        GeneratedField::IbcSequenceId => {
                            if ibc_sequence_id__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("ibcSequenceId"),
                                );
                            }
                            ibc_sequence_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ValidatorUnbonding {
                    chain_id: chain_id__.unwrap_or_default(),
                    epoch_number: epoch_number__.unwrap_or_default(),
                    mature_time: mature_time__,
                    validator_address: validator_address__.unwrap_or_default(),
                    amount: amount__,
                    ibc_sequence_id: ibc_sequence_id__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.liquidstakeibc.v1beta1.ValidatorUnbonding",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
