impl serde::Serialize for BondStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "BOND_STATUS_UNSPECIFIED",
            Self::Unbonded => "BOND_STATUS_UNBONDED",
            Self::Unbonding => "BOND_STATUS_UNBONDING",
            Self::Bonded => "BOND_STATUS_BONDED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for BondStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "BOND_STATUS_UNSPECIFIED",
            "BOND_STATUS_UNBONDED",
            "BOND_STATUS_UNBONDING",
            "BOND_STATUS_BONDED",
        ];
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = BondStatus;
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
                    .and_then(BondStatus::from_i32)
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
                    .and_then(BondStatus::from_i32)
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
                    "BOND_STATUS_UNSPECIFIED" => Ok(BondStatus::Unspecified),
                    "BOND_STATUS_UNBONDED" => Ok(BondStatus::Unbonded),
                    "BOND_STATUS_UNBONDING" => Ok(BondStatus::Unbonding),
                    "BOND_STATUS_BONDED" => Ok(BondStatus::Bonded),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Commission {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.commission_rates.is_some() {
            len += 1;
        }
        if self.update_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.Commission", len)?;
        if let Some(v) = self.commission_rates.as_ref() {
            struct_ser.serialize_field("commissionRates", v)?;
        }
        if let Some(v) = self.update_time.as_ref() {
            struct_ser.serialize_field("updateTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Commission {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "commission_rates",
            "commissionRates",
            "update_time",
            "updateTime",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CommissionRates,
            UpdateTime,
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
                            "commissionRates" | "commission_rates" => {
                                Ok(GeneratedField::CommissionRates)
                            }
                            "updateTime" | "update_time" => {
                                Ok(GeneratedField::UpdateTime)
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
            type Value = Commission;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.Commission")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<Commission, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut commission_rates__ = None;
                let mut update_time__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CommissionRates => {
                            if commission_rates__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("commissionRates"),
                                );
                            }
                            commission_rates__ = map.next_value()?;
                        }
                        GeneratedField::UpdateTime => {
                            if update_time__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updateTime"));
                            }
                            update_time__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Commission {
                    commission_rates: commission_rates__,
                    update_time: update_time__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.Commission",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for CommissionRates {
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
        if !self.max_rate.is_empty() {
            len += 1;
        }
        if !self.max_change_rate.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.CommissionRates", len)?;
        if !self.rate.is_empty() {
            struct_ser.serialize_field("rate", &self.rate)?;
        }
        if !self.max_rate.is_empty() {
            struct_ser.serialize_field("maxRate", &self.max_rate)?;
        }
        if !self.max_change_rate.is_empty() {
            struct_ser.serialize_field("maxChangeRate", &self.max_change_rate)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CommissionRates {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "rate",
            "max_rate",
            "maxRate",
            "max_change_rate",
            "maxChangeRate",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Rate,
            MaxRate,
            MaxChangeRate,
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
                            "maxRate" | "max_rate" => Ok(GeneratedField::MaxRate),
                            "maxChangeRate" | "max_change_rate" => {
                                Ok(GeneratedField::MaxChangeRate)
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
            type Value = CommissionRates;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.CommissionRates")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<CommissionRates, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut rate__ = None;
                let mut max_rate__ = None;
                let mut max_change_rate__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Rate => {
                            if rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rate"));
                            }
                            rate__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxRate => {
                            if max_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxRate"));
                            }
                            max_rate__ = Some(map.next_value()?);
                        }
                        GeneratedField::MaxChangeRate => {
                            if max_change_rate__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("maxChangeRate"),
                                );
                            }
                            max_change_rate__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(CommissionRates {
                    rate: rate__.unwrap_or_default(),
                    max_rate: max_rate__.unwrap_or_default(),
                    max_change_rate: max_change_rate__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.CommissionRates",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for DvPair {
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
        if !self.validator_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.DVPair", len)?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DvPair {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delegator_address",
            "delegatorAddress",
            "validator_address",
            "validatorAddress",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
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
                            "delegatorAddress" | "delegator_address" => {
                                Ok(GeneratedField::DelegatorAddress)
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
            type Value = DvPair;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.DVPair")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<DvPair, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
                let mut validator_address__ = None;
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
                Ok(DvPair {
                    delegator_address: delegator_address__.unwrap_or_default(),
                    validator_address: validator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.DVPair",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for DvPairs {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.pairs.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.DVPairs", len)?;
        if !self.pairs.is_empty() {
            struct_ser.serialize_field("pairs", &self.pairs)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DvPairs {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["pairs"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pairs,
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
                            "pairs" => Ok(GeneratedField::Pairs),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DvPairs;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.DVPairs")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<DvPairs, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pairs__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Pairs => {
                            if pairs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pairs"));
                            }
                            pairs__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DvPairs {
                    pairs: pairs__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.DVPairs",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for DvvTriplet {
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
        if !self.validator_src_address.is_empty() {
            len += 1;
        }
        if !self.validator_dst_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.DVVTriplet", len)?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        if !self.validator_src_address.is_empty() {
            struct_ser
                .serialize_field("validatorSrcAddress", &self.validator_src_address)?;
        }
        if !self.validator_dst_address.is_empty() {
            struct_ser
                .serialize_field("validatorDstAddress", &self.validator_dst_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DvvTriplet {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delegator_address",
            "delegatorAddress",
            "validator_src_address",
            "validatorSrcAddress",
            "validator_dst_address",
            "validatorDstAddress",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
            ValidatorSrcAddress,
            ValidatorDstAddress,
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
                            "validatorSrcAddress" | "validator_src_address" => {
                                Ok(GeneratedField::ValidatorSrcAddress)
                            }
                            "validatorDstAddress" | "validator_dst_address" => {
                                Ok(GeneratedField::ValidatorDstAddress)
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
            type Value = DvvTriplet;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.DVVTriplet")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<DvvTriplet, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
                let mut validator_src_address__ = None;
                let mut validator_dst_address__ = None;
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
                        GeneratedField::ValidatorSrcAddress => {
                            if validator_src_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorSrcAddress"),
                                );
                            }
                            validator_src_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::ValidatorDstAddress => {
                            if validator_dst_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorDstAddress"),
                                );
                            }
                            validator_dst_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DvvTriplet {
                    delegator_address: delegator_address__.unwrap_or_default(),
                    validator_src_address: validator_src_address__.unwrap_or_default(),
                    validator_dst_address: validator_dst_address__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.DVVTriplet",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for DvvTriplets {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.triplets.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.DVVTriplets", len)?;
        if !self.triplets.is_empty() {
            struct_ser.serialize_field("triplets", &self.triplets)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DvvTriplets {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["triplets"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Triplets,
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
                            "triplets" => Ok(GeneratedField::Triplets),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DvvTriplets;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.DVVTriplets")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<DvvTriplets, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut triplets__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Triplets => {
                            if triplets__.is_some() {
                                return Err(serde::de::Error::duplicate_field("triplets"));
                            }
                            triplets__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DvvTriplets {
                    triplets: triplets__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.DVVTriplets",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for Delegation {
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
        if !self.validator_address.is_empty() {
            len += 1;
        }
        if !self.shares.is_empty() {
            len += 1;
        }
        if self.validator_bond {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.Delegation", len)?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        if !self.shares.is_empty() {
            struct_ser.serialize_field("shares", &self.shares)?;
        }
        if self.validator_bond {
            struct_ser.serialize_field("validatorBond", &self.validator_bond)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Delegation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delegator_address",
            "delegatorAddress",
            "validator_address",
            "validatorAddress",
            "shares",
            "validator_bond",
            "validatorBond",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
            ValidatorAddress,
            Shares,
            ValidatorBond,
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
                            "validatorAddress" | "validator_address" => {
                                Ok(GeneratedField::ValidatorAddress)
                            }
                            "shares" => Ok(GeneratedField::Shares),
                            "validatorBond" | "validator_bond" => {
                                Ok(GeneratedField::ValidatorBond)
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
            type Value = Delegation;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.Delegation")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<Delegation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
                let mut validator_address__ = None;
                let mut shares__ = None;
                let mut validator_bond__ = None;
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
                        GeneratedField::ValidatorAddress => {
                            if validator_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorAddress"),
                                );
                            }
                            validator_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::Shares => {
                            if shares__.is_some() {
                                return Err(serde::de::Error::duplicate_field("shares"));
                            }
                            shares__ = Some(map.next_value()?);
                        }
                        GeneratedField::ValidatorBond => {
                            if validator_bond__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorBond"),
                                );
                            }
                            validator_bond__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Delegation {
                    delegator_address: delegator_address__.unwrap_or_default(),
                    validator_address: validator_address__.unwrap_or_default(),
                    shares: shares__.unwrap_or_default(),
                    validator_bond: validator_bond__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.Delegation",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for DelegationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.delegation.is_some() {
            len += 1;
        }
        if self.balance.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.DelegationResponse", len)?;
        if let Some(v) = self.delegation.as_ref() {
            struct_ser.serialize_field("delegation", v)?;
        }
        if let Some(v) = self.balance.as_ref() {
            struct_ser.serialize_field("balance", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DelegationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["delegation", "balance"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Delegation,
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
                            "delegation" => Ok(GeneratedField::Delegation),
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
            type Value = DelegationResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.DelegationResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<DelegationResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegation__ = None;
                let mut balance__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Delegation => {
                            if delegation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delegation"));
                            }
                            delegation__ = map.next_value()?;
                        }
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
                Ok(DelegationResponse {
                    delegation: delegation__,
                    balance: balance__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.DelegationResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for Description {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.moniker.is_empty() {
            len += 1;
        }
        if !self.identity.is_empty() {
            len += 1;
        }
        if !self.website.is_empty() {
            len += 1;
        }
        if !self.security_contact.is_empty() {
            len += 1;
        }
        if !self.details.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.Description", len)?;
        if !self.moniker.is_empty() {
            struct_ser.serialize_field("moniker", &self.moniker)?;
        }
        if !self.identity.is_empty() {
            struct_ser.serialize_field("identity", &self.identity)?;
        }
        if !self.website.is_empty() {
            struct_ser.serialize_field("website", &self.website)?;
        }
        if !self.security_contact.is_empty() {
            struct_ser.serialize_field("securityContact", &self.security_contact)?;
        }
        if !self.details.is_empty() {
            struct_ser.serialize_field("details", &self.details)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Description {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "moniker",
            "identity",
            "website",
            "security_contact",
            "securityContact",
            "details",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Moniker,
            Identity,
            Website,
            SecurityContact,
            Details,
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
                            "moniker" => Ok(GeneratedField::Moniker),
                            "identity" => Ok(GeneratedField::Identity),
                            "website" => Ok(GeneratedField::Website),
                            "securityContact" | "security_contact" => {
                                Ok(GeneratedField::SecurityContact)
                            }
                            "details" => Ok(GeneratedField::Details),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Description;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.Description")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<Description, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut moniker__ = None;
                let mut identity__ = None;
                let mut website__ = None;
                let mut security_contact__ = None;
                let mut details__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Moniker => {
                            if moniker__.is_some() {
                                return Err(serde::de::Error::duplicate_field("moniker"));
                            }
                            moniker__ = Some(map.next_value()?);
                        }
                        GeneratedField::Identity => {
                            if identity__.is_some() {
                                return Err(serde::de::Error::duplicate_field("identity"));
                            }
                            identity__ = Some(map.next_value()?);
                        }
                        GeneratedField::Website => {
                            if website__.is_some() {
                                return Err(serde::de::Error::duplicate_field("website"));
                            }
                            website__ = Some(map.next_value()?);
                        }
                        GeneratedField::SecurityContact => {
                            if security_contact__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("securityContact"),
                                );
                            }
                            security_contact__ = Some(map.next_value()?);
                        }
                        GeneratedField::Details => {
                            if details__.is_some() {
                                return Err(serde::de::Error::duplicate_field("details"));
                            }
                            details__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Description {
                    moniker: moniker__.unwrap_or_default(),
                    identity: identity__.unwrap_or_default(),
                    website: website__.unwrap_or_default(),
                    security_contact: security_contact__.unwrap_or_default(),
                    details: details__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.Description",
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
        if !self.last_total_power.is_empty() {
            len += 1;
        }
        if !self.last_validator_powers.is_empty() {
            len += 1;
        }
        if !self.validators.is_empty() {
            len += 1;
        }
        if !self.delegations.is_empty() {
            len += 1;
        }
        if !self.unbonding_delegations.is_empty() {
            len += 1;
        }
        if !self.redelegations.is_empty() {
            len += 1;
        }
        if self.exported {
            len += 1;
        }
        if !self.tokenize_share_records.is_empty() {
            len += 1;
        }
        if self.last_tokenize_share_record_id != 0 {
            len += 1;
        }
        if !self.total_liquid_staked_tokens.is_empty() {
            len += 1;
        }
        if !self.tokenize_share_locks.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.last_total_power.is_empty() {
            struct_ser
                .serialize_field(
                    "lastTotalPower",
                    pbjson::private::base64::encode(&self.last_total_power).as_str(),
                )?;
        }
        if !self.last_validator_powers.is_empty() {
            struct_ser
                .serialize_field("lastValidatorPowers", &self.last_validator_powers)?;
        }
        if !self.validators.is_empty() {
            struct_ser.serialize_field("validators", &self.validators)?;
        }
        if !self.delegations.is_empty() {
            struct_ser.serialize_field("delegations", &self.delegations)?;
        }
        if !self.unbonding_delegations.is_empty() {
            struct_ser
                .serialize_field("unbondingDelegations", &self.unbonding_delegations)?;
        }
        if !self.redelegations.is_empty() {
            struct_ser.serialize_field("redelegations", &self.redelegations)?;
        }
        if self.exported {
            struct_ser.serialize_field("exported", &self.exported)?;
        }
        if !self.tokenize_share_records.is_empty() {
            struct_ser
                .serialize_field("tokenizeShareRecords", &self.tokenize_share_records)?;
        }
        if self.last_tokenize_share_record_id != 0 {
            struct_ser
                .serialize_field(
                    "lastTokenizeShareRecordId",
                    ToString::to_string(&self.last_tokenize_share_record_id).as_str(),
                )?;
        }
        if !self.total_liquid_staked_tokens.is_empty() {
            struct_ser
                .serialize_field(
                    "totalLiquidStakedTokens",
                    pbjson::private::base64::encode(&self.total_liquid_staked_tokens)
                        .as_str(),
                )?;
        }
        if !self.tokenize_share_locks.is_empty() {
            struct_ser
                .serialize_field("tokenizeShareLocks", &self.tokenize_share_locks)?;
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
            "last_total_power",
            "lastTotalPower",
            "last_validator_powers",
            "lastValidatorPowers",
            "validators",
            "delegations",
            "unbonding_delegations",
            "unbondingDelegations",
            "redelegations",
            "exported",
            "tokenize_share_records",
            "tokenizeShareRecords",
            "last_tokenize_share_record_id",
            "lastTokenizeShareRecordId",
            "total_liquid_staked_tokens",
            "totalLiquidStakedTokens",
            "tokenize_share_locks",
            "tokenizeShareLocks",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            LastTotalPower,
            LastValidatorPowers,
            Validators,
            Delegations,
            UnbondingDelegations,
            Redelegations,
            Exported,
            TokenizeShareRecords,
            LastTokenizeShareRecordId,
            TotalLiquidStakedTokens,
            TokenizeShareLocks,
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
                            "lastTotalPower" | "last_total_power" => {
                                Ok(GeneratedField::LastTotalPower)
                            }
                            "lastValidatorPowers" | "last_validator_powers" => {
                                Ok(GeneratedField::LastValidatorPowers)
                            }
                            "validators" => Ok(GeneratedField::Validators),
                            "delegations" => Ok(GeneratedField::Delegations),
                            "unbondingDelegations" | "unbonding_delegations" => {
                                Ok(GeneratedField::UnbondingDelegations)
                            }
                            "redelegations" => Ok(GeneratedField::Redelegations),
                            "exported" => Ok(GeneratedField::Exported),
                            "tokenizeShareRecords" | "tokenize_share_records" => {
                                Ok(GeneratedField::TokenizeShareRecords)
                            }
                            "lastTokenizeShareRecordId"
                            | "last_tokenize_share_record_id" => {
                                Ok(GeneratedField::LastTokenizeShareRecordId)
                            }
                            "totalLiquidStakedTokens" | "total_liquid_staked_tokens" => {
                                Ok(GeneratedField::TotalLiquidStakedTokens)
                            }
                            "tokenizeShareLocks" | "tokenize_share_locks" => {
                                Ok(GeneratedField::TokenizeShareLocks)
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
                formatter.write_str("struct cosmos.staking.v1beta1.GenesisState")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut last_total_power__ = None;
                let mut last_validator_powers__ = None;
                let mut validators__ = None;
                let mut delegations__ = None;
                let mut unbonding_delegations__ = None;
                let mut redelegations__ = None;
                let mut exported__ = None;
                let mut tokenize_share_records__ = None;
                let mut last_tokenize_share_record_id__ = None;
                let mut total_liquid_staked_tokens__ = None;
                let mut tokenize_share_locks__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                        GeneratedField::LastTotalPower => {
                            if last_total_power__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("lastTotalPower"),
                                );
                            }
                            last_total_power__ = Some(
                                map
                                    .next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::LastValidatorPowers => {
                            if last_validator_powers__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("lastValidatorPowers"),
                                );
                            }
                            last_validator_powers__ = Some(map.next_value()?);
                        }
                        GeneratedField::Validators => {
                            if validators__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validators"));
                            }
                            validators__ = Some(map.next_value()?);
                        }
                        GeneratedField::Delegations => {
                            if delegations__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("delegations"),
                                );
                            }
                            delegations__ = Some(map.next_value()?);
                        }
                        GeneratedField::UnbondingDelegations => {
                            if unbonding_delegations__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("unbondingDelegations"),
                                );
                            }
                            unbonding_delegations__ = Some(map.next_value()?);
                        }
                        GeneratedField::Redelegations => {
                            if redelegations__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("redelegations"),
                                );
                            }
                            redelegations__ = Some(map.next_value()?);
                        }
                        GeneratedField::Exported => {
                            if exported__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exported"));
                            }
                            exported__ = Some(map.next_value()?);
                        }
                        GeneratedField::TokenizeShareRecords => {
                            if tokenize_share_records__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("tokenizeShareRecords"),
                                );
                            }
                            tokenize_share_records__ = Some(map.next_value()?);
                        }
                        GeneratedField::LastTokenizeShareRecordId => {
                            if last_tokenize_share_record_id__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field(
                                        "lastTokenizeShareRecordId",
                                    ),
                                );
                            }
                            last_tokenize_share_record_id__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TotalLiquidStakedTokens => {
                            if total_liquid_staked_tokens__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("totalLiquidStakedTokens"),
                                );
                            }
                            total_liquid_staked_tokens__ = Some(
                                map
                                    .next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::TokenizeShareLocks => {
                            if tokenize_share_locks__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("tokenizeShareLocks"),
                                );
                            }
                            tokenize_share_locks__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    last_total_power: last_total_power__.unwrap_or_default(),
                    last_validator_powers: last_validator_powers__.unwrap_or_default(),
                    validators: validators__.unwrap_or_default(),
                    delegations: delegations__.unwrap_or_default(),
                    unbonding_delegations: unbonding_delegations__.unwrap_or_default(),
                    redelegations: redelegations__.unwrap_or_default(),
                    exported: exported__.unwrap_or_default(),
                    tokenize_share_records: tokenize_share_records__.unwrap_or_default(),
                    last_tokenize_share_record_id: last_tokenize_share_record_id__
                        .unwrap_or_default(),
                    total_liquid_staked_tokens: total_liquid_staked_tokens__
                        .unwrap_or_default(),
                    tokenize_share_locks: tokenize_share_locks__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.GenesisState",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
// impl serde::Serialize for HistoricalInfo {
//     #[allow(deprecated)]
//     fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         use serde::ser::SerializeStruct;
//         let mut len = 0;
//         if self.header.is_some() {
//             len += 1;
//         }
//         if !self.valset.is_empty() {
//             len += 1;
//         }
//         let mut struct_ser = serializer
//             .serialize_struct("cosmos.staking.v1beta1.HistoricalInfo", len)?;
//         if let Some(v) = self.header.as_ref() {
//             struct_ser.serialize_field("header", v)?;
//         }
//         if !self.valset.is_empty() {
//             struct_ser.serialize_field("valset", &self.valset)?;
//         }
//         struct_ser.end()
//     }
// }
// impl<'de> serde::Deserialize<'de> for HistoricalInfo {
//     #[allow(deprecated)]
//     fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         const FIELDS: &[&str] = &["header", "valset"];
//         #[allow(clippy::enum_variant_names)]
//         enum GeneratedField {
//             Header,
//             Valset,
//             __SkipField__,
//         }
//         impl<'de> serde::Deserialize<'de> for GeneratedField {
//             fn deserialize<D>(
//                 deserializer: D,
//             ) -> std::result::Result<GeneratedField, D::Error>
//             where
//                 D: serde::Deserializer<'de>,
//             {
//                 struct GeneratedVisitor;
//                 impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
//                     type Value = GeneratedField;
//                     fn expecting(
//                         &self,
//                         formatter: &mut std::fmt::Formatter<'_>,
//                     ) -> std::fmt::Result {
//                         write!(formatter, "expected one of: {:?}", & FIELDS)
//                     }
//                     #[allow(unused_variables)]
//                     fn visit_str<E>(
//                         self,
//                         value: &str,
//                     ) -> std::result::Result<GeneratedField, E>
//                     where
//                         E: serde::de::Error,
//                     {
//                         match value {
//                             "header" => Ok(GeneratedField::Header),
//                             "valset" => Ok(GeneratedField::Valset),
//                             _ => Ok(GeneratedField::__SkipField__),
//                         }
//                     }
//                 }
//                 deserializer.deserialize_identifier(GeneratedVisitor)
//             }
//         }
//         struct GeneratedVisitor;
//         impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
//             type Value = HistoricalInfo;
//             fn expecting(
//                 &self,
//                 formatter: &mut std::fmt::Formatter<'_>,
//             ) -> std::fmt::Result {
//                 formatter.write_str("struct cosmos.staking.v1beta1.HistoricalInfo")
//             }
//             fn visit_map<V>(
//                 self,
//                 mut map: V,
//             ) -> std::result::Result<HistoricalInfo, V::Error>
//             where
//                 V: serde::de::MapAccess<'de>,
//             {
//                 let mut header__ = None;
//                 let mut valset__ = None;
//                 while let Some(k) = map.next_key()? {
//                     match k {
//                         GeneratedField::Header => {
//                             if header__.is_some() {
//                                 return Err(serde::de::Error::duplicate_field("header"));
//                             }
//                             header__ = map.next_value()?;
//                         }
//                         GeneratedField::Valset => {
//                             if valset__.is_some() {
//                                 return Err(serde::de::Error::duplicate_field("valset"));
//                             }
//                             valset__ = Some(map.next_value()?);
//                         }
//                         GeneratedField::__SkipField__ => {
//                             let _ = map.next_value::<serde::de::IgnoredAny>()?;
//                         }
//                     }
//                 }
//                 Ok(HistoricalInfo {
//                     header: header__,
//                     valset: valset__.unwrap_or_default(),
//                 })
//             }
//         }
//         deserializer
//             .deserialize_struct(
//                 "cosmos.staking.v1beta1.HistoricalInfo",
//                 FIELDS,
//                 GeneratedVisitor,
//             )
//     }
// }
impl serde::Serialize for Infraction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "INFRACTION_UNSPECIFIED",
            Self::DoubleSign => "INFRACTION_DOUBLE_SIGN",
            Self::Downtime => "INFRACTION_DOWNTIME",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Infraction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "INFRACTION_UNSPECIFIED",
            "INFRACTION_DOUBLE_SIGN",
            "INFRACTION_DOWNTIME",
        ];
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Infraction;
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
                    .and_then(Infraction::from_i32)
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
                    .and_then(Infraction::from_i32)
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
                    "INFRACTION_UNSPECIFIED" => Ok(Infraction::Unspecified),
                    "INFRACTION_DOUBLE_SIGN" => Ok(Infraction::DoubleSign),
                    "INFRACTION_DOWNTIME" => Ok(Infraction::Downtime),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for LastValidatorPower {
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
        if self.power != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.LastValidatorPower", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if self.power != 0 {
            struct_ser
                .serialize_field("power", ToString::to_string(&self.power).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LastValidatorPower {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["address", "power"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Power,
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
                            "power" => Ok(GeneratedField::Power),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LastValidatorPower;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.LastValidatorPower")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<LastValidatorPower, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut power__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map.next_value()?);
                        }
                        GeneratedField::Power => {
                            if power__.is_some() {
                                return Err(serde::de::Error::duplicate_field("power"));
                            }
                            power__ = Some(
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
                Ok(LastValidatorPower {
                    address: address__.unwrap_or_default(),
                    power: power__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.LastValidatorPower",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgBeginRedelegate {
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
        if !self.validator_src_address.is_empty() {
            len += 1;
        }
        if !self.validator_dst_address.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.MsgBeginRedelegate", len)?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        if !self.validator_src_address.is_empty() {
            struct_ser
                .serialize_field("validatorSrcAddress", &self.validator_src_address)?;
        }
        if !self.validator_dst_address.is_empty() {
            struct_ser
                .serialize_field("validatorDstAddress", &self.validator_dst_address)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgBeginRedelegate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delegator_address",
            "delegatorAddress",
            "validator_src_address",
            "validatorSrcAddress",
            "validator_dst_address",
            "validatorDstAddress",
            "amount",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
            ValidatorSrcAddress,
            ValidatorDstAddress,
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
                            "validatorSrcAddress" | "validator_src_address" => {
                                Ok(GeneratedField::ValidatorSrcAddress)
                            }
                            "validatorDstAddress" | "validator_dst_address" => {
                                Ok(GeneratedField::ValidatorDstAddress)
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
            type Value = MsgBeginRedelegate;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.MsgBeginRedelegate")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgBeginRedelegate, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
                let mut validator_src_address__ = None;
                let mut validator_dst_address__ = None;
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
                        GeneratedField::ValidatorSrcAddress => {
                            if validator_src_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorSrcAddress"),
                                );
                            }
                            validator_src_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::ValidatorDstAddress => {
                            if validator_dst_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorDstAddress"),
                                );
                            }
                            validator_dst_address__ = Some(map.next_value()?);
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
                Ok(MsgBeginRedelegate {
                    delegator_address: delegator_address__.unwrap_or_default(),
                    validator_src_address: validator_src_address__.unwrap_or_default(),
                    validator_dst_address: validator_dst_address__.unwrap_or_default(),
                    amount: amount__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgBeginRedelegate",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgBeginRedelegateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.completion_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.MsgBeginRedelegateResponse", len)?;
        if let Some(v) = self.completion_time.as_ref() {
            struct_ser.serialize_field("completionTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgBeginRedelegateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["completion_time", "completionTime"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CompletionTime,
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
                            "completionTime" | "completion_time" => {
                                Ok(GeneratedField::CompletionTime)
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
            type Value = MsgBeginRedelegateResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct cosmos.staking.v1beta1.MsgBeginRedelegateResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgBeginRedelegateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut completion_time__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CompletionTime => {
                            if completion_time__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("completionTime"),
                                );
                            }
                            completion_time__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgBeginRedelegateResponse {
                    completion_time: completion_time__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgBeginRedelegateResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgCancelUnbondingDelegation {
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
        if !self.validator_address.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        if self.creation_height != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "cosmos.staking.v1beta1.MsgCancelUnbondingDelegation",
                len,
            )?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        if self.creation_height != 0 {
            struct_ser
                .serialize_field(
                    "creationHeight",
                    ToString::to_string(&self.creation_height).as_str(),
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCancelUnbondingDelegation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delegator_address",
            "delegatorAddress",
            "validator_address",
            "validatorAddress",
            "amount",
            "creation_height",
            "creationHeight",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
            ValidatorAddress,
            Amount,
            CreationHeight,
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
                            "validatorAddress" | "validator_address" => {
                                Ok(GeneratedField::ValidatorAddress)
                            }
                            "amount" => Ok(GeneratedField::Amount),
                            "creationHeight" | "creation_height" => {
                                Ok(GeneratedField::CreationHeight)
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
            type Value = MsgCancelUnbondingDelegation;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct cosmos.staking.v1beta1.MsgCancelUnbondingDelegation",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgCancelUnbondingDelegation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
                let mut validator_address__ = None;
                let mut amount__ = None;
                let mut creation_height__ = None;
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
                        GeneratedField::CreationHeight => {
                            if creation_height__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("creationHeight"),
                                );
                            }
                            creation_height__ = Some(
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
                Ok(MsgCancelUnbondingDelegation {
                    delegator_address: delegator_address__.unwrap_or_default(),
                    validator_address: validator_address__.unwrap_or_default(),
                    amount: amount__,
                    creation_height: creation_height__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgCancelUnbondingDelegation",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgCancelUnbondingDelegationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "cosmos.staking.v1beta1.MsgCancelUnbondingDelegationResponse",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCancelUnbondingDelegationResponse {
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
            type Value = MsgCancelUnbondingDelegationResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct cosmos.staking.v1beta1.MsgCancelUnbondingDelegationResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgCancelUnbondingDelegationResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgCancelUnbondingDelegationResponse {
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgCancelUnbondingDelegationResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgCreateValidator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.description.is_some() {
            len += 1;
        }
        if self.commission.is_some() {
            len += 1;
        }
        if !self.min_self_delegation.is_empty() {
            len += 1;
        }
        if !self.delegator_address.is_empty() {
            len += 1;
        }
        if !self.validator_address.is_empty() {
            len += 1;
        }
        if self.pubkey.is_some() {
            len += 1;
        }
        if self.value.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.MsgCreateValidator", len)?;
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if let Some(v) = self.commission.as_ref() {
            struct_ser.serialize_field("commission", v)?;
        }
        if !self.min_self_delegation.is_empty() {
            struct_ser.serialize_field("minSelfDelegation", &self.min_self_delegation)?;
        }
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        if let Some(v) = self.pubkey.as_ref() {
            struct_ser.serialize_field("pubkey", v)?;
        }
        if let Some(v) = self.value.as_ref() {
            struct_ser.serialize_field("value", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCreateValidator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "description",
            "commission",
            "min_self_delegation",
            "minSelfDelegation",
            "delegator_address",
            "delegatorAddress",
            "validator_address",
            "validatorAddress",
            "pubkey",
            "value",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Description,
            Commission,
            MinSelfDelegation,
            DelegatorAddress,
            ValidatorAddress,
            Pubkey,
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
                            "description" => Ok(GeneratedField::Description),
                            "commission" => Ok(GeneratedField::Commission),
                            "minSelfDelegation" | "min_self_delegation" => {
                                Ok(GeneratedField::MinSelfDelegation)
                            }
                            "delegatorAddress" | "delegator_address" => {
                                Ok(GeneratedField::DelegatorAddress)
                            }
                            "validatorAddress" | "validator_address" => {
                                Ok(GeneratedField::ValidatorAddress)
                            }
                            "pubkey" => Ok(GeneratedField::Pubkey),
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
            type Value = MsgCreateValidator;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.MsgCreateValidator")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgCreateValidator, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut description__ = None;
                let mut commission__ = None;
                let mut min_self_delegation__ = None;
                let mut delegator_address__ = None;
                let mut validator_address__ = None;
                let mut pubkey__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("description"),
                                );
                            }
                            description__ = map.next_value()?;
                        }
                        GeneratedField::Commission => {
                            if commission__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commission"));
                            }
                            commission__ = map.next_value()?;
                        }
                        GeneratedField::MinSelfDelegation => {
                            if min_self_delegation__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("minSelfDelegation"),
                                );
                            }
                            min_self_delegation__ = Some(map.next_value()?);
                        }
                        GeneratedField::DelegatorAddress => {
                            if delegator_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("delegatorAddress"),
                                );
                            }
                            delegator_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::ValidatorAddress => {
                            if validator_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorAddress"),
                                );
                            }
                            validator_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pubkey => {
                            if pubkey__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pubkey"));
                            }
                            pubkey__ = map.next_value()?;
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgCreateValidator {
                    description: description__,
                    commission: commission__,
                    min_self_delegation: min_self_delegation__.unwrap_or_default(),
                    delegator_address: delegator_address__.unwrap_or_default(),
                    validator_address: validator_address__.unwrap_or_default(),
                    pubkey: pubkey__,
                    value: value__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgCreateValidator",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgCreateValidatorResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.MsgCreateValidatorResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgCreateValidatorResponse {
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
            type Value = MsgCreateValidatorResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct cosmos.staking.v1beta1.MsgCreateValidatorResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgCreateValidatorResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgCreateValidatorResponse {})
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgCreateValidatorResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgDelegate {
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
        if !self.validator_address.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.MsgDelegate", len)?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDelegate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delegator_address",
            "delegatorAddress",
            "validator_address",
            "validatorAddress",
            "amount",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
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
                            "delegatorAddress" | "delegator_address" => {
                                Ok(GeneratedField::DelegatorAddress)
                            }
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
            type Value = MsgDelegate;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.MsgDelegate")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgDelegate, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
                let mut validator_address__ = None;
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
                Ok(MsgDelegate {
                    delegator_address: delegator_address__.unwrap_or_default(),
                    validator_address: validator_address__.unwrap_or_default(),
                    amount: amount__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgDelegate",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgDelegateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.MsgDelegateResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDelegateResponse {
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
            type Value = MsgDelegateResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.MsgDelegateResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgDelegateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgDelegateResponse {})
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgDelegateResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgDisableTokenizeShares {
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
            .serialize_struct("cosmos.staking.v1beta1.MsgDisableTokenizeShares", len)?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDisableTokenizeShares {
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
            type Value = MsgDisableTokenizeShares;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct cosmos.staking.v1beta1.MsgDisableTokenizeShares")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgDisableTokenizeShares, V::Error>
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
                Ok(MsgDisableTokenizeShares {
                    delegator_address: delegator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgDisableTokenizeShares",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgDisableTokenizeSharesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "cosmos.staking.v1beta1.MsgDisableTokenizeSharesResponse",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDisableTokenizeSharesResponse {
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
            type Value = MsgDisableTokenizeSharesResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct cosmos.staking.v1beta1.MsgDisableTokenizeSharesResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgDisableTokenizeSharesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgDisableTokenizeSharesResponse {
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgDisableTokenizeSharesResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgEditValidator {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.description.is_some() {
            len += 1;
        }
        if !self.validator_address.is_empty() {
            len += 1;
        }
        if !self.commission_rate.is_empty() {
            len += 1;
        }
        if !self.min_self_delegation.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.MsgEditValidator", len)?;
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        if !self.commission_rate.is_empty() {
            struct_ser.serialize_field("commissionRate", &self.commission_rate)?;
        }
        if !self.min_self_delegation.is_empty() {
            struct_ser.serialize_field("minSelfDelegation", &self.min_self_delegation)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgEditValidator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "description",
            "validator_address",
            "validatorAddress",
            "commission_rate",
            "commissionRate",
            "min_self_delegation",
            "minSelfDelegation",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Description,
            ValidatorAddress,
            CommissionRate,
            MinSelfDelegation,
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
                            "description" => Ok(GeneratedField::Description),
                            "validatorAddress" | "validator_address" => {
                                Ok(GeneratedField::ValidatorAddress)
                            }
                            "commissionRate" | "commission_rate" => {
                                Ok(GeneratedField::CommissionRate)
                            }
                            "minSelfDelegation" | "min_self_delegation" => {
                                Ok(GeneratedField::MinSelfDelegation)
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
            type Value = MsgEditValidator;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.MsgEditValidator")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgEditValidator, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut description__ = None;
                let mut validator_address__ = None;
                let mut commission_rate__ = None;
                let mut min_self_delegation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("description"),
                                );
                            }
                            description__ = map.next_value()?;
                        }
                        GeneratedField::ValidatorAddress => {
                            if validator_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorAddress"),
                                );
                            }
                            validator_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::CommissionRate => {
                            if commission_rate__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("commissionRate"),
                                );
                            }
                            commission_rate__ = Some(map.next_value()?);
                        }
                        GeneratedField::MinSelfDelegation => {
                            if min_self_delegation__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("minSelfDelegation"),
                                );
                            }
                            min_self_delegation__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgEditValidator {
                    description: description__,
                    validator_address: validator_address__.unwrap_or_default(),
                    commission_rate: commission_rate__.unwrap_or_default(),
                    min_self_delegation: min_self_delegation__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgEditValidator",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgEditValidatorResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.MsgEditValidatorResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgEditValidatorResponse {
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
            type Value = MsgEditValidatorResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct cosmos.staking.v1beta1.MsgEditValidatorResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgEditValidatorResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgEditValidatorResponse {})
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgEditValidatorResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgEnableTokenizeShares {
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
            .serialize_struct("cosmos.staking.v1beta1.MsgEnableTokenizeShares", len)?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgEnableTokenizeShares {
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
            type Value = MsgEnableTokenizeShares;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct cosmos.staking.v1beta1.MsgEnableTokenizeShares")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgEnableTokenizeShares, V::Error>
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
                Ok(MsgEnableTokenizeShares {
                    delegator_address: delegator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgEnableTokenizeShares",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgEnableTokenizeSharesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.completion_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "cosmos.staking.v1beta1.MsgEnableTokenizeSharesResponse",
                len,
            )?;
        if let Some(v) = self.completion_time.as_ref() {
            struct_ser.serialize_field("completionTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgEnableTokenizeSharesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["completion_time", "completionTime"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CompletionTime,
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
                            "completionTime" | "completion_time" => {
                                Ok(GeneratedField::CompletionTime)
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
            type Value = MsgEnableTokenizeSharesResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct cosmos.staking.v1beta1.MsgEnableTokenizeSharesResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgEnableTokenizeSharesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut completion_time__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CompletionTime => {
                            if completion_time__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("completionTime"),
                                );
                            }
                            completion_time__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgEnableTokenizeSharesResponse {
                    completion_time: completion_time__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgEnableTokenizeSharesResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgRedeemTokensForShares {
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
            .serialize_struct("cosmos.staking.v1beta1.MsgRedeemTokensForShares", len)?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRedeemTokensForShares {
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
            type Value = MsgRedeemTokensForShares;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct cosmos.staking.v1beta1.MsgRedeemTokensForShares")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgRedeemTokensForShares, V::Error>
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
                Ok(MsgRedeemTokensForShares {
                    delegator_address: delegator_address__.unwrap_or_default(),
                    amount: amount__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgRedeemTokensForShares",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgRedeemTokensForSharesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "cosmos.staking.v1beta1.MsgRedeemTokensForSharesResponse",
                len,
            )?;
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgRedeemTokensForSharesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["amount"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = MsgRedeemTokensForSharesResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct cosmos.staking.v1beta1.MsgRedeemTokensForSharesResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgRedeemTokensForSharesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut amount__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                Ok(MsgRedeemTokensForSharesResponse {
                    amount: amount__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgRedeemTokensForSharesResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgTokenizeShares {
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
        if !self.validator_address.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        if !self.tokenized_share_owner.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.MsgTokenizeShares", len)?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        if !self.tokenized_share_owner.is_empty() {
            struct_ser
                .serialize_field("tokenizedShareOwner", &self.tokenized_share_owner)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgTokenizeShares {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delegator_address",
            "delegatorAddress",
            "validator_address",
            "validatorAddress",
            "amount",
            "tokenized_share_owner",
            "tokenizedShareOwner",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
            ValidatorAddress,
            Amount,
            TokenizedShareOwner,
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
                            "validatorAddress" | "validator_address" => {
                                Ok(GeneratedField::ValidatorAddress)
                            }
                            "amount" => Ok(GeneratedField::Amount),
                            "tokenizedShareOwner" | "tokenized_share_owner" => {
                                Ok(GeneratedField::TokenizedShareOwner)
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
            type Value = MsgTokenizeShares;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.MsgTokenizeShares")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgTokenizeShares, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
                let mut validator_address__ = None;
                let mut amount__ = None;
                let mut tokenized_share_owner__ = None;
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
                        GeneratedField::TokenizedShareOwner => {
                            if tokenized_share_owner__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("tokenizedShareOwner"),
                                );
                            }
                            tokenized_share_owner__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgTokenizeShares {
                    delegator_address: delegator_address__.unwrap_or_default(),
                    validator_address: validator_address__.unwrap_or_default(),
                    amount: amount__,
                    tokenized_share_owner: tokenized_share_owner__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgTokenizeShares",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgTokenizeSharesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.MsgTokenizeSharesResponse", len)?;
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgTokenizeSharesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["amount"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = MsgTokenizeSharesResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct cosmos.staking.v1beta1.MsgTokenizeSharesResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgTokenizeSharesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut amount__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
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
                Ok(MsgTokenizeSharesResponse {
                    amount: amount__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgTokenizeSharesResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgTransferTokenizeShareRecord {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.tokenize_share_record_id != 0 {
            len += 1;
        }
        if !self.sender.is_empty() {
            len += 1;
        }
        if !self.new_owner.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "cosmos.staking.v1beta1.MsgTransferTokenizeShareRecord",
                len,
            )?;
        if self.tokenize_share_record_id != 0 {
            struct_ser
                .serialize_field(
                    "tokenizeShareRecordId",
                    ToString::to_string(&self.tokenize_share_record_id).as_str(),
                )?;
        }
        if !self.sender.is_empty() {
            struct_ser.serialize_field("sender", &self.sender)?;
        }
        if !self.new_owner.is_empty() {
            struct_ser.serialize_field("newOwner", &self.new_owner)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgTransferTokenizeShareRecord {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "tokenize_share_record_id",
            "tokenizeShareRecordId",
            "sender",
            "new_owner",
            "newOwner",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            TokenizeShareRecordId,
            Sender,
            NewOwner,
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
                            "tokenizeShareRecordId" | "tokenize_share_record_id" => {
                                Ok(GeneratedField::TokenizeShareRecordId)
                            }
                            "sender" => Ok(GeneratedField::Sender),
                            "newOwner" | "new_owner" => Ok(GeneratedField::NewOwner),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgTransferTokenizeShareRecord;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct cosmos.staking.v1beta1.MsgTransferTokenizeShareRecord",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgTransferTokenizeShareRecord, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut tokenize_share_record_id__ = None;
                let mut sender__ = None;
                let mut new_owner__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::TokenizeShareRecordId => {
                            if tokenize_share_record_id__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("tokenizeShareRecordId"),
                                );
                            }
                            tokenize_share_record_id__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Sender => {
                            if sender__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sender"));
                            }
                            sender__ = Some(map.next_value()?);
                        }
                        GeneratedField::NewOwner => {
                            if new_owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("newOwner"));
                            }
                            new_owner__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgTransferTokenizeShareRecord {
                    tokenize_share_record_id: tokenize_share_record_id__
                        .unwrap_or_default(),
                    sender: sender__.unwrap_or_default(),
                    new_owner: new_owner__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgTransferTokenizeShareRecord",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgTransferTokenizeShareRecordResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "cosmos.staking.v1beta1.MsgTransferTokenizeShareRecordResponse",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgTransferTokenizeShareRecordResponse {
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
            type Value = MsgTransferTokenizeShareRecordResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct cosmos.staking.v1beta1.MsgTransferTokenizeShareRecordResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgTransferTokenizeShareRecordResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgTransferTokenizeShareRecordResponse {
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgTransferTokenizeShareRecordResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgUnbondValidator {
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
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.MsgUnbondValidator", len)?;
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnbondValidator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["validator_address", "validatorAddress"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = MsgUnbondValidator;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.MsgUnbondValidator")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgUnbondValidator, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut validator_address__ = None;
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
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgUnbondValidator {
                    validator_address: validator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgUnbondValidator",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgUnbondValidatorResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.MsgUnbondValidatorResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUnbondValidatorResponse {
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
            type Value = MsgUnbondValidatorResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct cosmos.staking.v1beta1.MsgUnbondValidatorResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgUnbondValidatorResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgUnbondValidatorResponse {})
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgUnbondValidatorResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgUndelegate {
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
        if !self.validator_address.is_empty() {
            len += 1;
        }
        if self.amount.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.MsgUndelegate", len)?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        if let Some(v) = self.amount.as_ref() {
            struct_ser.serialize_field("amount", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUndelegate {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delegator_address",
            "delegatorAddress",
            "validator_address",
            "validatorAddress",
            "amount",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
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
                            "delegatorAddress" | "delegator_address" => {
                                Ok(GeneratedField::DelegatorAddress)
                            }
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
            type Value = MsgUndelegate;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.MsgUndelegate")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgUndelegate, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
                let mut validator_address__ = None;
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
                Ok(MsgUndelegate {
                    delegator_address: delegator_address__.unwrap_or_default(),
                    validator_address: validator_address__.unwrap_or_default(),
                    amount: amount__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgUndelegate",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgUndelegateResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.completion_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.MsgUndelegateResponse", len)?;
        if let Some(v) = self.completion_time.as_ref() {
            struct_ser.serialize_field("completionTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgUndelegateResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["completion_time", "completionTime"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CompletionTime,
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
                            "completionTime" | "completion_time" => {
                                Ok(GeneratedField::CompletionTime)
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
            type Value = MsgUndelegateResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct cosmos.staking.v1beta1.MsgUndelegateResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgUndelegateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut completion_time__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CompletionTime => {
                            if completion_time__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("completionTime"),
                                );
                            }
                            completion_time__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgUndelegateResponse {
                    completion_time: completion_time__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgUndelegateResponse",
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
            .serialize_struct("cosmos.staking.v1beta1.MsgUpdateParams", len)?;
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
                formatter.write_str("struct cosmos.staking.v1beta1.MsgUpdateParams")
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
                "cosmos.staking.v1beta1.MsgUpdateParams",
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
            .serialize_struct("cosmos.staking.v1beta1.MsgUpdateParamsResponse", len)?;
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
                    .write_str("struct cosmos.staking.v1beta1.MsgUpdateParamsResponse")
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
                "cosmos.staking.v1beta1.MsgUpdateParamsResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgValidatorBond {
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
        if !self.validator_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.MsgValidatorBond", len)?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgValidatorBond {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delegator_address",
            "delegatorAddress",
            "validator_address",
            "validatorAddress",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
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
                            "delegatorAddress" | "delegator_address" => {
                                Ok(GeneratedField::DelegatorAddress)
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
            type Value = MsgValidatorBond;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.MsgValidatorBond")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgValidatorBond, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
                let mut validator_address__ = None;
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
                Ok(MsgValidatorBond {
                    delegator_address: delegator_address__.unwrap_or_default(),
                    validator_address: validator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgValidatorBond",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgValidatorBondResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.MsgValidatorBondResponse", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgValidatorBondResponse {
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
            type Value = MsgValidatorBondResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct cosmos.staking.v1beta1.MsgValidatorBondResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgValidatorBondResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgValidatorBondResponse {})
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.MsgValidatorBondResponse",
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
        if self.unbonding_time.is_some() {
            len += 1;
        }
        if self.max_validators != 0 {
            len += 1;
        }
        if self.max_entries != 0 {
            len += 1;
        }
        if self.historical_entries != 0 {
            len += 1;
        }
        if !self.bond_denom.is_empty() {
            len += 1;
        }
        if !self.min_commission_rate.is_empty() {
            len += 1;
        }
        if !self.validator_bond_factor.is_empty() {
            len += 1;
        }
        if !self.global_liquid_staking_cap.is_empty() {
            len += 1;
        }
        if !self.validator_liquid_staking_cap.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.Params", len)?;
        if let Some(v) = self.unbonding_time.as_ref() {
            struct_ser.serialize_field("unbondingTime", v)?;
        }
        if self.max_validators != 0 {
            struct_ser.serialize_field("maxValidators", &self.max_validators)?;
        }
        if self.max_entries != 0 {
            struct_ser.serialize_field("maxEntries", &self.max_entries)?;
        }
        if self.historical_entries != 0 {
            struct_ser.serialize_field("historicalEntries", &self.historical_entries)?;
        }
        if !self.bond_denom.is_empty() {
            struct_ser.serialize_field("bondDenom", &self.bond_denom)?;
        }
        if !self.min_commission_rate.is_empty() {
            struct_ser.serialize_field("minCommissionRate", &self.min_commission_rate)?;
        }
        if !self.validator_bond_factor.is_empty() {
            struct_ser
                .serialize_field("validatorBondFactor", &self.validator_bond_factor)?;
        }
        if !self.global_liquid_staking_cap.is_empty() {
            struct_ser
                .serialize_field(
                    "globalLiquidStakingCap",
                    &self.global_liquid_staking_cap,
                )?;
        }
        if !self.validator_liquid_staking_cap.is_empty() {
            struct_ser
                .serialize_field(
                    "validatorLiquidStakingCap",
                    &self.validator_liquid_staking_cap,
                )?;
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
            "unbonding_time",
            "unbondingTime",
            "max_validators",
            "maxValidators",
            "max_entries",
            "maxEntries",
            "historical_entries",
            "historicalEntries",
            "bond_denom",
            "bondDenom",
            "min_commission_rate",
            "minCommissionRate",
            "validator_bond_factor",
            "validatorBondFactor",
            "global_liquid_staking_cap",
            "globalLiquidStakingCap",
            "validator_liquid_staking_cap",
            "validatorLiquidStakingCap",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            UnbondingTime,
            MaxValidators,
            MaxEntries,
            HistoricalEntries,
            BondDenom,
            MinCommissionRate,
            ValidatorBondFactor,
            GlobalLiquidStakingCap,
            ValidatorLiquidStakingCap,
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
                            "unbondingTime" | "unbonding_time" => {
                                Ok(GeneratedField::UnbondingTime)
                            }
                            "maxValidators" | "max_validators" => {
                                Ok(GeneratedField::MaxValidators)
                            }
                            "maxEntries" | "max_entries" => {
                                Ok(GeneratedField::MaxEntries)
                            }
                            "historicalEntries" | "historical_entries" => {
                                Ok(GeneratedField::HistoricalEntries)
                            }
                            "bondDenom" | "bond_denom" => Ok(GeneratedField::BondDenom),
                            "minCommissionRate" | "min_commission_rate" => {
                                Ok(GeneratedField::MinCommissionRate)
                            }
                            "validatorBondFactor" | "validator_bond_factor" => {
                                Ok(GeneratedField::ValidatorBondFactor)
                            }
                            "globalLiquidStakingCap" | "global_liquid_staking_cap" => {
                                Ok(GeneratedField::GlobalLiquidStakingCap)
                            }
                            "validatorLiquidStakingCap"
                            | "validator_liquid_staking_cap" => {
                                Ok(GeneratedField::ValidatorLiquidStakingCap)
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
                formatter.write_str("struct cosmos.staking.v1beta1.Params")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut unbonding_time__ = None;
                let mut max_validators__ = None;
                let mut max_entries__ = None;
                let mut historical_entries__ = None;
                let mut bond_denom__ = None;
                let mut min_commission_rate__ = None;
                let mut validator_bond_factor__ = None;
                let mut global_liquid_staking_cap__ = None;
                let mut validator_liquid_staking_cap__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::UnbondingTime => {
                            if unbonding_time__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("unbondingTime"),
                                );
                            }
                            unbonding_time__ = map.next_value()?;
                        }
                        GeneratedField::MaxValidators => {
                            if max_validators__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("maxValidators"),
                                );
                            }
                            max_validators__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MaxEntries => {
                            if max_entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxEntries"));
                            }
                            max_entries__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::HistoricalEntries => {
                            if historical_entries__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("historicalEntries"),
                                );
                            }
                            historical_entries__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::BondDenom => {
                            if bond_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("bondDenom"));
                            }
                            bond_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::MinCommissionRate => {
                            if min_commission_rate__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("minCommissionRate"),
                                );
                            }
                            min_commission_rate__ = Some(map.next_value()?);
                        }
                        GeneratedField::ValidatorBondFactor => {
                            if validator_bond_factor__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorBondFactor"),
                                );
                            }
                            validator_bond_factor__ = Some(map.next_value()?);
                        }
                        GeneratedField::GlobalLiquidStakingCap => {
                            if global_liquid_staking_cap__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("globalLiquidStakingCap"),
                                );
                            }
                            global_liquid_staking_cap__ = Some(map.next_value()?);
                        }
                        GeneratedField::ValidatorLiquidStakingCap => {
                            if validator_liquid_staking_cap__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field(
                                        "validatorLiquidStakingCap",
                                    ),
                                );
                            }
                            validator_liquid_staking_cap__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Params {
                    unbonding_time: unbonding_time__,
                    max_validators: max_validators__.unwrap_or_default(),
                    max_entries: max_entries__.unwrap_or_default(),
                    historical_entries: historical_entries__.unwrap_or_default(),
                    bond_denom: bond_denom__.unwrap_or_default(),
                    min_commission_rate: min_commission_rate__.unwrap_or_default(),
                    validator_bond_factor: validator_bond_factor__.unwrap_or_default(),
                    global_liquid_staking_cap: global_liquid_staking_cap__
                        .unwrap_or_default(),
                    validator_liquid_staking_cap: validator_liquid_staking_cap__
                        .unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.Params",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for PendingTokenizeShareAuthorizations {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.addresses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "cosmos.staking.v1beta1.PendingTokenizeShareAuthorizations",
                len,
            )?;
        if !self.addresses.is_empty() {
            struct_ser.serialize_field("addresses", &self.addresses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PendingTokenizeShareAuthorizations {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["addresses"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Addresses,
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
                            "addresses" => Ok(GeneratedField::Addresses),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PendingTokenizeShareAuthorizations;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct cosmos.staking.v1beta1.PendingTokenizeShareAuthorizations",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<PendingTokenizeShareAuthorizations, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut addresses__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Addresses => {
                            if addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addresses"));
                            }
                            addresses__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(PendingTokenizeShareAuthorizations {
                    addresses: addresses__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.PendingTokenizeShareAuthorizations",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for Pool {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.not_bonded_tokens.is_empty() {
            len += 1;
        }
        if !self.bonded_tokens.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.Pool", len)?;
        if !self.not_bonded_tokens.is_empty() {
            struct_ser.serialize_field("notBondedTokens", &self.not_bonded_tokens)?;
        }
        if !self.bonded_tokens.is_empty() {
            struct_ser.serialize_field("bondedTokens", &self.bonded_tokens)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Pool {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "not_bonded_tokens",
            "notBondedTokens",
            "bonded_tokens",
            "bondedTokens",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NotBondedTokens,
            BondedTokens,
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
                            "notBondedTokens" | "not_bonded_tokens" => {
                                Ok(GeneratedField::NotBondedTokens)
                            }
                            "bondedTokens" | "bonded_tokens" => {
                                Ok(GeneratedField::BondedTokens)
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
            type Value = Pool;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.Pool")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Pool, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut not_bonded_tokens__ = None;
                let mut bonded_tokens__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::NotBondedTokens => {
                            if not_bonded_tokens__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("notBondedTokens"),
                                );
                            }
                            not_bonded_tokens__ = Some(map.next_value()?);
                        }
                        GeneratedField::BondedTokens => {
                            if bonded_tokens__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("bondedTokens"),
                                );
                            }
                            bonded_tokens__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Pool {
                    not_bonded_tokens: not_bonded_tokens__.unwrap_or_default(),
                    bonded_tokens: bonded_tokens__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct("cosmos.staking.v1beta1.Pool", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Redelegation {
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
        if !self.validator_src_address.is_empty() {
            len += 1;
        }
        if !self.validator_dst_address.is_empty() {
            len += 1;
        }
        if !self.entries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.Redelegation", len)?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        if !self.validator_src_address.is_empty() {
            struct_ser
                .serialize_field("validatorSrcAddress", &self.validator_src_address)?;
        }
        if !self.validator_dst_address.is_empty() {
            struct_ser
                .serialize_field("validatorDstAddress", &self.validator_dst_address)?;
        }
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Redelegation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delegator_address",
            "delegatorAddress",
            "validator_src_address",
            "validatorSrcAddress",
            "validator_dst_address",
            "validatorDstAddress",
            "entries",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
            ValidatorSrcAddress,
            ValidatorDstAddress,
            Entries,
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
                            "validatorSrcAddress" | "validator_src_address" => {
                                Ok(GeneratedField::ValidatorSrcAddress)
                            }
                            "validatorDstAddress" | "validator_dst_address" => {
                                Ok(GeneratedField::ValidatorDstAddress)
                            }
                            "entries" => Ok(GeneratedField::Entries),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Redelegation;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.Redelegation")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<Redelegation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
                let mut validator_src_address__ = None;
                let mut validator_dst_address__ = None;
                let mut entries__ = None;
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
                        GeneratedField::ValidatorSrcAddress => {
                            if validator_src_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorSrcAddress"),
                                );
                            }
                            validator_src_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::ValidatorDstAddress => {
                            if validator_dst_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorDstAddress"),
                                );
                            }
                            validator_dst_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Redelegation {
                    delegator_address: delegator_address__.unwrap_or_default(),
                    validator_src_address: validator_src_address__.unwrap_or_default(),
                    validator_dst_address: validator_dst_address__.unwrap_or_default(),
                    entries: entries__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.Redelegation",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for RedelegationEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.creation_height != 0 {
            len += 1;
        }
        if self.completion_time.is_some() {
            len += 1;
        }
        if !self.initial_balance.is_empty() {
            len += 1;
        }
        if !self.shares_dst.is_empty() {
            len += 1;
        }
        if self.unbonding_id != 0 {
            len += 1;
        }
        if self.unbonding_on_hold_ref_count != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.RedelegationEntry", len)?;
        if self.creation_height != 0 {
            struct_ser
                .serialize_field(
                    "creationHeight",
                    ToString::to_string(&self.creation_height).as_str(),
                )?;
        }
        if let Some(v) = self.completion_time.as_ref() {
            struct_ser.serialize_field("completionTime", v)?;
        }
        if !self.initial_balance.is_empty() {
            struct_ser.serialize_field("initialBalance", &self.initial_balance)?;
        }
        if !self.shares_dst.is_empty() {
            struct_ser.serialize_field("sharesDst", &self.shares_dst)?;
        }
        if self.unbonding_id != 0 {
            struct_ser
                .serialize_field(
                    "unbondingId",
                    ToString::to_string(&self.unbonding_id).as_str(),
                )?;
        }
        if self.unbonding_on_hold_ref_count != 0 {
            struct_ser
                .serialize_field(
                    "unbondingOnHoldRefCount",
                    ToString::to_string(&self.unbonding_on_hold_ref_count).as_str(),
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RedelegationEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "creation_height",
            "creationHeight",
            "completion_time",
            "completionTime",
            "initial_balance",
            "initialBalance",
            "shares_dst",
            "sharesDst",
            "unbonding_id",
            "unbondingId",
            "unbonding_on_hold_ref_count",
            "unbondingOnHoldRefCount",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreationHeight,
            CompletionTime,
            InitialBalance,
            SharesDst,
            UnbondingId,
            UnbondingOnHoldRefCount,
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
                            "creationHeight" | "creation_height" => {
                                Ok(GeneratedField::CreationHeight)
                            }
                            "completionTime" | "completion_time" => {
                                Ok(GeneratedField::CompletionTime)
                            }
                            "initialBalance" | "initial_balance" => {
                                Ok(GeneratedField::InitialBalance)
                            }
                            "sharesDst" | "shares_dst" => Ok(GeneratedField::SharesDst),
                            "unbondingId" | "unbonding_id" => {
                                Ok(GeneratedField::UnbondingId)
                            }
                            "unbondingOnHoldRefCount" | "unbonding_on_hold_ref_count" => {
                                Ok(GeneratedField::UnbondingOnHoldRefCount)
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
            type Value = RedelegationEntry;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.RedelegationEntry")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<RedelegationEntry, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut creation_height__ = None;
                let mut completion_time__ = None;
                let mut initial_balance__ = None;
                let mut shares_dst__ = None;
                let mut unbonding_id__ = None;
                let mut unbonding_on_hold_ref_count__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CreationHeight => {
                            if creation_height__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("creationHeight"),
                                );
                            }
                            creation_height__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::CompletionTime => {
                            if completion_time__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("completionTime"),
                                );
                            }
                            completion_time__ = map.next_value()?;
                        }
                        GeneratedField::InitialBalance => {
                            if initial_balance__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("initialBalance"),
                                );
                            }
                            initial_balance__ = Some(map.next_value()?);
                        }
                        GeneratedField::SharesDst => {
                            if shares_dst__.is_some() {
                                return Err(serde::de::Error::duplicate_field("sharesDst"));
                            }
                            shares_dst__ = Some(map.next_value()?);
                        }
                        GeneratedField::UnbondingId => {
                            if unbonding_id__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("unbondingId"),
                                );
                            }
                            unbonding_id__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::UnbondingOnHoldRefCount => {
                            if unbonding_on_hold_ref_count__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("unbondingOnHoldRefCount"),
                                );
                            }
                            unbonding_on_hold_ref_count__ = Some(
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
                Ok(RedelegationEntry {
                    creation_height: creation_height__.unwrap_or_default(),
                    completion_time: completion_time__,
                    initial_balance: initial_balance__.unwrap_or_default(),
                    shares_dst: shares_dst__.unwrap_or_default(),
                    unbonding_id: unbonding_id__.unwrap_or_default(),
                    unbonding_on_hold_ref_count: unbonding_on_hold_ref_count__
                        .unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.RedelegationEntry",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for RedelegationEntryResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.redelegation_entry.is_some() {
            len += 1;
        }
        if !self.balance.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.RedelegationEntryResponse", len)?;
        if let Some(v) = self.redelegation_entry.as_ref() {
            struct_ser.serialize_field("redelegationEntry", v)?;
        }
        if !self.balance.is_empty() {
            struct_ser.serialize_field("balance", &self.balance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RedelegationEntryResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["redelegation_entry", "redelegationEntry", "balance"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RedelegationEntry,
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
                            "redelegationEntry" | "redelegation_entry" => {
                                Ok(GeneratedField::RedelegationEntry)
                            }
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
            type Value = RedelegationEntryResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct cosmos.staking.v1beta1.RedelegationEntryResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<RedelegationEntryResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut redelegation_entry__ = None;
                let mut balance__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RedelegationEntry => {
                            if redelegation_entry__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("redelegationEntry"),
                                );
                            }
                            redelegation_entry__ = map.next_value()?;
                        }
                        GeneratedField::Balance => {
                            if balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balance"));
                            }
                            balance__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RedelegationEntryResponse {
                    redelegation_entry: redelegation_entry__,
                    balance: balance__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.RedelegationEntryResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for RedelegationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.redelegation.is_some() {
            len += 1;
        }
        if !self.entries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.RedelegationResponse", len)?;
        if let Some(v) = self.redelegation.as_ref() {
            struct_ser.serialize_field("redelegation", v)?;
        }
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RedelegationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["redelegation", "entries"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Redelegation,
            Entries,
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
                            "redelegation" => Ok(GeneratedField::Redelegation),
                            "entries" => Ok(GeneratedField::Entries),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RedelegationResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.RedelegationResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<RedelegationResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut redelegation__ = None;
                let mut entries__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Redelegation => {
                            if redelegation__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("redelegation"),
                                );
                            }
                            redelegation__ = map.next_value()?;
                        }
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(RedelegationResponse {
                    redelegation: redelegation__,
                    entries: entries__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.RedelegationResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for TokenizeShareLock {
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
        if !self.status.is_empty() {
            len += 1;
        }
        if self.completion_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.TokenizeShareLock", len)?;
        if !self.address.is_empty() {
            struct_ser.serialize_field("address", &self.address)?;
        }
        if !self.status.is_empty() {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if let Some(v) = self.completion_time.as_ref() {
            struct_ser.serialize_field("completionTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TokenizeShareLock {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "address",
            "status",
            "completion_time",
            "completionTime",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Address,
            Status,
            CompletionTime,
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
                            "status" => Ok(GeneratedField::Status),
                            "completionTime" | "completion_time" => {
                                Ok(GeneratedField::CompletionTime)
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
            type Value = TokenizeShareLock;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.TokenizeShareLock")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<TokenizeShareLock, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut address__ = None;
                let mut status__ = None;
                let mut completion_time__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Address => {
                            if address__.is_some() {
                                return Err(serde::de::Error::duplicate_field("address"));
                            }
                            address__ = Some(map.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map.next_value()?);
                        }
                        GeneratedField::CompletionTime => {
                            if completion_time__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("completionTime"),
                                );
                            }
                            completion_time__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(TokenizeShareLock {
                    address: address__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    completion_time: completion_time__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.TokenizeShareLock",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for TokenizeShareLockStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "TOKENIZE_SHARE_LOCK_STATUS_UNSPECIFIED",
            Self::Locked => "TOKENIZE_SHARE_LOCK_STATUS_LOCKED",
            Self::Unlocked => "TOKENIZE_SHARE_LOCK_STATUS_UNLOCKED",
            Self::LockExpiring => "TOKENIZE_SHARE_LOCK_STATUS_LOCK_EXPIRING",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TokenizeShareLockStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TOKENIZE_SHARE_LOCK_STATUS_UNSPECIFIED",
            "TOKENIZE_SHARE_LOCK_STATUS_LOCKED",
            "TOKENIZE_SHARE_LOCK_STATUS_UNLOCKED",
            "TOKENIZE_SHARE_LOCK_STATUS_LOCK_EXPIRING",
        ];
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TokenizeShareLockStatus;
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
                    .and_then(TokenizeShareLockStatus::from_i32)
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
                    .and_then(TokenizeShareLockStatus::from_i32)
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
                    "TOKENIZE_SHARE_LOCK_STATUS_UNSPECIFIED" => {
                        Ok(TokenizeShareLockStatus::Unspecified)
                    }
                    "TOKENIZE_SHARE_LOCK_STATUS_LOCKED" => {
                        Ok(TokenizeShareLockStatus::Locked)
                    }
                    "TOKENIZE_SHARE_LOCK_STATUS_UNLOCKED" => {
                        Ok(TokenizeShareLockStatus::Unlocked)
                    }
                    "TOKENIZE_SHARE_LOCK_STATUS_LOCK_EXPIRING" => {
                        Ok(TokenizeShareLockStatus::LockExpiring)
                    }
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TokenizeShareRecord {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.id != 0 {
            len += 1;
        }
        if !self.owner.is_empty() {
            len += 1;
        }
        if !self.module_account.is_empty() {
            len += 1;
        }
        if !self.validator.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.TokenizeShareRecord", len)?;
        if self.id != 0 {
            struct_ser.serialize_field("id", ToString::to_string(&self.id).as_str())?;
        }
        if !self.owner.is_empty() {
            struct_ser.serialize_field("owner", &self.owner)?;
        }
        if !self.module_account.is_empty() {
            struct_ser.serialize_field("moduleAccount", &self.module_account)?;
        }
        if !self.validator.is_empty() {
            struct_ser.serialize_field("validator", &self.validator)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TokenizeShareRecord {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "owner",
            "module_account",
            "moduleAccount",
            "validator",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            Owner,
            ModuleAccount,
            Validator,
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
                            "id" => Ok(GeneratedField::Id),
                            "owner" => Ok(GeneratedField::Owner),
                            "moduleAccount" | "module_account" => {
                                Ok(GeneratedField::ModuleAccount)
                            }
                            "validator" => Ok(GeneratedField::Validator),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TokenizeShareRecord;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.TokenizeShareRecord")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<TokenizeShareRecord, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut owner__ = None;
                let mut module_account__ = None;
                let mut validator__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Owner => {
                            if owner__.is_some() {
                                return Err(serde::de::Error::duplicate_field("owner"));
                            }
                            owner__ = Some(map.next_value()?);
                        }
                        GeneratedField::ModuleAccount => {
                            if module_account__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("moduleAccount"),
                                );
                            }
                            module_account__ = Some(map.next_value()?);
                        }
                        GeneratedField::Validator => {
                            if validator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("validator"));
                            }
                            validator__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(TokenizeShareRecord {
                    id: id__.unwrap_or_default(),
                    owner: owner__.unwrap_or_default(),
                    module_account: module_account__.unwrap_or_default(),
                    validator: validator__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.TokenizeShareRecord",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for UnbondingDelegation {
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
        if !self.validator_address.is_empty() {
            len += 1;
        }
        if !self.entries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.UnbondingDelegation", len)?;
        if !self.delegator_address.is_empty() {
            struct_ser.serialize_field("delegatorAddress", &self.delegator_address)?;
        }
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        if !self.entries.is_empty() {
            struct_ser.serialize_field("entries", &self.entries)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnbondingDelegation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "delegator_address",
            "delegatorAddress",
            "validator_address",
            "validatorAddress",
            "entries",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            DelegatorAddress,
            ValidatorAddress,
            Entries,
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
                            "validatorAddress" | "validator_address" => {
                                Ok(GeneratedField::ValidatorAddress)
                            }
                            "entries" => Ok(GeneratedField::Entries),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UnbondingDelegation;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.UnbondingDelegation")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<UnbondingDelegation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut delegator_address__ = None;
                let mut validator_address__ = None;
                let mut entries__ = None;
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
                        GeneratedField::ValidatorAddress => {
                            if validator_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorAddress"),
                                );
                            }
                            validator_address__ = Some(map.next_value()?);
                        }
                        GeneratedField::Entries => {
                            if entries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("entries"));
                            }
                            entries__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UnbondingDelegation {
                    delegator_address: delegator_address__.unwrap_or_default(),
                    validator_address: validator_address__.unwrap_or_default(),
                    entries: entries__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.UnbondingDelegation",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for UnbondingDelegationEntry {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.creation_height != 0 {
            len += 1;
        }
        if self.completion_time.is_some() {
            len += 1;
        }
        if !self.initial_balance.is_empty() {
            len += 1;
        }
        if !self.balance.is_empty() {
            len += 1;
        }
        if self.unbonding_id != 0 {
            len += 1;
        }
        if self.unbonding_on_hold_ref_count != 0 {
            len += 1;
        }
        if !self.validator_bond_factor.is_empty() {
            len += 1;
        }
        if !self.global_liquid_staking_cap.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.UnbondingDelegationEntry", len)?;
        if self.creation_height != 0 {
            struct_ser
                .serialize_field(
                    "creationHeight",
                    ToString::to_string(&self.creation_height).as_str(),
                )?;
        }
        if let Some(v) = self.completion_time.as_ref() {
            struct_ser.serialize_field("completionTime", v)?;
        }
        if !self.initial_balance.is_empty() {
            struct_ser.serialize_field("initialBalance", &self.initial_balance)?;
        }
        if !self.balance.is_empty() {
            struct_ser.serialize_field("balance", &self.balance)?;
        }
        if self.unbonding_id != 0 {
            struct_ser
                .serialize_field(
                    "unbondingId",
                    ToString::to_string(&self.unbonding_id).as_str(),
                )?;
        }
        if self.unbonding_on_hold_ref_count != 0 {
            struct_ser
                .serialize_field(
                    "unbondingOnHoldRefCount",
                    ToString::to_string(&self.unbonding_on_hold_ref_count).as_str(),
                )?;
        }
        if !self.validator_bond_factor.is_empty() {
            struct_ser
                .serialize_field("validatorBondFactor", &self.validator_bond_factor)?;
        }
        if !self.global_liquid_staking_cap.is_empty() {
            struct_ser
                .serialize_field(
                    "globalLiquidStakingCap",
                    &self.global_liquid_staking_cap,
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UnbondingDelegationEntry {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "creation_height",
            "creationHeight",
            "completion_time",
            "completionTime",
            "initial_balance",
            "initialBalance",
            "balance",
            "unbonding_id",
            "unbondingId",
            "unbonding_on_hold_ref_count",
            "unbondingOnHoldRefCount",
            "validator_bond_factor",
            "validatorBondFactor",
            "global_liquid_staking_cap",
            "globalLiquidStakingCap",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CreationHeight,
            CompletionTime,
            InitialBalance,
            Balance,
            UnbondingId,
            UnbondingOnHoldRefCount,
            ValidatorBondFactor,
            GlobalLiquidStakingCap,
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
                            "creationHeight" | "creation_height" => {
                                Ok(GeneratedField::CreationHeight)
                            }
                            "completionTime" | "completion_time" => {
                                Ok(GeneratedField::CompletionTime)
                            }
                            "initialBalance" | "initial_balance" => {
                                Ok(GeneratedField::InitialBalance)
                            }
                            "balance" => Ok(GeneratedField::Balance),
                            "unbondingId" | "unbonding_id" => {
                                Ok(GeneratedField::UnbondingId)
                            }
                            "unbondingOnHoldRefCount" | "unbonding_on_hold_ref_count" => {
                                Ok(GeneratedField::UnbondingOnHoldRefCount)
                            }
                            "validatorBondFactor" | "validator_bond_factor" => {
                                Ok(GeneratedField::ValidatorBondFactor)
                            }
                            "globalLiquidStakingCap" | "global_liquid_staking_cap" => {
                                Ok(GeneratedField::GlobalLiquidStakingCap)
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
            type Value = UnbondingDelegationEntry;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct cosmos.staking.v1beta1.UnbondingDelegationEntry")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<UnbondingDelegationEntry, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut creation_height__ = None;
                let mut completion_time__ = None;
                let mut initial_balance__ = None;
                let mut balance__ = None;
                let mut unbonding_id__ = None;
                let mut unbonding_on_hold_ref_count__ = None;
                let mut validator_bond_factor__ = None;
                let mut global_liquid_staking_cap__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::CreationHeight => {
                            if creation_height__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("creationHeight"),
                                );
                            }
                            creation_height__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::CompletionTime => {
                            if completion_time__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("completionTime"),
                                );
                            }
                            completion_time__ = map.next_value()?;
                        }
                        GeneratedField::InitialBalance => {
                            if initial_balance__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("initialBalance"),
                                );
                            }
                            initial_balance__ = Some(map.next_value()?);
                        }
                        GeneratedField::Balance => {
                            if balance__.is_some() {
                                return Err(serde::de::Error::duplicate_field("balance"));
                            }
                            balance__ = Some(map.next_value()?);
                        }
                        GeneratedField::UnbondingId => {
                            if unbonding_id__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("unbondingId"),
                                );
                            }
                            unbonding_id__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::UnbondingOnHoldRefCount => {
                            if unbonding_on_hold_ref_count__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("unbondingOnHoldRefCount"),
                                );
                            }
                            unbonding_on_hold_ref_count__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::ValidatorBondFactor => {
                            if validator_bond_factor__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorBondFactor"),
                                );
                            }
                            validator_bond_factor__ = Some(map.next_value()?);
                        }
                        GeneratedField::GlobalLiquidStakingCap => {
                            if global_liquid_staking_cap__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("globalLiquidStakingCap"),
                                );
                            }
                            global_liquid_staking_cap__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(UnbondingDelegationEntry {
                    creation_height: creation_height__.unwrap_or_default(),
                    completion_time: completion_time__,
                    initial_balance: initial_balance__.unwrap_or_default(),
                    balance: balance__.unwrap_or_default(),
                    unbonding_id: unbonding_id__.unwrap_or_default(),
                    unbonding_on_hold_ref_count: unbonding_on_hold_ref_count__
                        .unwrap_or_default(),
                    validator_bond_factor: validator_bond_factor__.unwrap_or_default(),
                    global_liquid_staking_cap: global_liquid_staking_cap__
                        .unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.UnbondingDelegationEntry",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for ValAddresses {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.addresses.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.ValAddresses", len)?;
        if !self.addresses.is_empty() {
            struct_ser.serialize_field("addresses", &self.addresses)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ValAddresses {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["addresses"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Addresses,
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
                            "addresses" => Ok(GeneratedField::Addresses),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValAddresses;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct cosmos.staking.v1beta1.ValAddresses")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<ValAddresses, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut addresses__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Addresses => {
                            if addresses__.is_some() {
                                return Err(serde::de::Error::duplicate_field("addresses"));
                            }
                            addresses__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ValAddresses {
                    addresses: addresses__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.ValAddresses",
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
        if self.consensus_pubkey.is_some() {
            len += 1;
        }
        if self.jailed {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if !self.tokens.is_empty() {
            len += 1;
        }
        if !self.delegator_shares.is_empty() {
            len += 1;
        }
        if self.description.is_some() {
            len += 1;
        }
        if self.unbonding_height != 0 {
            len += 1;
        }
        if self.unbonding_time.is_some() {
            len += 1;
        }
        if self.commission.is_some() {
            len += 1;
        }
        if !self.min_self_delegation.is_empty() {
            len += 1;
        }
        if self.unbonding_on_hold_ref_count != 0 {
            len += 1;
        }
        if !self.unbonding_ids.is_empty() {
            len += 1;
        }
        if !self.validator_bond_shares.is_empty() {
            len += 1;
        }
        if !self.liquid_shares.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("cosmos.staking.v1beta1.Validator", len)?;
        if !self.operator_address.is_empty() {
            struct_ser.serialize_field("operatorAddress", &self.operator_address)?;
        }
        if let Some(v) = self.consensus_pubkey.as_ref() {
            struct_ser.serialize_field("consensusPubkey", v)?;
        }
        if self.jailed {
            struct_ser.serialize_field("jailed", &self.jailed)?;
        }
        if self.status != 0 {
            let v = BondStatus::from_i32(self.status)
                .ok_or_else(|| serde::ser::Error::custom(
                    format!("Invalid variant {}", self.status),
                ))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if !self.tokens.is_empty() {
            struct_ser.serialize_field("tokens", &self.tokens)?;
        }
        if !self.delegator_shares.is_empty() {
            struct_ser.serialize_field("delegatorShares", &self.delegator_shares)?;
        }
        if let Some(v) = self.description.as_ref() {
            struct_ser.serialize_field("description", v)?;
        }
        if self.unbonding_height != 0 {
            struct_ser
                .serialize_field(
                    "unbondingHeight",
                    ToString::to_string(&self.unbonding_height).as_str(),
                )?;
        }
        if let Some(v) = self.unbonding_time.as_ref() {
            struct_ser.serialize_field("unbondingTime", v)?;
        }
        if let Some(v) = self.commission.as_ref() {
            struct_ser.serialize_field("commission", v)?;
        }
        if !self.min_self_delegation.is_empty() {
            struct_ser.serialize_field("minSelfDelegation", &self.min_self_delegation)?;
        }
        if self.unbonding_on_hold_ref_count != 0 {
            struct_ser
                .serialize_field(
                    "unbondingOnHoldRefCount",
                    ToString::to_string(&self.unbonding_on_hold_ref_count).as_str(),
                )?;
        }
        if !self.unbonding_ids.is_empty() {
            struct_ser
                .serialize_field(
                    "unbondingIds",
                    &self
                        .unbonding_ids
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>(),
                )?;
        }
        if !self.validator_bond_shares.is_empty() {
            struct_ser
                .serialize_field("validatorBondShares", &self.validator_bond_shares)?;
        }
        if !self.liquid_shares.is_empty() {
            struct_ser.serialize_field("liquidShares", &self.liquid_shares)?;
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
            "consensus_pubkey",
            "consensusPubkey",
            "jailed",
            "status",
            "tokens",
            "delegator_shares",
            "delegatorShares",
            "description",
            "unbonding_height",
            "unbondingHeight",
            "unbonding_time",
            "unbondingTime",
            "commission",
            "min_self_delegation",
            "minSelfDelegation",
            "unbonding_on_hold_ref_count",
            "unbondingOnHoldRefCount",
            "unbonding_ids",
            "unbondingIds",
            "validator_bond_shares",
            "validatorBondShares",
            "liquid_shares",
            "liquidShares",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OperatorAddress,
            ConsensusPubkey,
            Jailed,
            Status,
            Tokens,
            DelegatorShares,
            Description,
            UnbondingHeight,
            UnbondingTime,
            Commission,
            MinSelfDelegation,
            UnbondingOnHoldRefCount,
            UnbondingIds,
            ValidatorBondShares,
            LiquidShares,
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
                            "consensusPubkey" | "consensus_pubkey" => {
                                Ok(GeneratedField::ConsensusPubkey)
                            }
                            "jailed" => Ok(GeneratedField::Jailed),
                            "status" => Ok(GeneratedField::Status),
                            "tokens" => Ok(GeneratedField::Tokens),
                            "delegatorShares" | "delegator_shares" => {
                                Ok(GeneratedField::DelegatorShares)
                            }
                            "description" => Ok(GeneratedField::Description),
                            "unbondingHeight" | "unbonding_height" => {
                                Ok(GeneratedField::UnbondingHeight)
                            }
                            "unbondingTime" | "unbonding_time" => {
                                Ok(GeneratedField::UnbondingTime)
                            }
                            "commission" => Ok(GeneratedField::Commission),
                            "minSelfDelegation" | "min_self_delegation" => {
                                Ok(GeneratedField::MinSelfDelegation)
                            }
                            "unbondingOnHoldRefCount" | "unbonding_on_hold_ref_count" => {
                                Ok(GeneratedField::UnbondingOnHoldRefCount)
                            }
                            "unbondingIds" | "unbonding_ids" => {
                                Ok(GeneratedField::UnbondingIds)
                            }
                            "validatorBondShares" | "validator_bond_shares" => {
                                Ok(GeneratedField::ValidatorBondShares)
                            }
                            "liquidShares" | "liquid_shares" => {
                                Ok(GeneratedField::LiquidShares)
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
                formatter.write_str("struct cosmos.staking.v1beta1.Validator")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Validator, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut operator_address__ = None;
                let mut consensus_pubkey__ = None;
                let mut jailed__ = None;
                let mut status__ = None;
                let mut tokens__ = None;
                let mut delegator_shares__ = None;
                let mut description__ = None;
                let mut unbonding_height__ = None;
                let mut unbonding_time__ = None;
                let mut commission__ = None;
                let mut min_self_delegation__ = None;
                let mut unbonding_on_hold_ref_count__ = None;
                let mut unbonding_ids__ = None;
                let mut validator_bond_shares__ = None;
                let mut liquid_shares__ = None;
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
                        GeneratedField::ConsensusPubkey => {
                            if consensus_pubkey__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("consensusPubkey"),
                                );
                            }
                            consensus_pubkey__ = map.next_value()?;
                        }
                        GeneratedField::Jailed => {
                            if jailed__.is_some() {
                                return Err(serde::de::Error::duplicate_field("jailed"));
                            }
                            jailed__ = Some(map.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map.next_value::<BondStatus>()? as i32);
                        }
                        GeneratedField::Tokens => {
                            if tokens__.is_some() {
                                return Err(serde::de::Error::duplicate_field("tokens"));
                            }
                            tokens__ = Some(map.next_value()?);
                        }
                        GeneratedField::DelegatorShares => {
                            if delegator_shares__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("delegatorShares"),
                                );
                            }
                            delegator_shares__ = Some(map.next_value()?);
                        }
                        GeneratedField::Description => {
                            if description__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("description"),
                                );
                            }
                            description__ = map.next_value()?;
                        }
                        GeneratedField::UnbondingHeight => {
                            if unbonding_height__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("unbondingHeight"),
                                );
                            }
                            unbonding_height__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::UnbondingTime => {
                            if unbonding_time__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("unbondingTime"),
                                );
                            }
                            unbonding_time__ = map.next_value()?;
                        }
                        GeneratedField::Commission => {
                            if commission__.is_some() {
                                return Err(serde::de::Error::duplicate_field("commission"));
                            }
                            commission__ = map.next_value()?;
                        }
                        GeneratedField::MinSelfDelegation => {
                            if min_self_delegation__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("minSelfDelegation"),
                                );
                            }
                            min_self_delegation__ = Some(map.next_value()?);
                        }
                        GeneratedField::UnbondingOnHoldRefCount => {
                            if unbonding_on_hold_ref_count__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("unbondingOnHoldRefCount"),
                                );
                            }
                            unbonding_on_hold_ref_count__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::UnbondingIds => {
                            if unbonding_ids__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("unbondingIds"),
                                );
                            }
                            unbonding_ids__ = Some(
                                map
                                    .next_value::<
                                        Vec<::pbjson::private::NumberDeserialize<_>>,
                                    >()?
                                    .into_iter()
                                    .map(|x| x.0)
                                    .collect(),
                            );
                        }
                        GeneratedField::ValidatorBondShares => {
                            if validator_bond_shares__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorBondShares"),
                                );
                            }
                            validator_bond_shares__ = Some(map.next_value()?);
                        }
                        GeneratedField::LiquidShares => {
                            if liquid_shares__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("liquidShares"),
                                );
                            }
                            liquid_shares__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Validator {
                    operator_address: operator_address__.unwrap_or_default(),
                    consensus_pubkey: consensus_pubkey__,
                    jailed: jailed__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    tokens: tokens__.unwrap_or_default(),
                    delegator_shares: delegator_shares__.unwrap_or_default(),
                    description: description__,
                    unbonding_height: unbonding_height__.unwrap_or_default(),
                    unbonding_time: unbonding_time__,
                    commission: commission__,
                    min_self_delegation: min_self_delegation__.unwrap_or_default(),
                    unbonding_on_hold_ref_count: unbonding_on_hold_ref_count__
                        .unwrap_or_default(),
                    unbonding_ids: unbonding_ids__.unwrap_or_default(),
                    validator_bond_shares: validator_bond_shares__.unwrap_or_default(),
                    liquid_shares: liquid_shares__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "cosmos.staking.v1beta1.Validator",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
// impl serde::Serialize for ValidatorUpdates {
//     #[allow(deprecated)]
//     fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         use serde::ser::SerializeStruct;
//         let mut len = 0;
//         if !self.updates.is_empty() {
//             len += 1;
//         }
//         let mut struct_ser = serializer
//             .serialize_struct("cosmos.staking.v1beta1.ValidatorUpdates", len)?;
//         if !self.updates.is_empty() {
//             struct_ser.serialize_field("updates", &self.updates)?;
//         }
//         struct_ser.end()
//     }
// }
// impl<'de> serde::Deserialize<'de> for ValidatorUpdates {
//     #[allow(deprecated)]
//     fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         const FIELDS: &[&str] = &["updates"];
//         #[allow(clippy::enum_variant_names)]
//         enum GeneratedField {
//             Updates,
//             __SkipField__,
//         }
//         impl<'de> serde::Deserialize<'de> for GeneratedField {
//             fn deserialize<D>(
//                 deserializer: D,
//             ) -> std::result::Result<GeneratedField, D::Error>
//             where
//                 D: serde::Deserializer<'de>,
//             {
//                 struct GeneratedVisitor;
//                 impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
//                     type Value = GeneratedField;
//                     fn expecting(
//                         &self,
//                         formatter: &mut std::fmt::Formatter<'_>,
//                     ) -> std::fmt::Result {
//                         write!(formatter, "expected one of: {:?}", & FIELDS)
//                     }
//                     #[allow(unused_variables)]
//                     fn visit_str<E>(
//                         self,
//                         value: &str,
//                     ) -> std::result::Result<GeneratedField, E>
//                     where
//                         E: serde::de::Error,
//                     {
//                         match value {
//                             "updates" => Ok(GeneratedField::Updates),
//                             _ => Ok(GeneratedField::__SkipField__),
//                         }
//                     }
//                 }
//                 deserializer.deserialize_identifier(GeneratedVisitor)
//             }
//         }
//         struct GeneratedVisitor;
//         impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
//             type Value = ValidatorUpdates;
//             fn expecting(
//                 &self,
//                 formatter: &mut std::fmt::Formatter<'_>,
//             ) -> std::fmt::Result {
//                 formatter.write_str("struct cosmos.staking.v1beta1.ValidatorUpdates")
//             }
//             fn visit_map<V>(
//                 self,
//                 mut map: V,
//             ) -> std::result::Result<ValidatorUpdates, V::Error>
//             where
//                 V: serde::de::MapAccess<'de>,
//             {
//                 let mut updates__ = None;
//                 while let Some(k) = map.next_key()? {
//                     match k {
//                         GeneratedField::Updates => {
//                             if updates__.is_some() {
//                                 return Err(serde::de::Error::duplicate_field("updates"));
//                             }
//                             updates__ = Some(map.next_value()?);
//                         }
//                         GeneratedField::__SkipField__ => {
//                             let _ = map.next_value::<serde::de::IgnoredAny>()?;
//                         }
//                     }
//                 }
//                 Ok(ValidatorUpdates {
//                     updates: updates__.unwrap_or_default(),
//                 })
//             }
//         }
//         deserializer
//             .deserialize_struct(
//                 "cosmos.staking.v1beta1.ValidatorUpdates",
//                 FIELDS,
//                 GeneratedVisitor,
//             )
//     }
// }
