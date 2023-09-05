impl serde::Serialize for AllowListedValidator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.validator_address.is_empty() {
            len += 1;
        }
        if !self.target_weight.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.AllowListedValidator", len)?;
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        if !self.target_weight.is_empty() {
            struct_ser.serialize_field("targetWeight", &self.target_weight)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AllowListedValidator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "validator_address",
            "validatorAddress",
            "target_weight",
            "targetWeight",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidatorAddress,
            TargetWeight,
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
                            "validatorAddress" | "validator_address" => {
                                Ok(GeneratedField::ValidatorAddress)
                            }
                            "targetWeight" | "target_weight" => {
                                Ok(GeneratedField::TargetWeight)
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
            type Value = AllowListedValidator;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.lscosmos.v1beta1.AllowListedValidator")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<AllowListedValidator, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut validator_address__ = None;
                let mut target_weight__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ValidatorAddress => {
                            if validator_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorAddress"),
                                );
                            }
                            validator_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::TargetWeight => {
                            if target_weight__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("targetWeight"),
                                );
                            }
                            target_weight__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AllowListedValidator {
                    validator_address: validator_address__.unwrap_or_default(),
                    target_weight: target_weight__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.AllowListedValidator",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for AllowListedValidatorSetChangeProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if self.allow_listed_validators.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.AllowListedValidatorSetChangeProposal",
                len,
            )?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if let Some(v) = self.allow_listed_validators.as_ref() {
            struct_ser.serialize_field("allowListedValidators", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AllowListedValidatorSetChangeProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "allow_listed_validators",
            "allowListedValidators",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            AllowListedValidators,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "allowListedValidators" | "allow_listed_validators" => {
                                Ok(GeneratedField::AllowListedValidators)
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
            type Value = AllowListedValidatorSetChangeProposal;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.AllowListedValidatorSetChangeProposal",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<AllowListedValidatorSetChangeProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut allow_listed_validators__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("description"),
                                );
                            }
                            description__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowListedValidators => {
                            if allow_listed_validators__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("allowListedValidators"),
                                );
                            }
                            allow_listed_validators__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AllowListedValidatorSetChangeProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    allow_listed_validators: allow_listed_validators__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.AllowListedValidatorSetChangeProposal",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for AllowListedValidators {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.allow_listed_validators.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.AllowListedValidators", len)?;
        if !self.allow_listed_validators.is_empty() {
            struct_ser
                .serialize_field(
                    "allowListedValidators",
                    &self.allow_listed_validators,
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AllowListedValidators {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["allow_listed_validators", "allowListedValidators"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowListedValidators,
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
                            "allowListedValidators" | "allow_listed_validators" => {
                                Ok(GeneratedField::AllowListedValidators)
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
            type Value = AllowListedValidators;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.lscosmos.v1beta1.AllowListedValidators")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<AllowListedValidators, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut allow_listed_validators__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AllowListedValidators => {
                            if allow_listed_validators__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("allowListedValidators"),
                                );
                            }
                            allow_listed_validators__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AllowListedValidators {
                    allow_listed_validators: allow_listed_validators__
                        .unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.AllowListedValidators",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for DelegationState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.host_delegation_account_balance.is_empty() {
            len += 1;
        }
        if !self.host_chain_delegation_address.is_empty() {
            len += 1;
        }
        if !self.host_account_delegations.is_empty() {
            len += 1;
        }
        if !self.host_account_undelegations.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.DelegationState", len)?;
        if !self.host_delegation_account_balance.is_empty() {
            struct_ser
                .serialize_field(
                    "hostDelegationAccountBalance",
                    &self.host_delegation_account_balance,
                )?;
        }
        if !self.host_chain_delegation_address.is_empty() {
            struct_ser
                .serialize_field(
                    "hostChainDelegationAddress",
                    &self.host_chain_delegation_address,
                )?;
        }
        if !self.host_account_delegations.is_empty() {
            struct_ser
                .serialize_field(
                    "hostAccountDelegations",
                    &self.host_account_delegations,
                )?;
        }
        if !self.host_account_undelegations.is_empty() {
            struct_ser
                .serialize_field(
                    "hostAccountUndelegations",
                    &self.host_account_undelegations,
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DelegationState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "host_delegation_account_balance",
            "hostDelegationAccountBalance",
            "host_chain_delegation_address",
            "hostChainDelegationAddress",
            "host_account_delegations",
            "hostAccountDelegations",
            "host_account_undelegations",
            "hostAccountUndelegations",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HostDelegationAccountBalance,
            HostChainDelegationAddress,
            HostAccountDelegations,
            HostAccountUndelegations,
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
                            "hostDelegationAccountBalance"
                            | "host_delegation_account_balance" => {
                                Ok(GeneratedField::HostDelegationAccountBalance)
                            }
                            "hostChainDelegationAddress"
                            | "host_chain_delegation_address" => {
                                Ok(GeneratedField::HostChainDelegationAddress)
                            }
                            "hostAccountDelegations" | "host_account_delegations" => {
                                Ok(GeneratedField::HostAccountDelegations)
                            }
                            "hostAccountUndelegations" | "host_account_undelegations" => {
                                Ok(GeneratedField::HostAccountUndelegations)
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
            type Value = DelegationState;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.lscosmos.v1beta1.DelegationState")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<DelegationState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut host_delegation_account_balance__ = None;
                let mut host_chain_delegation_address__ = None;
                let mut host_account_delegations__ = None;
                let mut host_account_undelegations__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HostDelegationAccountBalance => {
                            if host_delegation_account_balance__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field(
                                        "hostDelegationAccountBalance",
                                    ),
                                );
                            }
                            host_delegation_account_balance__ = Some(map.next_value()?);
                        }
                        GeneratedField::HostChainDelegationAddress => {
                            if host_chain_delegation_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field(
                                        "hostChainDelegationAddress",
                                    ),
                                );
                            }
                            host_chain_delegation_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::HostAccountDelegations => {
                            if host_account_delegations__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("hostAccountDelegations"),
                                );
                            }
                            host_account_delegations__ = Some(map.next_value()?);
                        }
                        GeneratedField::HostAccountUndelegations => {
                            if host_account_undelegations__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field(
                                        "hostAccountUndelegations",
                                    ),
                                );
                            }
                            host_account_undelegations__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DelegationState {
                    host_delegation_account_balance: host_delegation_account_balance__
                        .unwrap_or_default(),
                    host_chain_delegation_address: host_chain_delegation_address__
                        .unwrap_or_default(),
                    host_account_delegations: host_account_delegations__
                        .unwrap_or_default(),
                    host_account_undelegations: host_account_undelegations__
                        .unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.DelegationState",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for DelegatorUnbondingEpochEntry {
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
        if self.epoch_number != 0 {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.DelegatorUnbondingEpochEntry",
                len,
            )?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        if self.epoch_number != 0 {
            struct_ser
                .serialize_field(
                    "epochNumber",
                    ToString::to_string(&self.epoch_number).as_str(),
                )?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DelegatorUnbondingEpochEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delegator_address",
            "delegatorAddress",
            "epoch_number",
            "epochNumber",
            "amount",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
            EpochNumber,
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
                            "epochNumber" | "epoch_number" => {
                                Ok(GeneratedField::EpochNumber)
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
            type Value = DelegatorUnbondingEpochEntry;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.DelegatorUnbondingEpochEntry",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<DelegatorUnbondingEpochEntry, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
                let mut epoch_number__ = None;
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
                Ok(DelegatorUnbondingEpochEntry {
                    delegator_address: delegator_address__.unwrap_or_default(),
                    epoch_number: epoch_number__.unwrap_or_default(),
                    amount: amount__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.DelegatorUnbondingEpochEntry",
                FIELDS,
                GeneratedVisitor,
            )
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
        if self.module_enabled {
            len += 1;
        }
        if self.host_chain_params.is_some() {
            len += 1;
        }
        if self.allow_listed_validators.is_some() {
            len += 1;
        }
        if self.delegation_state.is_some() {
            len += 1;
        }
        if self.host_chain_reward_address.is_some() {
            len += 1;
        }
        if self.i_b_c_amount_transient_store.is_some() {
            len += 1;
        }
        if !self.unbonding_epoch_c_values.is_empty() {
            len += 1;
        }
        if !self.delegator_unbonding_epoch_entries.is_empty() {
            len += 1;
        }
        if self.host_accounts.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if self.module_enabled {
            struct_ser.serialize_field("moduleEnabled", &self.module_enabled)?;
        }
        if let Some(v) = self.host_chain_params.as_ref() {
            struct_ser.serialize_field("hostChainParams", v)?;
        }
        if let Some(v) = self.allow_listed_validators.as_ref() {
            struct_ser.serialize_field("allowListedValidators", v)?;
        }
        if let Some(v) = self.delegation_state.as_ref() {
            struct_ser.serialize_field("delegationState", v)?;
        }
        if let Some(v) = self.host_chain_reward_address.as_ref() {
            struct_ser.serialize_field("hostChainRewardAddress", v)?;
        }
        if let Some(v) = self.i_b_c_amount_transient_store.as_ref() {
            struct_ser.serialize_field("iBCAmountTransientStore", v)?;
        }
        if !self.unbonding_epoch_c_values.is_empty() {
            struct_ser
                .serialize_field(
                    "unbondingEpochCValues",
                    &self.unbonding_epoch_c_values,
                )?;
        }
        if !self.delegator_unbonding_epoch_entries.is_empty() {
            struct_ser
                .serialize_field(
                    "delegatorUnbondingEpochEntries",
                    &self.delegator_unbonding_epoch_entries,
                )?;
        }
        if let Some(v) = self.host_accounts.as_ref() {
            struct_ser.serialize_field("hostAccounts", v)?;
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
            "module_enabled",
            "moduleEnabled",
            "host_chain_params",
            "hostChainParams",
            "allow_listed_validators",
            "allowListedValidators",
            "delegation_state",
            "delegationState",
            "host_chain_reward_address",
            "hostChainRewardAddress",
            "i_b_c_amount_transient_store",
            "iBCAmountTransientStore",
            "unbonding_epoch_c_values",
            "unbondingEpochCValues",
            "delegator_unbonding_epoch_entries",
            "delegatorUnbondingEpochEntries",
            "host_accounts",
            "hostAccounts",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            ModuleEnabled,
            HostChainParams,
            AllowListedValidators,
            DelegationState,
            HostChainRewardAddress,
            IBCAmountTransientStore,
            UnbondingEpochCValues,
            DelegatorUnbondingEpochEntries,
            HostAccounts,
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
                            "moduleEnabled" | "module_enabled" => {
                                Ok(GeneratedField::ModuleEnabled)
                            }
                            "hostChainParams" | "host_chain_params" => {
                                Ok(GeneratedField::HostChainParams)
                            }
                            "allowListedValidators" | "allow_listed_validators" => {
                                Ok(GeneratedField::AllowListedValidators)
                            }
                            "delegationState" | "delegation_state" => {
                                Ok(GeneratedField::DelegationState)
                            }
                            "hostChainRewardAddress" | "host_chain_reward_address" => {
                                Ok(GeneratedField::HostChainRewardAddress)
                            }
                            "iBCAmountTransientStore"
                            | "i_b_c_amount_transient_store" => {
                                Ok(GeneratedField::IBCAmountTransientStore)
                            }
                            "unbondingEpochCValues" | "unbonding_epoch_c_values" => {
                                Ok(GeneratedField::UnbondingEpochCValues)
                            }
                            "delegatorUnbondingEpochEntries"
                            | "delegator_unbonding_epoch_entries" => {
                                Ok(GeneratedField::DelegatorUnbondingEpochEntries)
                            }
                            "hostAccounts" | "host_accounts" => {
                                Ok(GeneratedField::HostAccounts)
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
                formatter.write_str("struct pstake.lscosmos.v1beta1.GenesisState")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut module_enabled__ = None;
                let mut host_chain_params__ = None;
                let mut allow_listed_validators__ = None;
                let mut delegation_state__ = None;
                let mut host_chain_reward_address__ = None;
                let mut i_b_c_amount_transient_store__ = None;
                let mut unbonding_epoch_c_values__ = None;
                let mut delegator_unbonding_epoch_entries__ = None;
                let mut host_accounts__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                        GeneratedField::ModuleEnabled => {
                            if module_enabled__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("moduleEnabled"),
                                );
                            }
                            module_enabled__ = Some(map.next_value()?);
                        }
                        GeneratedField::HostChainParams => {
                            if host_chain_params__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("hostChainParams"),
                                );
                            }
                            host_chain_params__ = map.next_value()?;
                        }
                        GeneratedField::AllowListedValidators => {
                            if allow_listed_validators__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("allowListedValidators"),
                                );
                            }
                            allow_listed_validators__ = map.next_value()?;
                        }
                        GeneratedField::DelegationState => {
                            if delegation_state__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("delegationState"),
                                );
                            }
                            delegation_state__ = map.next_value()?;
                        }
                        GeneratedField::HostChainRewardAddress => {
                            if host_chain_reward_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("hostChainRewardAddress"),
                                );
                            }
                            host_chain_reward_address__ = map.next_value()?;
                        }
                        GeneratedField::IBCAmountTransientStore => {
                            if i_b_c_amount_transient_store__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("iBCAmountTransientStore"),
                                );
                            }
                            i_b_c_amount_transient_store__ = map.next_value()?;
                        }
                        GeneratedField::UnbondingEpochCValues => {
                            if unbonding_epoch_c_values__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("unbondingEpochCValues"),
                                );
                            }
                            unbonding_epoch_c_values__ = Some(map.next_value()?);
                        }
                        GeneratedField::DelegatorUnbondingEpochEntries => {
                            if delegator_unbonding_epoch_entries__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field(
                                        "delegatorUnbondingEpochEntries",
                                    ),
                                );
                            }
                            delegator_unbonding_epoch_entries__ = Some(
                                map.next_value()?,
                            );
                        }
                        GeneratedField::HostAccounts => {
                            if host_accounts__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("hostAccounts"),
                                );
                            }
                            host_accounts__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    module_enabled: module_enabled__.unwrap_or_default(),
                    host_chain_params: host_chain_params__,
                    allow_listed_validators: allow_listed_validators__,
                    delegation_state: delegation_state__,
                    host_chain_reward_address: host_chain_reward_address__,
                    i_b_c_amount_transient_store: i_b_c_amount_transient_store__,
                    unbonding_epoch_c_values: unbonding_epoch_c_values__
                        .unwrap_or_default(),
                    delegator_unbonding_epoch_entries: delegator_unbonding_epoch_entries__
                        .unwrap_or_default(),
                    host_accounts: host_accounts__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.GenesisState",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for HostAccountDelegation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.validator_address.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.HostAccountDelegation", len)?;
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostAccountDelegation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["validator_address", "validatorAddress", "amount"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidatorAddress,
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
                            "validatorAddress" | "validator_address" => {
                                Ok(GeneratedField::ValidatorAddress)
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
            type Value = HostAccountDelegation;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.lscosmos.v1beta1.HostAccountDelegation")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<HostAccountDelegation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut validator_address__ = None;
                let mut amount__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostAccountDelegation {
                    validator_address: validator_address__.unwrap_or_default(),
                    amount: amount__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.HostAccountDelegation",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for HostAccountUndelegation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.epoch_number != 0 {
            len += 1;
        }
        if self.total_undelegation_amount.is_some() {
            len += 1;
        }
        if self.completion_time.is_some() {
            len += 1;
        }
        if !self.undelegation_entries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.HostAccountUndelegation", len)?;
        if self.epoch_number != 0 {
            struct_ser
                .serialize_field(
                    "epochNumber",
                    ToString::to_string(&self.epoch_number).as_str(),
                )?;
        }
        if let Some(v) = self.total_undelegation_amount.as_ref() {
            struct_ser.serialize_field("totalUndelegationAmount", v)?;
        }
        if let Some(v) = self.completion_time.as_ref() {
            struct_ser.serialize_field("completionTime", v)?;
        }
        if !self.undelegation_entries.is_empty() {
            struct_ser
                .serialize_field("undelegationEntries", &self.undelegation_entries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostAccountUndelegation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "epoch_number",
            "epochNumber",
            "total_undelegation_amount",
            "totalUndelegationAmount",
            "completion_time",
            "completionTime",
            "undelegation_entries",
            "undelegationEntries",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EpochNumber,
            TotalUndelegationAmount,
            CompletionTime,
            UndelegationEntries,
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
                            "epochNumber" | "epoch_number" => {
                                Ok(GeneratedField::EpochNumber)
                            }
                            "totalUndelegationAmount" | "total_undelegation_amount" => {
                                Ok(GeneratedField::TotalUndelegationAmount)
                            }
                            "completionTime" | "completion_time" => {
                                Ok(GeneratedField::CompletionTime)
                            }
                            "undelegationEntries" | "undelegation_entries" => {
                                Ok(GeneratedField::UndelegationEntries)
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
            type Value = HostAccountUndelegation;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.lscosmos.v1beta1.HostAccountUndelegation")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<HostAccountUndelegation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut epoch_number__ = None;
                let mut total_undelegation_amount__ = None;
                let mut completion_time__ = None;
                let mut undelegation_entries__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                        GeneratedField::TotalUndelegationAmount => {
                            if total_undelegation_amount__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("totalUndelegationAmount"),
                                );
                            }
                            total_undelegation_amount__ = map.next_value()?;
                        }
                        GeneratedField::CompletionTime => {
                            if completion_time__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("completionTime"),
                                );
                            }
                            completion_time__ = map.next_value()?;
                        }
                        GeneratedField::UndelegationEntries => {
                            if undelegation_entries__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("undelegationEntries"),
                                );
                            }
                            undelegation_entries__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostAccountUndelegation {
                    epoch_number: epoch_number__.unwrap_or_default(),
                    total_undelegation_amount: total_undelegation_amount__,
                    completion_time: completion_time__,
                    undelegation_entries: undelegation_entries__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.HostAccountUndelegation",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for HostAccounts {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.delegator_account_owner_i_d.is_empty() {
            len += 1;
        }
        if !self.rewards_account_owner_i_d.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.HostAccounts", len)?;
        if !self.delegator_account_owner_i_d.is_empty() {
            struct_ser
                .serialize_field(
                    "delegatorAccountOwnerID",
                    &self.delegator_account_owner_i_d,
                )?;
        }
        if !self.rewards_account_owner_i_d.is_empty() {
            struct_ser
                .serialize_field(
                    "rewardsAccountOwnerID",
                    &self.rewards_account_owner_i_d,
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostAccounts {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delegator_account_owner_i_d",
            "delegatorAccountOwnerID",
            "rewards_account_owner_i_d",
            "rewardsAccountOwnerID",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAccountOwnerID,
            RewardsAccountOwnerID,
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
                            "delegatorAccountOwnerID" | "delegator_account_owner_i_d" => {
                                Ok(GeneratedField::DelegatorAccountOwnerID)
                            }
                            "rewardsAccountOwnerID" | "rewards_account_owner_i_d" => {
                                Ok(GeneratedField::RewardsAccountOwnerID)
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
            type Value = HostAccounts;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.lscosmos.v1beta1.HostAccounts")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<HostAccounts, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_account_owner_i_d__ = None;
                let mut rewards_account_owner_i_d__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DelegatorAccountOwnerID => {
                            if delegator_account_owner_i_d__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("delegatorAccountOwnerID"),
                                );
                            }
                            delegator_account_owner_i_d__ = Some(map.next_value()?);
                        }
                        GeneratedField::RewardsAccountOwnerID => {
                            if rewards_account_owner_i_d__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("rewardsAccountOwnerID"),
                                );
                            }
                            rewards_account_owner_i_d__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostAccounts {
                    delegator_account_owner_i_d: delegator_account_owner_i_d__
                        .unwrap_or_default(),
                    rewards_account_owner_i_d: rewards_account_owner_i_d__
                        .unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.HostAccounts",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for HostChainParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.chain_i_d.is_empty() {
            len += 1;
        }
        if !self.connection_i_d.is_empty() {
            len += 1;
        }
        if !self.transfer_channel.is_empty() {
            len += 1;
        }
        if !self.transfer_port.is_empty() {
            len += 1;
        }
        if !self.base_denom.is_empty() {
            len += 1;
        }
        if !self.mint_denom.is_empty() {
            len += 1;
        }
        if !self.min_deposit.is_empty() {
            len += 1;
        }
        if self.pstake_params.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.HostChainParams", len)?;
        if !self.chain_i_d.is_empty() {
            struct_ser.serialize_field("chainID", &self.chain_i_d)?;
        }
        if !self.connection_i_d.is_empty() {
            struct_ser.serialize_field("connectionID", &self.connection_i_d)?;
        }
        if !self.transfer_channel.is_empty() {
            struct_ser.serialize_field("transferChannel", &self.transfer_channel)?;
        }
        if !self.transfer_port.is_empty() {
            struct_ser.serialize_field("transferPort", &self.transfer_port)?;
        }
        if !self.base_denom.is_empty() {
            struct_ser.serialize_field("baseDenom", &self.base_denom)?;
        }
        if !self.mint_denom.is_empty() {
            struct_ser.serialize_field("mintDenom", &self.mint_denom)?;
        }
        if !self.min_deposit.is_empty() {
            struct_ser.serialize_field("minDeposit", &self.min_deposit)?;
        }
        if let Some(v) = self.pstake_params.as_ref() {
            struct_ser.serialize_field("pstakeParams", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostChainParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "chain_i_d",
            "chainID",
            "connection_i_d",
            "connectionID",
            "transfer_channel",
            "transferChannel",
            "transfer_port",
            "transferPort",
            "base_denom",
            "baseDenom",
            "mint_denom",
            "mintDenom",
            "min_deposit",
            "minDeposit",
            "pstake_params",
            "pstakeParams",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ChainID,
            ConnectionID,
            TransferChannel,
            TransferPort,
            BaseDenom,
            MintDenom,
            MinDeposit,
            PstakeParams,
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
                            "chainID" | "chain_i_d" => Ok(GeneratedField::ChainID),
                            "connectionID" | "connection_i_d" => {
                                Ok(GeneratedField::ConnectionID)
                            }
                            "transferChannel" | "transfer_channel" => {
                                Ok(GeneratedField::TransferChannel)
                            }
                            "transferPort" | "transfer_port" => {
                                Ok(GeneratedField::TransferPort)
                            }
                            "baseDenom" | "base_denom" => Ok(GeneratedField::BaseDenom),
                            "mintDenom" | "mint_denom" => Ok(GeneratedField::MintDenom),
                            "minDeposit" | "min_deposit" => {
                                Ok(GeneratedField::MinDeposit)
                            }
                            "pstakeParams" | "pstake_params" => {
                                Ok(GeneratedField::PstakeParams)
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
            type Value = HostChainParams;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.lscosmos.v1beta1.HostChainParams")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<HostChainParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut chain_i_d__ = None;
                let mut connection_i_d__ = None;
                let mut transfer_channel__ = None;
                let mut transfer_port__ = None;
                let mut base_denom__ = None;
                let mut mint_denom__ = None;
                let mut min_deposit__ = None;
                let mut pstake_params__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ChainID => {
                            if chain_i_d__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainID"));
                            }
                            chain_i_d__ = Some(map.next_value()?);
                        }
                        GeneratedField::ConnectionID => {
                            if connection_i_d__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("connectionID"),
                                );
                            }
                            connection_i_d__ = Some(map.next_value()?);
                        }
                        GeneratedField::TransferChannel => {
                            if transfer_channel__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("transferChannel"),
                                );
                            }
                            transfer_channel__ = Some(map.next_value()?);
                        }
                        GeneratedField::TransferPort => {
                            if transfer_port__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("transferPort"),
                                );
                            }
                            transfer_port__ = Some(map.next_value()?);
                        }
                        GeneratedField::BaseDenom => {
                            if base_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseDenom"));
                            }
                            base_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::MintDenom => {
                            if mint_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mintDenom"));
                            }
                            mint_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::MinDeposit => {
                            if min_deposit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minDeposit"));
                            }
                            min_deposit__ = Some(map.next_value()?);
                        }
                        GeneratedField::PstakeParams => {
                            if pstake_params__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("pstakeParams"),
                                );
                            }
                            pstake_params__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(HostChainParams {
                    chain_i_d: chain_i_d__.unwrap_or_default(),
                    connection_i_d: connection_i_d__.unwrap_or_default(),
                    transfer_channel: transfer_channel__.unwrap_or_default(),
                    transfer_port: transfer_port__.unwrap_or_default(),
                    base_denom: base_denom__.unwrap_or_default(),
                    mint_denom: mint_denom__.unwrap_or_default(),
                    min_deposit: min_deposit__.unwrap_or_default(),
                    pstake_params: pstake_params__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.HostChainParams",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for HostChainRewardAddress {
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
            .serialize_struct("pstake.lscosmos.v1beta1.HostChainRewardAddress", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for HostChainRewardAddress {
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
            type Value = HostChainRewardAddress;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.lscosmos.v1beta1.HostChainRewardAddress")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<HostChainRewardAddress, V::Error>
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
                Ok(HostChainRewardAddress {
                    address: address__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.HostChainRewardAddress",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for IbcAmountTransientStore {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.i_b_c_transfer.is_empty() {
            len += 1;
        }
        if self.i_c_a_delegate.is_some() {
            len += 1;
        }
        if !self.undelegaton_complete_i_b_c_transfer.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.IBCAmountTransientStore", len)?;
        if !self.i_b_c_transfer.is_empty() {
            struct_ser.serialize_field("iBCTransfer", &self.i_b_c_transfer)?;
        }
        if let Some(v) = self.i_c_a_delegate.as_ref() {
            struct_ser.serialize_field("iCADelegate", v)?;
        }
        if !self.undelegaton_complete_i_b_c_transfer.is_empty() {
            struct_ser
                .serialize_field(
                    "undelegatonCompleteIBCTransfer",
                    &self.undelegaton_complete_i_b_c_transfer,
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IbcAmountTransientStore {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "i_b_c_transfer",
            "iBCTransfer",
            "i_c_a_delegate",
            "iCADelegate",
            "undelegaton_complete_i_b_c_transfer",
            "undelegatonCompleteIBCTransfer",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IBCTransfer,
            ICADelegate,
            UndelegatonCompleteIBCTransfer,
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
                            "iBCTransfer" | "i_b_c_transfer" => {
                                Ok(GeneratedField::IBCTransfer)
                            }
                            "iCADelegate" | "i_c_a_delegate" => {
                                Ok(GeneratedField::ICADelegate)
                            }
                            "undelegatonCompleteIBCTransfer"
                            | "undelegaton_complete_i_b_c_transfer" => {
                                Ok(GeneratedField::UndelegatonCompleteIBCTransfer)
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
            type Value = IbcAmountTransientStore;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.lscosmos.v1beta1.IBCAmountTransientStore")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<IbcAmountTransientStore, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut i_b_c_transfer__ = None;
                let mut i_c_a_delegate__ = None;
                let mut undelegaton_complete_i_b_c_transfer__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IBCTransfer => {
                            if i_b_c_transfer__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("iBCTransfer"),
                                );
                            }
                            i_b_c_transfer__ = Some(map.next_value()?);
                        }
                        GeneratedField::ICADelegate => {
                            if i_c_a_delegate__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("iCADelegate"),
                                );
                            }
                            i_c_a_delegate__ = map.next_value()?;
                        }
                        GeneratedField::UndelegatonCompleteIBCTransfer => {
                            if undelegaton_complete_i_b_c_transfer__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field(
                                        "undelegatonCompleteIBCTransfer",
                                    ),
                                );
                            }
                            undelegaton_complete_i_b_c_transfer__ = Some(
                                map.next_value()?,
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(IbcAmountTransientStore {
                    i_b_c_transfer: i_b_c_transfer__.unwrap_or_default(),
                    i_c_a_delegate: i_c_a_delegate__,
                    undelegaton_complete_i_b_c_transfer: undelegaton_complete_i_b_c_transfer__
                        .unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.IBCAmountTransientStore",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MinDepositAndFeeChangeProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.min_deposit.is_empty() {
            len += 1;
        }
        if !self.pstake_deposit_fee.is_empty() {
            len += 1;
        }
        if !self.pstake_restake_fee.is_empty() {
            len += 1;
        }
        if !self.pstake_unstake_fee.is_empty() {
            len += 1;
        }
        if !self.pstake_redemption_fee.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.MinDepositAndFeeChangeProposal",
                len,
            )?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.min_deposit.is_empty() {
            struct_ser.serialize_field("minDeposit", &self.min_deposit)?;
        }
        if !self.pstake_deposit_fee.is_empty() {
            struct_ser.serialize_field("pstakeDepositFee", &self.pstake_deposit_fee)?;
        }
        if !self.pstake_restake_fee.is_empty() {
            struct_ser.serialize_field("pstakeRestakeFee", &self.pstake_restake_fee)?;
        }
        if !self.pstake_unstake_fee.is_empty() {
            struct_ser.serialize_field("pstakeUnstakeFee", &self.pstake_unstake_fee)?;
        }
        if !self.pstake_redemption_fee.is_empty() {
            struct_ser
                .serialize_field("pstakeRedemptionFee", &self.pstake_redemption_fee)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MinDepositAndFeeChangeProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "min_deposit",
            "minDeposit",
            "pstake_deposit_fee",
            "pstakeDepositFee",
            "pstake_restake_fee",
            "pstakeRestakeFee",
            "pstake_unstake_fee",
            "pstakeUnstakeFee",
            "pstake_redemption_fee",
            "pstakeRedemptionFee",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            MinDeposit,
            PstakeDepositFee,
            PstakeRestakeFee,
            PstakeUnstakeFee,
            PstakeRedemptionFee,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "minDeposit" | "min_deposit" => {
                                Ok(GeneratedField::MinDeposit)
                            }
                            "pstakeDepositFee" | "pstake_deposit_fee" => {
                                Ok(GeneratedField::PstakeDepositFee)
                            }
                            "pstakeRestakeFee" | "pstake_restake_fee" => {
                                Ok(GeneratedField::PstakeRestakeFee)
                            }
                            "pstakeUnstakeFee" | "pstake_unstake_fee" => {
                                Ok(GeneratedField::PstakeUnstakeFee)
                            }
                            "pstakeRedemptionFee" | "pstake_redemption_fee" => {
                                Ok(GeneratedField::PstakeRedemptionFee)
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
            type Value = MinDepositAndFeeChangeProposal;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.MinDepositAndFeeChangeProposal",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MinDepositAndFeeChangeProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut min_deposit__ = None;
                let mut pstake_deposit_fee__ = None;
                let mut pstake_restake_fee__ = None;
                let mut pstake_unstake_fee__ = None;
                let mut pstake_redemption_fee__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("description"),
                                );
                            }
                            description__ = Some(map.next_value()?);
                        }
                        GeneratedField::MinDeposit => {
                            if min_deposit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minDeposit"));
                            }
                            min_deposit__ = Some(map.next_value()?);
                        }
                        GeneratedField::PstakeDepositFee => {
                            if pstake_deposit_fee__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("pstakeDepositFee"),
                                );
                            }
                            pstake_deposit_fee__ = Some(map.next_value()?);
                        }
                        GeneratedField::PstakeRestakeFee => {
                            if pstake_restake_fee__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("pstakeRestakeFee"),
                                );
                            }
                            pstake_restake_fee__ = Some(map.next_value()?);
                        }
                        GeneratedField::PstakeUnstakeFee => {
                            if pstake_unstake_fee__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("pstakeUnstakeFee"),
                                );
                            }
                            pstake_unstake_fee__ = Some(map.next_value()?);
                        }
                        GeneratedField::PstakeRedemptionFee => {
                            if pstake_redemption_fee__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("pstakeRedemptionFee"),
                                );
                            }
                            pstake_redemption_fee__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MinDepositAndFeeChangeProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    min_deposit: min_deposit__.unwrap_or_default(),
                    pstake_deposit_fee: pstake_deposit_fee__.unwrap_or_default(),
                    pstake_restake_fee: pstake_restake_fee__.unwrap_or_default(),
                    pstake_unstake_fee: pstake_unstake_fee__.unwrap_or_default(),
                    pstake_redemption_fee: pstake_redemption_fee__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.MinDepositAndFeeChangeProposal",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgChangeModuleState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pstake_address.is_empty() {
            len += 1;
        }
        if self.module_state {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.MsgChangeModuleState", len)?;
        if !self.pstake_address.is_empty() {
            struct_ser.serialize_field("pstakeAddress", &self.pstake_address)?;
        }
        if self.module_state {
            struct_ser.serialize_field("moduleState", &self.module_state)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgChangeModuleState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pstake_address",
            "pstakeAddress",
            "module_state",
            "moduleState",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PstakeAddress,
            ModuleState,
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
                            "pstakeAddress" | "pstake_address" => {
                                Ok(GeneratedField::PstakeAddress)
                            }
                            "moduleState" | "module_state" => {
                                Ok(GeneratedField::ModuleState)
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
            type Value = MsgChangeModuleState;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.lscosmos.v1beta1.MsgChangeModuleState")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgChangeModuleState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pstake_address__ = None;
                let mut module_state__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PstakeAddress => {
                            if pstake_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("pstakeAddress"),
                                );
                            }
                            pstake_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::ModuleState => {
                            if module_state__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("moduleState"),
                                );
                            }
                            module_state__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgChangeModuleState {
                    pstake_address: pstake_address__.unwrap_or_default(),
                    module_state: module_state__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.MsgChangeModuleState",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgChangeModuleStateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.MsgChangeModuleStateResponse",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgChangeModuleStateResponse {
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
            type Value = MsgChangeModuleStateResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.MsgChangeModuleStateResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgChangeModuleStateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgChangeModuleStateResponse {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.MsgChangeModuleStateResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgClaim {
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
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.MsgClaim", len)?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgClaim {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["delegator_address", "delegatorAddress"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
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
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgClaim;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.lscosmos.v1beta1.MsgClaim")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<MsgClaim, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
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
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgClaim {
                    delegator_address: delegator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.MsgClaim",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgClaimResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.MsgClaimResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgClaimResponse {
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
            type Value = MsgClaimResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.lscosmos.v1beta1.MsgClaimResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgClaimResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgClaimResponse {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.MsgClaimResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgJumpStart {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pstake_address.is_empty() {
            len += 1;
        }
        if !self.chain_i_d.is_empty() {
            len += 1;
        }
        if !self.connection_i_d.is_empty() {
            len += 1;
        }
        if !self.transfer_channel.is_empty() {
            len += 1;
        }
        if !self.transfer_port.is_empty() {
            len += 1;
        }
        if !self.base_denom.is_empty() {
            len += 1;
        }
        if !self.mint_denom.is_empty() {
            len += 1;
        }
        if !self.min_deposit.is_empty() {
            len += 1;
        }
        if self.allow_listed_validators.is_some() {
            len += 1;
        }
        if self.pstake_params.is_some() {
            len += 1;
        }
        if self.host_accounts.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.MsgJumpStart", len)?;
        if !self.pstake_address.is_empty() {
            struct_ser.serialize_field("pstakeAddress", &self.pstake_address)?;
        }
        if !self.chain_i_d.is_empty() {
            struct_ser.serialize_field("chainID", &self.chain_i_d)?;
        }
        if !self.connection_i_d.is_empty() {
            struct_ser.serialize_field("connectionID", &self.connection_i_d)?;
        }
        if !self.transfer_channel.is_empty() {
            struct_ser.serialize_field("transferChannel", &self.transfer_channel)?;
        }
        if !self.transfer_port.is_empty() {
            struct_ser.serialize_field("transferPort", &self.transfer_port)?;
        }
        if !self.base_denom.is_empty() {
            struct_ser.serialize_field("baseDenom", &self.base_denom)?;
        }
        if !self.mint_denom.is_empty() {
            struct_ser.serialize_field("mintDenom", &self.mint_denom)?;
        }
        if !self.min_deposit.is_empty() {
            struct_ser.serialize_field("minDeposit", &self.min_deposit)?;
        }
        if let Some(v) = self.allow_listed_validators.as_ref() {
            struct_ser.serialize_field("allowListedValidators", v)?;
        }
        if let Some(v) = self.pstake_params.as_ref() {
            struct_ser.serialize_field("pstakeParams", v)?;
        }
        if let Some(v) = self.host_accounts.as_ref() {
            struct_ser.serialize_field("hostAccounts", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgJumpStart {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pstake_address",
            "pstakeAddress",
            "chain_i_d",
            "chainID",
            "connection_i_d",
            "connectionID",
            "transfer_channel",
            "transferChannel",
            "transfer_port",
            "transferPort",
            "base_denom",
            "baseDenom",
            "mint_denom",
            "mintDenom",
            "min_deposit",
            "minDeposit",
            "allow_listed_validators",
            "allowListedValidators",
            "pstake_params",
            "pstakeParams",
            "host_accounts",
            "hostAccounts",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PstakeAddress,
            ChainID,
            ConnectionID,
            TransferChannel,
            TransferPort,
            BaseDenom,
            MintDenom,
            MinDeposit,
            AllowListedValidators,
            PstakeParams,
            HostAccounts,
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
                            "pstakeAddress" | "pstake_address" => {
                                Ok(GeneratedField::PstakeAddress)
                            }
                            "chainID" | "chain_i_d" => Ok(GeneratedField::ChainID),
                            "connectionID" | "connection_i_d" => {
                                Ok(GeneratedField::ConnectionID)
                            }
                            "transferChannel" | "transfer_channel" => {
                                Ok(GeneratedField::TransferChannel)
                            }
                            "transferPort" | "transfer_port" => {
                                Ok(GeneratedField::TransferPort)
                            }
                            "baseDenom" | "base_denom" => Ok(GeneratedField::BaseDenom),
                            "mintDenom" | "mint_denom" => Ok(GeneratedField::MintDenom),
                            "minDeposit" | "min_deposit" => {
                                Ok(GeneratedField::MinDeposit)
                            }
                            "allowListedValidators" | "allow_listed_validators" => {
                                Ok(GeneratedField::AllowListedValidators)
                            }
                            "pstakeParams" | "pstake_params" => {
                                Ok(GeneratedField::PstakeParams)
                            }
                            "hostAccounts" | "host_accounts" => {
                                Ok(GeneratedField::HostAccounts)
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
            type Value = MsgJumpStart;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.lscosmos.v1beta1.MsgJumpStart")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgJumpStart, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pstake_address__ = None;
                let mut chain_i_d__ = None;
                let mut connection_i_d__ = None;
                let mut transfer_channel__ = None;
                let mut transfer_port__ = None;
                let mut base_denom__ = None;
                let mut mint_denom__ = None;
                let mut min_deposit__ = None;
                let mut allow_listed_validators__ = None;
                let mut pstake_params__ = None;
                let mut host_accounts__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PstakeAddress => {
                            if pstake_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("pstakeAddress"),
                                );
                            }
                            pstake_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChainID => {
                            if chain_i_d__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainID"));
                            }
                            chain_i_d__ = Some(map.next_value()?);
                        }
                        GeneratedField::ConnectionID => {
                            if connection_i_d__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("connectionID"),
                                );
                            }
                            connection_i_d__ = Some(map.next_value()?);
                        }
                        GeneratedField::TransferChannel => {
                            if transfer_channel__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("transferChannel"),
                                );
                            }
                            transfer_channel__ = Some(map.next_value()?);
                        }
                        GeneratedField::TransferPort => {
                            if transfer_port__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("transferPort"),
                                );
                            }
                            transfer_port__ = Some(map.next_value()?);
                        }
                        GeneratedField::BaseDenom => {
                            if base_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseDenom"));
                            }
                            base_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::MintDenom => {
                            if mint_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mintDenom"));
                            }
                            mint_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::MinDeposit => {
                            if min_deposit__.is_some() {
                                return Err(serde::de::Error::duplicate_field("minDeposit"));
                            }
                            min_deposit__ = Some(map.next_value()?);
                        }
                        GeneratedField::AllowListedValidators => {
                            if allow_listed_validators__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("allowListedValidators"),
                                );
                            }
                            allow_listed_validators__ = map.next_value()?;
                        }
                        GeneratedField::PstakeParams => {
                            if pstake_params__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("pstakeParams"),
                                );
                            }
                            pstake_params__ = map.next_value()?;
                        }
                        GeneratedField::HostAccounts => {
                            if host_accounts__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("hostAccounts"),
                                );
                            }
                            host_accounts__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgJumpStart {
                    pstake_address: pstake_address__.unwrap_or_default(),
                    chain_i_d: chain_i_d__.unwrap_or_default(),
                    connection_i_d: connection_i_d__.unwrap_or_default(),
                    transfer_channel: transfer_channel__.unwrap_or_default(),
                    transfer_port: transfer_port__.unwrap_or_default(),
                    base_denom: base_denom__.unwrap_or_default(),
                    mint_denom: mint_denom__.unwrap_or_default(),
                    min_deposit: min_deposit__.unwrap_or_default(),
                    allow_listed_validators: allow_listed_validators__,
                    pstake_params: pstake_params__,
                    host_accounts: host_accounts__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.MsgJumpStart",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgJumpStartResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.MsgJumpStartResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgJumpStartResponse {
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
            type Value = MsgJumpStartResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.lscosmos.v1beta1.MsgJumpStartResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgJumpStartResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgJumpStartResponse {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.MsgJumpStartResponse",
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
            .serialize_struct("pstake.lscosmos.v1beta1.MsgLiquidStake", len)?;
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
                formatter.write_str("struct pstake.lscosmos.v1beta1.MsgLiquidStake")
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
                "pstake.lscosmos.v1beta1.MsgLiquidStake",
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
            .serialize_struct("pstake.lscosmos.v1beta1.MsgLiquidStakeResponse", len)?;
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
                    .write_str("struct pstake.lscosmos.v1beta1.MsgLiquidStakeResponse")
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
                "pstake.lscosmos.v1beta1.MsgLiquidStakeResponse",
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
            .serialize_struct("pstake.lscosmos.v1beta1.MsgLiquidUnstake", len)?;
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
                formatter.write_str("struct pstake.lscosmos.v1beta1.MsgLiquidUnstake")
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
                "pstake.lscosmos.v1beta1.MsgLiquidUnstake",
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
            .serialize_struct("pstake.lscosmos.v1beta1.MsgLiquidUnstakeResponse", len)?;
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
                    .write_str("struct pstake.lscosmos.v1beta1.MsgLiquidUnstakeResponse")
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
                "pstake.lscosmos.v1beta1.MsgLiquidUnstakeResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgRecreateIca {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.MsgRecreateICA", len)?;
        if !self.from_address.is_empty() {
            struct_ser.serialize_field("fromAddress", &self.from_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRecreateIca {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["from_address", "fromAddress"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FromAddress,
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
                            "fromAddress" | "from_address" => {
                                Ok(GeneratedField::FromAddress)
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
            type Value = MsgRecreateIca;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.lscosmos.v1beta1.MsgRecreateICA")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgRecreateIca, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FromAddress => {
                            if from_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("fromAddress"),
                                );
                            }
                            from_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgRecreateIca {
                    from_address: from_address__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.MsgRecreateICA",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgRecreateIcaResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.MsgRecreateICAResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRecreateIcaResponse {
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
            type Value = MsgRecreateIcaResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.lscosmos.v1beta1.MsgRecreateICAResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgRecreateIcaResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgRecreateIcaResponse {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.MsgRecreateICAResponse",
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
            .serialize_struct("pstake.lscosmos.v1beta1.MsgRedeem", len)?;
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
                formatter.write_str("struct pstake.lscosmos.v1beta1.MsgRedeem")
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
                "pstake.lscosmos.v1beta1.MsgRedeem",
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
            .serialize_struct("pstake.lscosmos.v1beta1.MsgRedeemResponse", len)?;
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
                formatter.write_str("struct pstake.lscosmos.v1beta1.MsgRedeemResponse")
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
                "pstake.lscosmos.v1beta1.MsgRedeemResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgReportSlashing {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pstake_address.is_empty() {
            len += 1;
        }
        if !self.validator_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.MsgReportSlashing", len)?;
        if !self.pstake_address.is_empty() {
            struct_ser.serialize_field("pstakeAddress", &self.pstake_address)?;
        }
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgReportSlashing {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pstake_address",
            "pstakeAddress",
            "validator_address",
            "validatorAddress",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PstakeAddress,
            ValidatorAddress,
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
                            "pstakeAddress" | "pstake_address" => {
                                Ok(GeneratedField::PstakeAddress)
                            }
                            "validatorAddress" | "validator_address" => {
                                Ok(GeneratedField::ValidatorAddress)
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
            type Value = MsgReportSlashing;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.lscosmos.v1beta1.MsgReportSlashing")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgReportSlashing, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pstake_address__ = None;
                let mut validator_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PstakeAddress => {
                            if pstake_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("pstakeAddress"),
                                );
                            }
                            pstake_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::ValidatorAddress => {
                            if validator_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorAddress"),
                                );
                            }
                            validator_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgReportSlashing {
                    pstake_address: pstake_address__.unwrap_or_default(),
                    validator_address: validator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.MsgReportSlashing",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgReportSlashingResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.MsgReportSlashingResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgReportSlashingResponse {
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
            type Value = MsgReportSlashingResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.MsgReportSlashingResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgReportSlashingResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgReportSlashingResponse {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.MsgReportSlashingResponse",
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
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.Params", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Params {
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
            type Value = Params;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.lscosmos.v1beta1.Params")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(Params {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.Params",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for PstakeFeeAddressChangeProposal {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.title.is_empty() {
            len += 1;
        }
        if !self.description.is_empty() {
            len += 1;
        }
        if !self.pstake_fee_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.PstakeFeeAddressChangeProposal",
                len,
            )?;
        if !self.title.is_empty() {
            struct_ser.serialize_field("title", &self.title)?;
        }
        if !self.description.is_empty() {
            struct_ser.serialize_field("description", &self.description)?;
        }
        if !self.pstake_fee_address.is_empty() {
            struct_ser.serialize_field("pstakeFeeAddress", &self.pstake_fee_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PstakeFeeAddressChangeProposal {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "title",
            "description",
            "pstake_fee_address",
            "pstakeFeeAddress",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Title,
            Description,
            PstakeFeeAddress,
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
                            "title" => Ok(GeneratedField::Title),
                            "description" => Ok(GeneratedField::Description),
                            "pstakeFeeAddress" | "pstake_fee_address" => {
                                Ok(GeneratedField::PstakeFeeAddress)
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
            type Value = PstakeFeeAddressChangeProposal;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.PstakeFeeAddressChangeProposal",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<PstakeFeeAddressChangeProposal, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut title__ = None;
                let mut description__ = None;
                let mut pstake_fee_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Title => {
                            if title__.is_some() {
                                return Err(serde::de::Error::duplicate_field("title"));
                            }
                            title__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("description"),
                                );
                            }
                            description__ = Some(map.next_value()?);
                        }
                        GeneratedField::PstakeFeeAddress => {
                            if pstake_fee_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("pstakeFeeAddress"),
                                );
                            }
                            pstake_fee_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(PstakeFeeAddressChangeProposal {
                    title: title__.unwrap_or_default(),
                    description: description__.unwrap_or_default(),
                    pstake_fee_address: pstake_fee_address__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.PstakeFeeAddressChangeProposal",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for PstakeParams {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pstake_deposit_fee.is_empty() {
            len += 1;
        }
        if !self.pstake_restake_fee.is_empty() {
            len += 1;
        }
        if !self.pstake_unstake_fee.is_empty() {
            len += 1;
        }
        if !self.pstake_redemption_fee.is_empty() {
            len += 1;
        }
        if !self.pstake_fee_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.PstakeParams", len)?;
        if !self.pstake_deposit_fee.is_empty() {
            struct_ser.serialize_field("pstakeDepositFee", &self.pstake_deposit_fee)?;
        }
        if !self.pstake_restake_fee.is_empty() {
            struct_ser.serialize_field("pstakeRestakeFee", &self.pstake_restake_fee)?;
        }
        if !self.pstake_unstake_fee.is_empty() {
            struct_ser.serialize_field("pstakeUnstakeFee", &self.pstake_unstake_fee)?;
        }
        if !self.pstake_redemption_fee.is_empty() {
            struct_ser
                .serialize_field("pstakeRedemptionFee", &self.pstake_redemption_fee)?;
        }
        if !self.pstake_fee_address.is_empty() {
            struct_ser.serialize_field("pstakeFeeAddress", &self.pstake_fee_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PstakeParams {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "pstake_deposit_fee",
            "pstakeDepositFee",
            "pstake_restake_fee",
            "pstakeRestakeFee",
            "pstake_unstake_fee",
            "pstakeUnstakeFee",
            "pstake_redemption_fee",
            "pstakeRedemptionFee",
            "pstake_fee_address",
            "pstakeFeeAddress",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PstakeDepositFee,
            PstakeRestakeFee,
            PstakeUnstakeFee,
            PstakeRedemptionFee,
            PstakeFeeAddress,
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
                            "pstakeDepositFee" | "pstake_deposit_fee" => {
                                Ok(GeneratedField::PstakeDepositFee)
                            }
                            "pstakeRestakeFee" | "pstake_restake_fee" => {
                                Ok(GeneratedField::PstakeRestakeFee)
                            }
                            "pstakeUnstakeFee" | "pstake_unstake_fee" => {
                                Ok(GeneratedField::PstakeUnstakeFee)
                            }
                            "pstakeRedemptionFee" | "pstake_redemption_fee" => {
                                Ok(GeneratedField::PstakeRedemptionFee)
                            }
                            "pstakeFeeAddress" | "pstake_fee_address" => {
                                Ok(GeneratedField::PstakeFeeAddress)
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
            type Value = PstakeParams;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.lscosmos.v1beta1.PstakeParams")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<PstakeParams, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pstake_deposit_fee__ = None;
                let mut pstake_restake_fee__ = None;
                let mut pstake_unstake_fee__ = None;
                let mut pstake_redemption_fee__ = None;
                let mut pstake_fee_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PstakeDepositFee => {
                            if pstake_deposit_fee__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("pstakeDepositFee"),
                                );
                            }
                            pstake_deposit_fee__ = Some(map.next_value()?);
                        }
                        GeneratedField::PstakeRestakeFee => {
                            if pstake_restake_fee__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("pstakeRestakeFee"),
                                );
                            }
                            pstake_restake_fee__ = Some(map.next_value()?);
                        }
                        GeneratedField::PstakeUnstakeFee => {
                            if pstake_unstake_fee__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("pstakeUnstakeFee"),
                                );
                            }
                            pstake_unstake_fee__ = Some(map.next_value()?);
                        }
                        GeneratedField::PstakeRedemptionFee => {
                            if pstake_redemption_fee__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("pstakeRedemptionFee"),
                                );
                            }
                            pstake_redemption_fee__ = Some(map.next_value()?);
                        }
                        GeneratedField::PstakeFeeAddress => {
                            if pstake_fee_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("pstakeFeeAddress"),
                                );
                            }
                            pstake_fee_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(PstakeParams {
                    pstake_deposit_fee: pstake_deposit_fee__.unwrap_or_default(),
                    pstake_restake_fee: pstake_restake_fee__.unwrap_or_default(),
                    pstake_unstake_fee: pstake_unstake_fee__.unwrap_or_default(),
                    pstake_redemption_fee: pstake_redemption_fee__.unwrap_or_default(),
                    pstake_fee_address: pstake_fee_address__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.PstakeParams",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryAllDelegatorUnbondingEpochEntriesRequest {
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
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryAllDelegatorUnbondingEpochEntriesRequest",
                len,
            )?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllDelegatorUnbondingEpochEntriesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["delegator_address", "delegatorAddress"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
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
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAllDelegatorUnbondingEpochEntriesRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryAllDelegatorUnbondingEpochEntriesRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<
                QueryAllDelegatorUnbondingEpochEntriesRequest,
                V::Error,
            >
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
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
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryAllDelegatorUnbondingEpochEntriesRequest {
                    delegator_address: delegator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryAllDelegatorUnbondingEpochEntriesRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryAllDelegatorUnbondingEpochEntriesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.delegator_unbonding_epoch_entries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryAllDelegatorUnbondingEpochEntriesResponse",
                len,
            )?;
        if !self.delegator_unbonding_epoch_entries.is_empty() {
            struct_ser
                .serialize_field(
                    "delegatorUnbondingEpochEntries",
                    &self.delegator_unbonding_epoch_entries,
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllDelegatorUnbondingEpochEntriesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delegator_unbonding_epoch_entries",
            "delegatorUnbondingEpochEntries",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorUnbondingEpochEntries,
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
                            "delegatorUnbondingEpochEntries"
                            | "delegator_unbonding_epoch_entries" => {
                                Ok(GeneratedField::DelegatorUnbondingEpochEntries)
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
            type Value = QueryAllDelegatorUnbondingEpochEntriesResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryAllDelegatorUnbondingEpochEntriesResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<
                QueryAllDelegatorUnbondingEpochEntriesResponse,
                V::Error,
            >
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_unbonding_epoch_entries__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DelegatorUnbondingEpochEntries => {
                            if delegator_unbonding_epoch_entries__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field(
                                        "delegatorUnbondingEpochEntries",
                                    ),
                                );
                            }
                            delegator_unbonding_epoch_entries__ = Some(
                                map.next_value()?,
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryAllDelegatorUnbondingEpochEntriesResponse {
                    delegator_unbonding_epoch_entries: delegator_unbonding_epoch_entries__
                        .unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryAllDelegatorUnbondingEpochEntriesResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryAllStateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.QueryAllStateRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllStateRequest {
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
            type Value = QueryAllStateRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.lscosmos.v1beta1.QueryAllStateRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryAllStateRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryAllStateRequest {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryAllStateRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryAllStateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.genesis.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.QueryAllStateResponse", len)?;
        if let Some(v) = self.genesis.as_ref() {
            struct_ser.serialize_field("genesis", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllStateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["genesis"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Genesis,
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
                            "genesis" => Ok(GeneratedField::Genesis),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryAllStateResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.lscosmos.v1beta1.QueryAllStateResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryAllStateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut genesis__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Genesis => {
                            if genesis__.is_some() {
                                return Err(serde::de::Error::duplicate_field("genesis"));
                            }
                            genesis__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryAllStateResponse {
                    genesis: genesis__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryAllStateResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryAllowListedValidatorsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryAllowListedValidatorsRequest",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllowListedValidatorsRequest {
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
            type Value = QueryAllowListedValidatorsRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryAllowListedValidatorsRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryAllowListedValidatorsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryAllowListedValidatorsRequest {
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryAllowListedValidatorsRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryAllowListedValidatorsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.allow_listed_validators.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryAllowListedValidatorsResponse",
                len,
            )?;
        if let Some(v) = self.allow_listed_validators.as_ref() {
            struct_ser.serialize_field("allowListedValidators", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllowListedValidatorsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["allow_listed_validators", "allowListedValidators"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AllowListedValidators,
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
                            "allowListedValidators" | "allow_listed_validators" => {
                                Ok(GeneratedField::AllowListedValidators)
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
            type Value = QueryAllowListedValidatorsResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryAllowListedValidatorsResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryAllowListedValidatorsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut allow_listed_validators__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AllowListedValidators => {
                            if allow_listed_validators__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("allowListedValidators"),
                                );
                            }
                            allow_listed_validators__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryAllowListedValidatorsResponse {
                    allow_listed_validators: allow_listed_validators__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryAllowListedValidatorsResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryCValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.QueryCValueRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryCValueRequest {
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
            type Value = QueryCValueRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.lscosmos.v1beta1.QueryCValueRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryCValueRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryCValueRequest {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryCValueRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryCValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.c_value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.QueryCValueResponse", len)?;
        if !self.c_value.is_empty() {
            struct_ser.serialize_field("cValue", &self.c_value)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryCValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["c_value", "cValue"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CValue,
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
                            "cValue" | "c_value" => Ok(GeneratedField::CValue),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryCValueResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.lscosmos.v1beta1.QueryCValueResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryCValueResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut c_value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CValue => {
                            if c_value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("cValue"));
                            }
                            c_value__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryCValueResponse {
                    c_value: c_value__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryCValueResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryDelegationStateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryDelegationStateRequest",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryDelegationStateRequest {
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
            type Value = QueryDelegationStateRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryDelegationStateRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryDelegationStateRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryDelegationStateRequest {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryDelegationStateRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryDelegationStateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.delegation_state.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryDelegationStateResponse",
                len,
            )?;
        if let Some(v) = self.delegation_state.as_ref() {
            struct_ser.serialize_field("delegationState", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryDelegationStateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["delegation_state", "delegationState"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegationState,
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
                            "delegationState" | "delegation_state" => {
                                Ok(GeneratedField::DelegationState)
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
            type Value = QueryDelegationStateResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryDelegationStateResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryDelegationStateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegation_state__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DelegationState => {
                            if delegation_state__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("delegationState"),
                                );
                            }
                            delegation_state__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryDelegationStateResponse {
                    delegation_state: delegation_state__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryDelegationStateResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryDelegatorUnbondingEpochEntryRequest {
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
        if self.epoch_number != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryDelegatorUnbondingEpochEntryRequest",
                len,
            )?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        if self.epoch_number != 0 {
            struct_ser
                .serialize_field(
                    "epochNumber",
                    ToString::to_string(&self.epoch_number).as_str(),
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryDelegatorUnbondingEpochEntryRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delegator_address",
            "delegatorAddress",
            "epoch_number",
            "epochNumber",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
            EpochNumber,
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
                            "epochNumber" | "epoch_number" => {
                                Ok(GeneratedField::EpochNumber)
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
            type Value = QueryDelegatorUnbondingEpochEntryRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryDelegatorUnbondingEpochEntryRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryDelegatorUnbondingEpochEntryRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
                let mut epoch_number__ = None;
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
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryDelegatorUnbondingEpochEntryRequest {
                    delegator_address: delegator_address__.unwrap_or_default(),
                    epoch_number: epoch_number__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryDelegatorUnbondingEpochEntryRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryDelegatorUnbondingEpochEntryResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.delegator_unboding_epoch_entry.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryDelegatorUnbondingEpochEntryResponse",
                len,
            )?;
        if let Some(v) = self.delegator_unboding_epoch_entry.as_ref() {
            struct_ser.serialize_field("delegatorUnbodingEpochEntry", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryDelegatorUnbondingEpochEntryResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delegator_unboding_epoch_entry",
            "delegatorUnbodingEpochEntry",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorUnbodingEpochEntry,
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
                            "delegatorUnbodingEpochEntry"
                            | "delegator_unboding_epoch_entry" => {
                                Ok(GeneratedField::DelegatorUnbodingEpochEntry)
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
            type Value = QueryDelegatorUnbondingEpochEntryResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryDelegatorUnbondingEpochEntryResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryDelegatorUnbondingEpochEntryResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_unboding_epoch_entry__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::DelegatorUnbodingEpochEntry => {
                            if delegator_unboding_epoch_entry__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field(
                                        "delegatorUnbodingEpochEntry",
                                    ),
                                );
                            }
                            delegator_unboding_epoch_entry__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryDelegatorUnbondingEpochEntryResponse {
                    delegator_unboding_epoch_entry: delegator_unboding_epoch_entry__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryDelegatorUnbondingEpochEntryResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryDepositModuleAccountRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryDepositModuleAccountRequest",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryDepositModuleAccountRequest {
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
            type Value = QueryDepositModuleAccountRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryDepositModuleAccountRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryDepositModuleAccountRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryDepositModuleAccountRequest {
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryDepositModuleAccountRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryDepositModuleAccountResponse {
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
                "pstake.lscosmos.v1beta1.QueryDepositModuleAccountResponse",
                len,
            )?;
        if let Some(v) = self.balance.as_ref() {
            struct_ser.serialize_field("balance", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryDepositModuleAccountResponse {
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
            type Value = QueryDepositModuleAccountResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryDepositModuleAccountResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryDepositModuleAccountResponse, V::Error>
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
                Ok(QueryDepositModuleAccountResponse {
                    balance: balance__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryDepositModuleAccountResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryFailedUnbondingsRequest {
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
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryFailedUnbondingsRequest",
                len,
            )?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryFailedUnbondingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["delegator_address", "delegatorAddress"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
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
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryFailedUnbondingsRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryFailedUnbondingsRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryFailedUnbondingsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
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
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryFailedUnbondingsRequest {
                    delegator_address: delegator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryFailedUnbondingsRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryFailedUnbondingsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.failed_unbondings.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryFailedUnbondingsResponse",
                len,
            )?;
        if !self.failed_unbondings.is_empty() {
            struct_ser.serialize_field("failedUnbondings", &self.failed_unbondings)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryFailedUnbondingsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["failed_unbondings", "failedUnbondings"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FailedUnbondings,
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
                            "failedUnbondings" | "failed_unbondings" => {
                                Ok(GeneratedField::FailedUnbondings)
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
            type Value = QueryFailedUnbondingsResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryFailedUnbondingsResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryFailedUnbondingsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut failed_unbondings__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FailedUnbondings => {
                            if failed_unbondings__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("failedUnbondings"),
                                );
                            }
                            failed_unbondings__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryFailedUnbondingsResponse {
                    failed_unbondings: failed_unbondings__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryFailedUnbondingsResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryHostAccountUndelegationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.epoch_number != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryHostAccountUndelegationRequest",
                len,
            )?;
        if self.epoch_number != 0 {
            struct_ser
                .serialize_field(
                    "epochNumber",
                    ToString::to_string(&self.epoch_number).as_str(),
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryHostAccountUndelegationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["epoch_number", "epochNumber"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EpochNumber,
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
                            "epochNumber" | "epoch_number" => {
                                Ok(GeneratedField::EpochNumber)
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
            type Value = QueryHostAccountUndelegationRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryHostAccountUndelegationRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryHostAccountUndelegationRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut epoch_number__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryHostAccountUndelegationRequest {
                    epoch_number: epoch_number__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryHostAccountUndelegationRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryHostAccountUndelegationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.host_account_undelegation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryHostAccountUndelegationResponse",
                len,
            )?;
        if let Some(v) = self.host_account_undelegation.as_ref() {
            struct_ser.serialize_field("hostAccountUndelegation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryHostAccountUndelegationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "host_account_undelegation",
            "hostAccountUndelegation",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HostAccountUndelegation,
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
                            "hostAccountUndelegation" | "host_account_undelegation" => {
                                Ok(GeneratedField::HostAccountUndelegation)
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
            type Value = QueryHostAccountUndelegationResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryHostAccountUndelegationResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryHostAccountUndelegationResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut host_account_undelegation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HostAccountUndelegation => {
                            if host_account_undelegation__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("hostAccountUndelegation"),
                                );
                            }
                            host_account_undelegation__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryHostAccountUndelegationResponse {
                    host_account_undelegation: host_account_undelegation__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryHostAccountUndelegationResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryHostAccountsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.QueryHostAccountsRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryHostAccountsRequest {
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
            type Value = QueryHostAccountsRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.lscosmos.v1beta1.QueryHostAccountsRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryHostAccountsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryHostAccountsRequest {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryHostAccountsRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryHostAccountsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.host_accounts.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.QueryHostAccountsResponse", len)?;
        if let Some(v) = self.host_accounts.as_ref() {
            struct_ser.serialize_field("hostAccounts", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryHostAccountsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["host_accounts", "hostAccounts"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HostAccounts,
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
                            "hostAccounts" | "host_accounts" => {
                                Ok(GeneratedField::HostAccounts)
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
            type Value = QueryHostAccountsResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryHostAccountsResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryHostAccountsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut host_accounts__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HostAccounts => {
                            if host_accounts__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("hostAccounts"),
                                );
                            }
                            host_accounts__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryHostAccountsResponse {
                    host_accounts: host_accounts__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryHostAccountsResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryHostChainParamsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryHostChainParamsRequest",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryHostChainParamsRequest {
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
            type Value = QueryHostChainParamsRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryHostChainParamsRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryHostChainParamsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryHostChainParamsRequest {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryHostChainParamsRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryHostChainParamsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.host_chain_params.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryHostChainParamsResponse",
                len,
            )?;
        if let Some(v) = self.host_chain_params.as_ref() {
            struct_ser.serialize_field("hostChainParams", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryHostChainParamsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["host_chain_params", "hostChainParams"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            HostChainParams,
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
                            "hostChainParams" | "host_chain_params" => {
                                Ok(GeneratedField::HostChainParams)
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
            type Value = QueryHostChainParamsResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryHostChainParamsResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryHostChainParamsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut host_chain_params__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::HostChainParams => {
                            if host_chain_params__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("hostChainParams"),
                                );
                            }
                            host_chain_params__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryHostChainParamsResponse {
                    host_chain_params: host_chain_params__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryHostChainParamsResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryIbcTransientStoreRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryIBCTransientStoreRequest",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryIbcTransientStoreRequest {
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
            type Value = QueryIbcTransientStoreRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryIBCTransientStoreRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryIbcTransientStoreRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryIbcTransientStoreRequest {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryIBCTransientStoreRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryIbcTransientStoreResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.i_b_c_transient_store.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryIBCTransientStoreResponse",
                len,
            )?;
        if let Some(v) = self.i_b_c_transient_store.as_ref() {
            struct_ser.serialize_field("iBCTransientStore", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryIbcTransientStoreResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["i_b_c_transient_store", "iBCTransientStore"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            IBCTransientStore,
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
                            "iBCTransientStore" | "i_b_c_transient_store" => {
                                Ok(GeneratedField::IBCTransientStore)
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
            type Value = QueryIbcTransientStoreResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryIBCTransientStoreResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryIbcTransientStoreResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut i_b_c_transient_store__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::IBCTransientStore => {
                            if i_b_c_transient_store__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("iBCTransientStore"),
                                );
                            }
                            i_b_c_transient_store__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryIbcTransientStoreResponse {
                    i_b_c_transient_store: i_b_c_transient_store__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryIBCTransientStoreResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryModuleStateRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.QueryModuleStateRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryModuleStateRequest {
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
            type Value = QueryModuleStateRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.lscosmos.v1beta1.QueryModuleStateRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryModuleStateRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryModuleStateRequest {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryModuleStateRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryModuleStateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.module_state {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.QueryModuleStateResponse", len)?;
        if self.module_state {
            struct_ser.serialize_field("moduleState", &self.module_state)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryModuleStateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["module_state", "moduleState"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ModuleState,
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
                            "moduleState" | "module_state" => {
                                Ok(GeneratedField::ModuleState)
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
            type Value = QueryModuleStateResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.lscosmos.v1beta1.QueryModuleStateResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryModuleStateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut module_state__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ModuleState => {
                            if module_state__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("moduleState"),
                                );
                            }
                            module_state__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryModuleStateResponse {
                    module_state: module_state__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryModuleStateResponse",
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
            .serialize_struct("pstake.lscosmos.v1beta1.QueryParamsRequest", len)?;
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
                formatter.write_str("struct pstake.lscosmos.v1beta1.QueryParamsRequest")
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
                "pstake.lscosmos.v1beta1.QueryParamsRequest",
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
            .serialize_struct("pstake.lscosmos.v1beta1.QueryParamsResponse", len)?;
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
                formatter.write_str("struct pstake.lscosmos.v1beta1.QueryParamsResponse")
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
                "pstake.lscosmos.v1beta1.QueryParamsResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryPendingUnbondingsRequest {
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
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryPendingUnbondingsRequest",
                len,
            )?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryPendingUnbondingsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["delegator_address", "delegatorAddress"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
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
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryPendingUnbondingsRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryPendingUnbondingsRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryPendingUnbondingsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
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
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryPendingUnbondingsRequest {
                    delegator_address: delegator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryPendingUnbondingsRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryPendingUnbondingsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pending_unbondings.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryPendingUnbondingsResponse",
                len,
            )?;
        if !self.pending_unbondings.is_empty() {
            struct_ser.serialize_field("pendingUnbondings", &self.pending_unbondings)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryPendingUnbondingsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["pending_unbondings", "pendingUnbondings"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            PendingUnbondings,
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
                            "pendingUnbondings" | "pending_unbondings" => {
                                Ok(GeneratedField::PendingUnbondings)
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
            type Value = QueryPendingUnbondingsResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryPendingUnbondingsResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryPendingUnbondingsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pending_unbondings__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::PendingUnbondings => {
                            if pending_unbondings__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("pendingUnbondings"),
                                );
                            }
                            pending_unbondings__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryPendingUnbondingsResponse {
                    pending_unbondings: pending_unbondings__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryPendingUnbondingsResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryUnbondingEpochCValueRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.epoch_number != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryUnbondingEpochCValueRequest",
                len,
            )?;
        if self.epoch_number != 0 {
            struct_ser
                .serialize_field(
                    "epochNumber",
                    ToString::to_string(&self.epoch_number).as_str(),
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryUnbondingEpochCValueRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["epoch_number", "epochNumber"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EpochNumber,
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
                            "epochNumber" | "epoch_number" => {
                                Ok(GeneratedField::EpochNumber)
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
            type Value = QueryUnbondingEpochCValueRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryUnbondingEpochCValueRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryUnbondingEpochCValueRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut epoch_number__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryUnbondingEpochCValueRequest {
                    epoch_number: epoch_number__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryUnbondingEpochCValueRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryUnbondingEpochCValueResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.unbonding_epoch_c_value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.QueryUnbondingEpochCValueResponse",
                len,
            )?;
        if let Some(v) = self.unbonding_epoch_c_value.as_ref() {
            struct_ser.serialize_field("unbondingEpochCValue", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryUnbondingEpochCValueResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["unbonding_epoch_c_value", "unbondingEpochCValue"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UnbondingEpochCValue,
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
                            "unbondingEpochCValue" | "unbonding_epoch_c_value" => {
                                Ok(GeneratedField::UnbondingEpochCValue)
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
            type Value = QueryUnbondingEpochCValueResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.QueryUnbondingEpochCValueResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryUnbondingEpochCValueResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut unbonding_epoch_c_value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UnbondingEpochCValue => {
                            if unbonding_epoch_c_value__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("unbondingEpochCValue"),
                                );
                            }
                            unbonding_epoch_c_value__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryUnbondingEpochCValueResponse {
                    unbonding_epoch_c_value: unbonding_epoch_c_value__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryUnbondingEpochCValueResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryUnclaimedRequest {
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
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.QueryUnclaimedRequest", len)?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryUnclaimedRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["delegator_address", "delegatorAddress"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
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
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryUnclaimedRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.lscosmos.v1beta1.QueryUnclaimedRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryUnclaimedRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
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
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryUnclaimedRequest {
                    delegator_address: delegator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryUnclaimedRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryUnclaimedResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.unclaimed.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.QueryUnclaimedResponse", len)?;
        if !self.unclaimed.is_empty() {
            struct_ser.serialize_field("unclaimed", &self.unclaimed)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryUnclaimedResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["unclaimed"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Unclaimed,
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
                            "unclaimed" => Ok(GeneratedField::Unclaimed),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryUnclaimedResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.lscosmos.v1beta1.QueryUnclaimedResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryUnclaimedResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut unclaimed__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Unclaimed => {
                            if unclaimed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unclaimed"));
                            }
                            unclaimed__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryUnclaimedResponse {
                    unclaimed: unclaimed__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.QueryUnclaimedResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for TransientUndelegationTransfer {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.epoch_number != 0 {
            len += 1;
        }
        if self.amount_unbonded.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lscosmos.v1beta1.TransientUndelegationTransfer",
                len,
            )?;
        if self.epoch_number != 0 {
            struct_ser
                .serialize_field(
                    "epochNumber",
                    ToString::to_string(&self.epoch_number).as_str(),
                )?;
        }
        if let Some(v) = self.amount_unbonded.as_ref() {
            struct_ser.serialize_field("amountUnbonded", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TransientUndelegationTransfer {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "epoch_number",
            "epochNumber",
            "amount_unbonded",
            "amountUnbonded",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EpochNumber,
            AmountUnbonded,
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
                            "epochNumber" | "epoch_number" => {
                                Ok(GeneratedField::EpochNumber)
                            }
                            "amountUnbonded" | "amount_unbonded" => {
                                Ok(GeneratedField::AmountUnbonded)
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
            type Value = TransientUndelegationTransfer;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lscosmos.v1beta1.TransientUndelegationTransfer",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<TransientUndelegationTransfer, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut epoch_number__ = None;
                let mut amount_unbonded__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                        GeneratedField::AmountUnbonded => {
                            if amount_unbonded__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("amountUnbonded"),
                                );
                            }
                            amount_unbonded__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(TransientUndelegationTransfer {
                    epoch_number: epoch_number__.unwrap_or_default(),
                    amount_unbonded: amount_unbonded__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.TransientUndelegationTransfer",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for UnbondingEpochCValue {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.epoch_number != 0 {
            len += 1;
        }
        if self.s_t_k_burn.is_some() {
            len += 1;
        }
        if self.amount_unbonded.is_some() {
            len += 1;
        }
        if self.is_matured {
            len += 1;
        }
        if self.is_failed {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.UnbondingEpochCValue", len)?;
        if self.epoch_number != 0 {
            struct_ser
                .serialize_field(
                    "epochNumber",
                    ToString::to_string(&self.epoch_number).as_str(),
                )?;
        }
        if let Some(v) = self.s_t_k_burn.as_ref() {
            struct_ser.serialize_field("sTKBurn", v)?;
        }
        if let Some(v) = self.amount_unbonded.as_ref() {
            struct_ser.serialize_field("amountUnbonded", v)?;
        }
        if self.is_matured {
            struct_ser.serialize_field("isMatured", &self.is_matured)?;
        }
        if self.is_failed {
            struct_ser.serialize_field("isFailed", &self.is_failed)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnbondingEpochCValue {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "epoch_number",
            "epochNumber",
            "s_t_k_burn",
            "sTKBurn",
            "amount_unbonded",
            "amountUnbonded",
            "is_matured",
            "isMatured",
            "is_failed",
            "isFailed",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            EpochNumber,
            STKBurn,
            AmountUnbonded,
            IsMatured,
            IsFailed,
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
                            "epochNumber" | "epoch_number" => {
                                Ok(GeneratedField::EpochNumber)
                            }
                            "sTKBurn" | "s_t_k_burn" => Ok(GeneratedField::STKBurn),
                            "amountUnbonded" | "amount_unbonded" => {
                                Ok(GeneratedField::AmountUnbonded)
                            }
                            "isMatured" | "is_matured" => Ok(GeneratedField::IsMatured),
                            "isFailed" | "is_failed" => Ok(GeneratedField::IsFailed),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnbondingEpochCValue;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.lscosmos.v1beta1.UnbondingEpochCValue")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<UnbondingEpochCValue, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut epoch_number__ = None;
                let mut s_t_k_burn__ = None;
                let mut amount_unbonded__ = None;
                let mut is_matured__ = None;
                let mut is_failed__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                        GeneratedField::STKBurn => {
                            if s_t_k_burn__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sTKBurn"));
                            }
                            s_t_k_burn__ = map.next_value()?;
                        }
                        GeneratedField::AmountUnbonded => {
                            if amount_unbonded__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("amountUnbonded"),
                                );
                            }
                            amount_unbonded__ = map.next_value()?;
                        }
                        GeneratedField::IsMatured => {
                            if is_matured__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isMatured"));
                            }
                            is_matured__ = Some(map.next_value()?);
                        }
                        GeneratedField::IsFailed => {
                            if is_failed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isFailed"));
                            }
                            is_failed__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UnbondingEpochCValue {
                    epoch_number: epoch_number__.unwrap_or_default(),
                    s_t_k_burn: s_t_k_burn__,
                    amount_unbonded: amount_unbonded__,
                    is_matured: is_matured__.unwrap_or_default(),
                    is_failed: is_failed__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.UnbondingEpochCValue",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for UndelegationEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.validator_address.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lscosmos.v1beta1.UndelegationEntry", len)?;
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UndelegationEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["validator_address", "validatorAddress", "amount"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidatorAddress,
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
                            "validatorAddress" | "validator_address" => {
                                Ok(GeneratedField::ValidatorAddress)
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
            type Value = UndelegationEntry;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.lscosmos.v1beta1.UndelegationEntry")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<UndelegationEntry, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut validator_address__ = None;
                let mut amount__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UndelegationEntry {
                    validator_address: validator_address__.unwrap_or_default(),
                    amount: amount__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lscosmos.v1beta1.UndelegationEntry",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
