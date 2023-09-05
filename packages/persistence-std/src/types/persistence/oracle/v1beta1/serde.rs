impl serde::Serialize for AggregateExchangeRatePrevote {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hash.is_empty() {
            len += 1;
        }
        if !self.voter.is_empty() {
            len += 1;
        }
        if self.submit_block != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.AggregateExchangeRatePrevote",
                len,
            )?;
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        if !self.voter.is_empty() {
            struct_ser.serialize_field("voter", &self.voter)?;
        }
        if self.submit_block != 0 {
            struct_ser
                .serialize_field(
                    "submitBlock",
                    ToString::to_string(&self.submit_block).as_str(),
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AggregateExchangeRatePrevote {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["hash", "voter", "submit_block", "submitBlock"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            Voter,
            SubmitBlock,
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
                            "hash" => Ok(GeneratedField::Hash),
                            "voter" => Ok(GeneratedField::Voter),
                            "submitBlock" | "submit_block" => {
                                Ok(GeneratedField::SubmitBlock)
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
            type Value = AggregateExchangeRatePrevote;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.AggregateExchangeRatePrevote",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<AggregateExchangeRatePrevote, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                let mut voter__ = None;
                let mut submit_block__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map.next_value()?);
                        }
                        GeneratedField::Voter => {
                            if voter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("voter"));
                            }
                            voter__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubmitBlock => {
                            if submit_block__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("submitBlock"),
                                );
                            }
                            submit_block__ = Some(
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
                Ok(AggregateExchangeRatePrevote {
                    hash: hash__.unwrap_or_default(),
                    voter: voter__.unwrap_or_default(),
                    submit_block: submit_block__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.AggregateExchangeRatePrevote",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for AggregateExchangeRateVote {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.exchange_rate_tuples.is_empty() {
            len += 1;
        }
        if !self.voter.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.AggregateExchangeRateVote",
                len,
            )?;
        if !self.exchange_rate_tuples.is_empty() {
            struct_ser
                .serialize_field("exchangeRateTuples", &self.exchange_rate_tuples)?;
        }
        if !self.voter.is_empty() {
            struct_ser.serialize_field("voter", &self.voter)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AggregateExchangeRateVote {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["exchange_rate_tuples", "exchangeRateTuples", "voter"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExchangeRateTuples,
            Voter,
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
                            "exchangeRateTuples" | "exchange_rate_tuples" => {
                                Ok(GeneratedField::ExchangeRateTuples)
                            }
                            "voter" => Ok(GeneratedField::Voter),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AggregateExchangeRateVote;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.AggregateExchangeRateVote",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<AggregateExchangeRateVote, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut exchange_rate_tuples__ = None;
                let mut voter__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExchangeRateTuples => {
                            if exchange_rate_tuples__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("exchangeRateTuples"),
                                );
                            }
                            exchange_rate_tuples__ = Some(map.next_value()?);
                        }
                        GeneratedField::Voter => {
                            if voter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("voter"));
                            }
                            voter__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(AggregateExchangeRateVote {
                    exchange_rate_tuples: exchange_rate_tuples__.unwrap_or_default(),
                    voter: voter__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.AggregateExchangeRateVote",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for Denom {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.base_denom.is_empty() {
            len += 1;
        }
        if !self.symbol_denom.is_empty() {
            len += 1;
        }
        if self.exponent != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("persistence.oracle.v1beta1.Denom", len)?;
        if !self.base_denom.is_empty() {
            struct_ser.serialize_field("baseDenom", &self.base_denom)?;
        }
        if !self.symbol_denom.is_empty() {
            struct_ser.serialize_field("symbolDenom", &self.symbol_denom)?;
        }
        if self.exponent != 0 {
            struct_ser.serialize_field("exponent", &self.exponent)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Denom {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "base_denom",
            "baseDenom",
            "symbol_denom",
            "symbolDenom",
            "exponent",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            BaseDenom,
            SymbolDenom,
            Exponent,
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
                            "baseDenom" | "base_denom" => Ok(GeneratedField::BaseDenom),
                            "symbolDenom" | "symbol_denom" => {
                                Ok(GeneratedField::SymbolDenom)
                            }
                            "exponent" => Ok(GeneratedField::Exponent),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Denom;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct persistence.oracle.v1beta1.Denom")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Denom, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut base_denom__ = None;
                let mut symbol_denom__ = None;
                let mut exponent__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::BaseDenom => {
                            if base_denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("baseDenom"));
                            }
                            base_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::SymbolDenom => {
                            if symbol_denom__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("symbolDenom"),
                                );
                            }
                            symbol_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::Exponent => {
                            if exponent__.is_some() {
                                return Err(serde::de::Error::duplicate_field("exponent"));
                            }
                            exponent__ = Some(
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
                Ok(Denom {
                    base_denom: base_denom__.unwrap_or_default(),
                    symbol_denom: symbol_denom__.unwrap_or_default(),
                    exponent: exponent__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.Denom",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for ExchangeRateTuple {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.denom.is_empty() {
            len += 1;
        }
        if !self.exchange_rate.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("persistence.oracle.v1beta1.ExchangeRateTuple", len)?;
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
        }
        if !self.exchange_rate.is_empty() {
            struct_ser.serialize_field("exchangeRate", &self.exchange_rate)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExchangeRateTuple {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["denom", "exchange_rate", "exchangeRate"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Denom,
            ExchangeRate,
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
                            "denom" => Ok(GeneratedField::Denom),
                            "exchangeRate" | "exchange_rate" => {
                                Ok(GeneratedField::ExchangeRate)
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
            type Value = ExchangeRateTuple;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct persistence.oracle.v1beta1.ExchangeRateTuple")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<ExchangeRateTuple, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut denom__ = None;
                let mut exchange_rate__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExchangeRate => {
                            if exchange_rate__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("exchangeRate"),
                                );
                            }
                            exchange_rate__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(ExchangeRateTuple {
                    denom: denom__.unwrap_or_default(),
                    exchange_rate: exchange_rate__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.ExchangeRateTuple",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for FeederDelegation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.feeder_address.is_empty() {
            len += 1;
        }
        if !self.validator_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("persistence.oracle.v1beta1.FeederDelegation", len)?;
        if !self.feeder_address.is_empty() {
            struct_ser.serialize_field("feederAddress", &self.feeder_address)?;
        }
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for FeederDelegation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "feeder_address",
            "feederAddress",
            "validator_address",
            "validatorAddress",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeederAddress,
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
                            "feederAddress" | "feeder_address" => {
                                Ok(GeneratedField::FeederAddress)
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
            type Value = FeederDelegation;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct persistence.oracle.v1beta1.FeederDelegation")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<FeederDelegation, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut feeder_address__ = None;
                let mut validator_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FeederAddress => {
                            if feeder_address__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("feederAddress"),
                                );
                            }
                            feeder_address__ = Some(map.next_value()?);
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
                Ok(FeederDelegation {
                    feeder_address: feeder_address__.unwrap_or_default(),
                    validator_address: validator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.FeederDelegation",
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
        if !self.feeder_delegations.is_empty() {
            len += 1;
        }
        if !self.exchange_rates.is_empty() {
            len += 1;
        }
        if !self.miss_counters.is_empty() {
            len += 1;
        }
        if !self.aggregate_exchange_rate_prevotes.is_empty() {
            len += 1;
        }
        if !self.aggregate_exchange_rate_votes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("persistence.oracle.v1beta1.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.feeder_delegations.is_empty() {
            struct_ser.serialize_field("feederDelegations", &self.feeder_delegations)?;
        }
        if !self.exchange_rates.is_empty() {
            struct_ser.serialize_field("exchangeRates", &self.exchange_rates)?;
        }
        if !self.miss_counters.is_empty() {
            struct_ser.serialize_field("missCounters", &self.miss_counters)?;
        }
        if !self.aggregate_exchange_rate_prevotes.is_empty() {
            struct_ser
                .serialize_field(
                    "aggregateExchangeRatePrevotes",
                    &self.aggregate_exchange_rate_prevotes,
                )?;
        }
        if !self.aggregate_exchange_rate_votes.is_empty() {
            struct_ser
                .serialize_field(
                    "aggregateExchangeRateVotes",
                    &self.aggregate_exchange_rate_votes,
                )?;
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
            "feeder_delegations",
            "feederDelegations",
            "exchange_rates",
            "exchangeRates",
            "miss_counters",
            "missCounters",
            "aggregate_exchange_rate_prevotes",
            "aggregateExchangeRatePrevotes",
            "aggregate_exchange_rate_votes",
            "aggregateExchangeRateVotes",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            FeederDelegations,
            ExchangeRates,
            MissCounters,
            AggregateExchangeRatePrevotes,
            AggregateExchangeRateVotes,
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
                            "feederDelegations" | "feeder_delegations" => {
                                Ok(GeneratedField::FeederDelegations)
                            }
                            "exchangeRates" | "exchange_rates" => {
                                Ok(GeneratedField::ExchangeRates)
                            }
                            "missCounters" | "miss_counters" => {
                                Ok(GeneratedField::MissCounters)
                            }
                            "aggregateExchangeRatePrevotes"
                            | "aggregate_exchange_rate_prevotes" => {
                                Ok(GeneratedField::AggregateExchangeRatePrevotes)
                            }
                            "aggregateExchangeRateVotes"
                            | "aggregate_exchange_rate_votes" => {
                                Ok(GeneratedField::AggregateExchangeRateVotes)
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
                formatter.write_str("struct persistence.oracle.v1beta1.GenesisState")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut feeder_delegations__ = None;
                let mut exchange_rates__ = None;
                let mut miss_counters__ = None;
                let mut aggregate_exchange_rate_prevotes__ = None;
                let mut aggregate_exchange_rate_votes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                        GeneratedField::FeederDelegations => {
                            if feeder_delegations__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("feederDelegations"),
                                );
                            }
                            feeder_delegations__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExchangeRates => {
                            if exchange_rates__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("exchangeRates"),
                                );
                            }
                            exchange_rates__ = Some(map.next_value()?);
                        }
                        GeneratedField::MissCounters => {
                            if miss_counters__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("missCounters"),
                                );
                            }
                            miss_counters__ = Some(map.next_value()?);
                        }
                        GeneratedField::AggregateExchangeRatePrevotes => {
                            if aggregate_exchange_rate_prevotes__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field(
                                        "aggregateExchangeRatePrevotes",
                                    ),
                                );
                            }
                            aggregate_exchange_rate_prevotes__ = Some(map.next_value()?);
                        }
                        GeneratedField::AggregateExchangeRateVotes => {
                            if aggregate_exchange_rate_votes__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field(
                                        "aggregateExchangeRateVotes",
                                    ),
                                );
                            }
                            aggregate_exchange_rate_votes__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    feeder_delegations: feeder_delegations__.unwrap_or_default(),
                    exchange_rates: exchange_rates__.unwrap_or_default(),
                    miss_counters: miss_counters__.unwrap_or_default(),
                    aggregate_exchange_rate_prevotes: aggregate_exchange_rate_prevotes__
                        .unwrap_or_default(),
                    aggregate_exchange_rate_votes: aggregate_exchange_rate_votes__
                        .unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.GenesisState",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MissCounter {
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
        if self.miss_counter != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("persistence.oracle.v1beta1.MissCounter", len)?;
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        if self.miss_counter != 0 {
            struct_ser
                .serialize_field(
                    "missCounter",
                    ToString::to_string(&self.miss_counter).as_str(),
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MissCounter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "validator_address",
            "validatorAddress",
            "miss_counter",
            "missCounter",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidatorAddress,
            MissCounter,
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
                            "missCounter" | "miss_counter" => {
                                Ok(GeneratedField::MissCounter)
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
            type Value = MissCounter;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct persistence.oracle.v1beta1.MissCounter")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MissCounter, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut validator_address__ = None;
                let mut miss_counter__ = None;
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
                        GeneratedField::MissCounter => {
                            if miss_counter__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("missCounter"),
                                );
                            }
                            miss_counter__ = Some(
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
                Ok(MissCounter {
                    validator_address: validator_address__.unwrap_or_default(),
                    miss_counter: miss_counter__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.MissCounter",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgAddFundsToRewardPool {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.from.is_empty() {
            len += 1;
        }
        if !self.funds.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.MsgAddFundsToRewardPool",
                len,
            )?;
        if !self.from.is_empty() {
            struct_ser.serialize_field("from", &self.from)?;
        }
        if !self.funds.is_empty() {
            struct_ser.serialize_field("funds", &self.funds)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddFundsToRewardPool {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["from", "funds"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            From,
            Funds,
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
                            "from" => Ok(GeneratedField::From),
                            "funds" => Ok(GeneratedField::Funds),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgAddFundsToRewardPool;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.MsgAddFundsToRewardPool",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgAddFundsToRewardPool, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut from__ = None;
                let mut funds__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::From => {
                            if from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("from"));
                            }
                            from__ = Some(map.next_value()?);
                        }
                        GeneratedField::Funds => {
                            if funds__.is_some() {
                                return Err(serde::de::Error::duplicate_field("funds"));
                            }
                            funds__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgAddFundsToRewardPool {
                    from: from__.unwrap_or_default(),
                    funds: funds__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.MsgAddFundsToRewardPool",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgAddFundsToRewardPoolResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.MsgAddFundsToRewardPoolResponse",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAddFundsToRewardPoolResponse {
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
            type Value = MsgAddFundsToRewardPoolResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.MsgAddFundsToRewardPoolResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgAddFundsToRewardPoolResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAddFundsToRewardPoolResponse {})
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.MsgAddFundsToRewardPoolResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgAggregateExchangeRatePrevote {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.hash.is_empty() {
            len += 1;
        }
        if !self.feeder.is_empty() {
            len += 1;
        }
        if !self.validator.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.MsgAggregateExchangeRatePrevote",
                len,
            )?;
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        if !self.feeder.is_empty() {
            struct_ser.serialize_field("feeder", &self.feeder)?;
        }
        if !self.validator.is_empty() {
            struct_ser.serialize_field("validator", &self.validator)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAggregateExchangeRatePrevote {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["hash", "feeder", "validator"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Hash,
            Feeder,
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
                            "hash" => Ok(GeneratedField::Hash),
                            "feeder" => Ok(GeneratedField::Feeder),
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
            type Value = MsgAggregateExchangeRatePrevote;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.MsgAggregateExchangeRatePrevote",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgAggregateExchangeRatePrevote, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut hash__ = None;
                let mut feeder__ = None;
                let mut validator__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map.next_value()?);
                        }
                        GeneratedField::Feeder => {
                            if feeder__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feeder"));
                            }
                            feeder__ = Some(map.next_value()?);
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
                Ok(MsgAggregateExchangeRatePrevote {
                    hash: hash__.unwrap_or_default(),
                    feeder: feeder__.unwrap_or_default(),
                    validator: validator__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.MsgAggregateExchangeRatePrevote",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgAggregateExchangeRatePrevoteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.MsgAggregateExchangeRatePrevoteResponse",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAggregateExchangeRatePrevoteResponse {
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
            type Value = MsgAggregateExchangeRatePrevoteResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.MsgAggregateExchangeRatePrevoteResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgAggregateExchangeRatePrevoteResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAggregateExchangeRatePrevoteResponse {
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.MsgAggregateExchangeRatePrevoteResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgAggregateExchangeRateVote {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.salt.is_empty() {
            len += 1;
        }
        if !self.exchange_rates.is_empty() {
            len += 1;
        }
        if !self.feeder.is_empty() {
            len += 1;
        }
        if !self.validator.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.MsgAggregateExchangeRateVote",
                len,
            )?;
        if !self.salt.is_empty() {
            struct_ser.serialize_field("salt", &self.salt)?;
        }
        if !self.exchange_rates.is_empty() {
            struct_ser.serialize_field("exchangeRates", &self.exchange_rates)?;
        }
        if !self.feeder.is_empty() {
            struct_ser.serialize_field("feeder", &self.feeder)?;
        }
        if !self.validator.is_empty() {
            struct_ser.serialize_field("validator", &self.validator)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAggregateExchangeRateVote {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "salt",
            "exchange_rates",
            "exchangeRates",
            "feeder",
            "validator",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Salt,
            ExchangeRates,
            Feeder,
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
                            "salt" => Ok(GeneratedField::Salt),
                            "exchangeRates" | "exchange_rates" => {
                                Ok(GeneratedField::ExchangeRates)
                            }
                            "feeder" => Ok(GeneratedField::Feeder),
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
            type Value = MsgAggregateExchangeRateVote;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.MsgAggregateExchangeRateVote",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgAggregateExchangeRateVote, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut salt__ = None;
                let mut exchange_rates__ = None;
                let mut feeder__ = None;
                let mut validator__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Salt => {
                            if salt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("salt"));
                            }
                            salt__ = Some(map.next_value()?);
                        }
                        GeneratedField::ExchangeRates => {
                            if exchange_rates__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("exchangeRates"),
                                );
                            }
                            exchange_rates__ = Some(map.next_value()?);
                        }
                        GeneratedField::Feeder => {
                            if feeder__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feeder"));
                            }
                            feeder__ = Some(map.next_value()?);
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
                Ok(MsgAggregateExchangeRateVote {
                    salt: salt__.unwrap_or_default(),
                    exchange_rates: exchange_rates__.unwrap_or_default(),
                    feeder: feeder__.unwrap_or_default(),
                    validator: validator__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.MsgAggregateExchangeRateVote",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgAggregateExchangeRateVoteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.MsgAggregateExchangeRateVoteResponse",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgAggregateExchangeRateVoteResponse {
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
            type Value = MsgAggregateExchangeRateVoteResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.MsgAggregateExchangeRateVoteResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgAggregateExchangeRateVoteResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgAggregateExchangeRateVoteResponse {
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.MsgAggregateExchangeRateVoteResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgDelegateFeedConsent {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.operator.is_empty() {
            len += 1;
        }
        if !self.delegate.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("persistence.oracle.v1beta1.MsgDelegateFeedConsent", len)?;
        if !self.operator.is_empty() {
            struct_ser.serialize_field("operator", &self.operator)?;
        }
        if !self.delegate.is_empty() {
            struct_ser.serialize_field("delegate", &self.delegate)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDelegateFeedConsent {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["operator", "delegate"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Operator,
            Delegate,
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
                            "operator" => Ok(GeneratedField::Operator),
                            "delegate" => Ok(GeneratedField::Delegate),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MsgDelegateFeedConsent;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.MsgDelegateFeedConsent",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgDelegateFeedConsent, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut operator__ = None;
                let mut delegate__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Operator => {
                            if operator__.is_some() {
                                return Err(serde::de::Error::duplicate_field("operator"));
                            }
                            operator__ = Some(map.next_value()?);
                        }
                        GeneratedField::Delegate => {
                            if delegate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delegate"));
                            }
                            delegate__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(MsgDelegateFeedConsent {
                    operator: operator__.unwrap_or_default(),
                    delegate: delegate__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.MsgDelegateFeedConsent",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for MsgDelegateFeedConsentResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.MsgDelegateFeedConsentResponse",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgDelegateFeedConsentResponse {
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
            type Value = MsgDelegateFeedConsentResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.MsgDelegateFeedConsentResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgDelegateFeedConsentResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgDelegateFeedConsentResponse {})
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.MsgDelegateFeedConsentResponse",
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
        if self.vote_period != 0 {
            len += 1;
        }
        if !self.vote_threshold.is_empty() {
            len += 1;
        }
        if !self.reward_band.is_empty() {
            len += 1;
        }
        if self.reward_distribution_window != 0 {
            len += 1;
        }
        if !self.accept_list.is_empty() {
            len += 1;
        }
        if !self.slash_fraction.is_empty() {
            len += 1;
        }
        if self.slash_window != 0 {
            len += 1;
        }
        if !self.min_valid_per_window.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("persistence.oracle.v1beta1.Params", len)?;
        if self.vote_period != 0 {
            struct_ser
                .serialize_field(
                    "votePeriod",
                    ToString::to_string(&self.vote_period).as_str(),
                )?;
        }
        if !self.vote_threshold.is_empty() {
            struct_ser.serialize_field("voteThreshold", &self.vote_threshold)?;
        }
        if !self.reward_band.is_empty() {
            struct_ser.serialize_field("rewardBand", &self.reward_band)?;
        }
        if self.reward_distribution_window != 0 {
            struct_ser
                .serialize_field(
                    "rewardDistributionWindow",
                    ToString::to_string(&self.reward_distribution_window).as_str(),
                )?;
        }
        if !self.accept_list.is_empty() {
            struct_ser.serialize_field("acceptList", &self.accept_list)?;
        }
        if !self.slash_fraction.is_empty() {
            struct_ser.serialize_field("slashFraction", &self.slash_fraction)?;
        }
        if self.slash_window != 0 {
            struct_ser
                .serialize_field(
                    "slashWindow",
                    ToString::to_string(&self.slash_window).as_str(),
                )?;
        }
        if !self.min_valid_per_window.is_empty() {
            struct_ser.serialize_field("minValidPerWindow", &self.min_valid_per_window)?;
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
            "vote_period",
            "votePeriod",
            "vote_threshold",
            "voteThreshold",
            "reward_band",
            "rewardBand",
            "reward_distribution_window",
            "rewardDistributionWindow",
            "accept_list",
            "acceptList",
            "slash_fraction",
            "slashFraction",
            "slash_window",
            "slashWindow",
            "min_valid_per_window",
            "minValidPerWindow",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            VotePeriod,
            VoteThreshold,
            RewardBand,
            RewardDistributionWindow,
            AcceptList,
            SlashFraction,
            SlashWindow,
            MinValidPerWindow,
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
                            "votePeriod" | "vote_period" => {
                                Ok(GeneratedField::VotePeriod)
                            }
                            "voteThreshold" | "vote_threshold" => {
                                Ok(GeneratedField::VoteThreshold)
                            }
                            "rewardBand" | "reward_band" => {
                                Ok(GeneratedField::RewardBand)
                            }
                            "rewardDistributionWindow" | "reward_distribution_window" => {
                                Ok(GeneratedField::RewardDistributionWindow)
                            }
                            "acceptList" | "accept_list" => {
                                Ok(GeneratedField::AcceptList)
                            }
                            "slashFraction" | "slash_fraction" => {
                                Ok(GeneratedField::SlashFraction)
                            }
                            "slashWindow" | "slash_window" => {
                                Ok(GeneratedField::SlashWindow)
                            }
                            "minValidPerWindow" | "min_valid_per_window" => {
                                Ok(GeneratedField::MinValidPerWindow)
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
                formatter.write_str("struct persistence.oracle.v1beta1.Params")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut vote_period__ = None;
                let mut vote_threshold__ = None;
                let mut reward_band__ = None;
                let mut reward_distribution_window__ = None;
                let mut accept_list__ = None;
                let mut slash_fraction__ = None;
                let mut slash_window__ = None;
                let mut min_valid_per_window__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::VotePeriod => {
                            if vote_period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("votePeriod"));
                            }
                            vote_period__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::VoteThreshold => {
                            if vote_threshold__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("voteThreshold"),
                                );
                            }
                            vote_threshold__ = Some(map.next_value()?);
                        }
                        GeneratedField::RewardBand => {
                            if reward_band__.is_some() {
                                return Err(serde::de::Error::duplicate_field("rewardBand"));
                            }
                            reward_band__ = Some(map.next_value()?);
                        }
                        GeneratedField::RewardDistributionWindow => {
                            if reward_distribution_window__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field(
                                        "rewardDistributionWindow",
                                    ),
                                );
                            }
                            reward_distribution_window__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::AcceptList => {
                            if accept_list__.is_some() {
                                return Err(serde::de::Error::duplicate_field("acceptList"));
                            }
                            accept_list__ = Some(map.next_value()?);
                        }
                        GeneratedField::SlashFraction => {
                            if slash_fraction__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("slashFraction"),
                                );
                            }
                            slash_fraction__ = Some(map.next_value()?);
                        }
                        GeneratedField::SlashWindow => {
                            if slash_window__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("slashWindow"),
                                );
                            }
                            slash_window__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::MinValidPerWindow => {
                            if min_valid_per_window__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("minValidPerWindow"),
                                );
                            }
                            min_valid_per_window__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Params {
                    vote_period: vote_period__.unwrap_or_default(),
                    vote_threshold: vote_threshold__.unwrap_or_default(),
                    reward_band: reward_band__.unwrap_or_default(),
                    reward_distribution_window: reward_distribution_window__
                        .unwrap_or_default(),
                    accept_list: accept_list__.unwrap_or_default(),
                    slash_fraction: slash_fraction__.unwrap_or_default(),
                    slash_window: slash_window__.unwrap_or_default(),
                    min_valid_per_window: min_valid_per_window__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.Params",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryActiveExchangeRatesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.QueryActiveExchangeRatesRequest",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryActiveExchangeRatesRequest {
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
            type Value = QueryActiveExchangeRatesRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.QueryActiveExchangeRatesRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryActiveExchangeRatesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryActiveExchangeRatesRequest {})
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.QueryActiveExchangeRatesRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryActiveExchangeRatesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.active_rates.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.QueryActiveExchangeRatesResponse",
                len,
            )?;
        if !self.active_rates.is_empty() {
            struct_ser.serialize_field("activeRates", &self.active_rates)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryActiveExchangeRatesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["active_rates", "activeRates"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ActiveRates,
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
                            "activeRates" | "active_rates" => {
                                Ok(GeneratedField::ActiveRates)
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
            type Value = QueryActiveExchangeRatesResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.QueryActiveExchangeRatesResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryActiveExchangeRatesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut active_rates__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ActiveRates => {
                            if active_rates__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("activeRates"),
                                );
                            }
                            active_rates__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryActiveExchangeRatesResponse {
                    active_rates: active_rates__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.QueryActiveExchangeRatesResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryAggregatePrevoteRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.validator_addr.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.QueryAggregatePrevoteRequest",
                len,
            )?;
        if !self.validator_addr.is_empty() {
            struct_ser.serialize_field("validatorAddr", &self.validator_addr)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAggregatePrevoteRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["validator_addr", "validatorAddr"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidatorAddr,
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
                            "validatorAddr" | "validator_addr" => {
                                Ok(GeneratedField::ValidatorAddr)
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
            type Value = QueryAggregatePrevoteRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.QueryAggregatePrevoteRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryAggregatePrevoteRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut validator_addr__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ValidatorAddr => {
                            if validator_addr__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorAddr"),
                                );
                            }
                            validator_addr__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryAggregatePrevoteRequest {
                    validator_addr: validator_addr__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.QueryAggregatePrevoteRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryAggregatePrevoteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.aggregate_prevote.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.QueryAggregatePrevoteResponse",
                len,
            )?;
        if let Some(v) = self.aggregate_prevote.as_ref() {
            struct_ser.serialize_field("aggregatePrevote", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAggregatePrevoteResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["aggregate_prevote", "aggregatePrevote"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AggregatePrevote,
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
                            "aggregatePrevote" | "aggregate_prevote" => {
                                Ok(GeneratedField::AggregatePrevote)
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
            type Value = QueryAggregatePrevoteResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.QueryAggregatePrevoteResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryAggregatePrevoteResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut aggregate_prevote__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AggregatePrevote => {
                            if aggregate_prevote__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("aggregatePrevote"),
                                );
                            }
                            aggregate_prevote__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryAggregatePrevoteResponse {
                    aggregate_prevote: aggregate_prevote__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.QueryAggregatePrevoteResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryAggregatePrevotesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.QueryAggregatePrevotesRequest",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAggregatePrevotesRequest {
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
            type Value = QueryAggregatePrevotesRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.QueryAggregatePrevotesRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryAggregatePrevotesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryAggregatePrevotesRequest {})
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.QueryAggregatePrevotesRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryAggregatePrevotesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.aggregate_prevotes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.QueryAggregatePrevotesResponse",
                len,
            )?;
        if !self.aggregate_prevotes.is_empty() {
            struct_ser.serialize_field("aggregatePrevotes", &self.aggregate_prevotes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAggregatePrevotesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["aggregate_prevotes", "aggregatePrevotes"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AggregatePrevotes,
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
                            "aggregatePrevotes" | "aggregate_prevotes" => {
                                Ok(GeneratedField::AggregatePrevotes)
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
            type Value = QueryAggregatePrevotesResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.QueryAggregatePrevotesResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryAggregatePrevotesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut aggregate_prevotes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AggregatePrevotes => {
                            if aggregate_prevotes__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("aggregatePrevotes"),
                                );
                            }
                            aggregate_prevotes__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryAggregatePrevotesResponse {
                    aggregate_prevotes: aggregate_prevotes__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.QueryAggregatePrevotesResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryAggregateVoteRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.validator_addr.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.QueryAggregateVoteRequest",
                len,
            )?;
        if !self.validator_addr.is_empty() {
            struct_ser.serialize_field("validatorAddr", &self.validator_addr)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAggregateVoteRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["validator_addr", "validatorAddr"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidatorAddr,
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
                            "validatorAddr" | "validator_addr" => {
                                Ok(GeneratedField::ValidatorAddr)
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
            type Value = QueryAggregateVoteRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.QueryAggregateVoteRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryAggregateVoteRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut validator_addr__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ValidatorAddr => {
                            if validator_addr__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorAddr"),
                                );
                            }
                            validator_addr__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryAggregateVoteRequest {
                    validator_addr: validator_addr__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.QueryAggregateVoteRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryAggregateVoteResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.aggregate_vote.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.QueryAggregateVoteResponse",
                len,
            )?;
        if let Some(v) = self.aggregate_vote.as_ref() {
            struct_ser.serialize_field("aggregateVote", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAggregateVoteResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["aggregate_vote", "aggregateVote"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AggregateVote,
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
                            "aggregateVote" | "aggregate_vote" => {
                                Ok(GeneratedField::AggregateVote)
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
            type Value = QueryAggregateVoteResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.QueryAggregateVoteResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryAggregateVoteResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut aggregate_vote__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AggregateVote => {
                            if aggregate_vote__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("aggregateVote"),
                                );
                            }
                            aggregate_vote__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryAggregateVoteResponse {
                    aggregate_vote: aggregate_vote__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.QueryAggregateVoteResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryAggregateVotesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.QueryAggregateVotesRequest",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAggregateVotesRequest {
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
            type Value = QueryAggregateVotesRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.QueryAggregateVotesRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryAggregateVotesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryAggregateVotesRequest {})
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.QueryAggregateVotesRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryAggregateVotesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.aggregate_votes.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.QueryAggregateVotesResponse",
                len,
            )?;
        if !self.aggregate_votes.is_empty() {
            struct_ser.serialize_field("aggregateVotes", &self.aggregate_votes)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAggregateVotesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["aggregate_votes", "aggregateVotes"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            AggregateVotes,
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
                            "aggregateVotes" | "aggregate_votes" => {
                                Ok(GeneratedField::AggregateVotes)
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
            type Value = QueryAggregateVotesResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.QueryAggregateVotesResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryAggregateVotesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut aggregate_votes__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::AggregateVotes => {
                            if aggregate_votes__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("aggregateVotes"),
                                );
                            }
                            aggregate_votes__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryAggregateVotesResponse {
                    aggregate_votes: aggregate_votes__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.QueryAggregateVotesResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryAllExchangeRatesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.QueryAllExchangeRatesRequest",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllExchangeRatesRequest {
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
            type Value = QueryAllExchangeRatesRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.QueryAllExchangeRatesRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryAllExchangeRatesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryAllExchangeRatesRequest {})
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.QueryAllExchangeRatesRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryAllExchangeRatesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.exchange_rates.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.QueryAllExchangeRatesResponse",
                len,
            )?;
        if !self.exchange_rates.is_empty() {
            struct_ser.serialize_field("exchangeRates", &self.exchange_rates)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryAllExchangeRatesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["exchange_rates", "exchangeRates"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExchangeRates,
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
                            "exchangeRates" | "exchange_rates" => {
                                Ok(GeneratedField::ExchangeRates)
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
            type Value = QueryAllExchangeRatesResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.QueryAllExchangeRatesResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryAllExchangeRatesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut exchange_rates__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExchangeRates => {
                            if exchange_rates__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("exchangeRates"),
                                );
                            }
                            exchange_rates__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryAllExchangeRatesResponse {
                    exchange_rates: exchange_rates__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.QueryAllExchangeRatesResponse",
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
        if !self.denom.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.QueryExchangeRateRequest",
                len,
            )?;
        if !self.denom.is_empty() {
            struct_ser.serialize_field("denom", &self.denom)?;
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
        const FIELDS: &[&str] = &["denom"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Denom,
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
                            "denom" => Ok(GeneratedField::Denom),
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
                        "struct persistence.oracle.v1beta1.QueryExchangeRateRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryExchangeRateRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut denom__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Denom => {
                            if denom__.is_some() {
                                return Err(serde::de::Error::duplicate_field("denom"));
                            }
                            denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryExchangeRateRequest {
                    denom: denom__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.QueryExchangeRateRequest",
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
        if !self.exchange_rate.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.QueryExchangeRateResponse",
                len,
            )?;
        if !self.exchange_rate.is_empty() {
            struct_ser.serialize_field("exchangeRate", &self.exchange_rate)?;
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
        const FIELDS: &[&str] = &["exchange_rate", "exchangeRate"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ExchangeRate,
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
                            "exchangeRate" | "exchange_rate" => {
                                Ok(GeneratedField::ExchangeRate)
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
            type Value = QueryExchangeRateResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.QueryExchangeRateResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryExchangeRateResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut exchange_rate__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ExchangeRate => {
                            if exchange_rate__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("exchangeRate"),
                                );
                            }
                            exchange_rate__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryExchangeRateResponse {
                    exchange_rate: exchange_rate__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.QueryExchangeRateResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryFeederDelegationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.validator_addr.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.QueryFeederDelegationRequest",
                len,
            )?;
        if !self.validator_addr.is_empty() {
            struct_ser.serialize_field("validatorAddr", &self.validator_addr)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryFeederDelegationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["validator_addr", "validatorAddr"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidatorAddr,
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
                            "validatorAddr" | "validator_addr" => {
                                Ok(GeneratedField::ValidatorAddr)
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
            type Value = QueryFeederDelegationRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.QueryFeederDelegationRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryFeederDelegationRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut validator_addr__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ValidatorAddr => {
                            if validator_addr__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorAddr"),
                                );
                            }
                            validator_addr__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryFeederDelegationRequest {
                    validator_addr: validator_addr__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.QueryFeederDelegationRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryFeederDelegationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.feeder_addr.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.QueryFeederDelegationResponse",
                len,
            )?;
        if !self.feeder_addr.is_empty() {
            struct_ser.serialize_field("feederAddr", &self.feeder_addr)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryFeederDelegationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["feeder_addr", "feederAddr"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            FeederAddr,
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
                            "feederAddr" | "feeder_addr" => {
                                Ok(GeneratedField::FeederAddr)
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
            type Value = QueryFeederDelegationResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.QueryFeederDelegationResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryFeederDelegationResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut feeder_addr__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::FeederAddr => {
                            if feeder_addr__.is_some() {
                                return Err(serde::de::Error::duplicate_field("feederAddr"));
                            }
                            feeder_addr__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryFeederDelegationResponse {
                    feeder_addr: feeder_addr__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.QueryFeederDelegationResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryMissCounterRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.validator_addr.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.QueryMissCounterRequest",
                len,
            )?;
        if !self.validator_addr.is_empty() {
            struct_ser.serialize_field("validatorAddr", &self.validator_addr)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryMissCounterRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["validator_addr", "validatorAddr"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ValidatorAddr,
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
                            "validatorAddr" | "validator_addr" => {
                                Ok(GeneratedField::ValidatorAddr)
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
            type Value = QueryMissCounterRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.QueryMissCounterRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryMissCounterRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut validator_addr__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ValidatorAddr => {
                            if validator_addr__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorAddr"),
                                );
                            }
                            validator_addr__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryMissCounterRequest {
                    validator_addr: validator_addr__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.QueryMissCounterRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryMissCounterResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.miss_counter != 0 {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.QueryMissCounterResponse",
                len,
            )?;
        if self.miss_counter != 0 {
            struct_ser
                .serialize_field(
                    "missCounter",
                    ToString::to_string(&self.miss_counter).as_str(),
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryMissCounterResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["miss_counter", "missCounter"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MissCounter,
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
                            "missCounter" | "miss_counter" => {
                                Ok(GeneratedField::MissCounter)
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
            type Value = QueryMissCounterResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.QueryMissCounterResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryMissCounterResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut miss_counter__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MissCounter => {
                            if miss_counter__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("missCounter"),
                                );
                            }
                            miss_counter__ = Some(
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
                Ok(QueryMissCounterResponse {
                    miss_counter: miss_counter__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.QueryMissCounterResponse",
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
            .serialize_struct("persistence.oracle.v1beta1.QueryParamsRequest", len)?;
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
                    .write_str("struct persistence.oracle.v1beta1.QueryParamsRequest")
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
                "persistence.oracle.v1beta1.QueryParamsRequest",
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
            .serialize_struct("persistence.oracle.v1beta1.QueryParamsResponse", len)?;
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
                    .write_str("struct persistence.oracle.v1beta1.QueryParamsResponse")
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
                "persistence.oracle.v1beta1.QueryParamsResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryRewardPoolBalanceRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.QueryRewardPoolBalanceRequest",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRewardPoolBalanceRequest {
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
            type Value = QueryRewardPoolBalanceRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.QueryRewardPoolBalanceRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryRewardPoolBalanceRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryRewardPoolBalanceRequest {})
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.QueryRewardPoolBalanceRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryRewardPoolBalanceResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.remaining_funds.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.oracle.v1beta1.QueryRewardPoolBalanceResponse",
                len,
            )?;
        if !self.remaining_funds.is_empty() {
            struct_ser.serialize_field("remainingFunds", &self.remaining_funds)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRewardPoolBalanceResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["remaining_funds", "remainingFunds"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RemainingFunds,
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
                            "remainingFunds" | "remaining_funds" => {
                                Ok(GeneratedField::RemainingFunds)
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
            type Value = QueryRewardPoolBalanceResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.oracle.v1beta1.QueryRewardPoolBalanceResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryRewardPoolBalanceResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut remaining_funds__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RemainingFunds => {
                            if remaining_funds__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("remainingFunds"),
                                );
                            }
                            remaining_funds__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryRewardPoolBalanceResponse {
                    remaining_funds: remaining_funds__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.oracle.v1beta1.QueryRewardPoolBalanceResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
