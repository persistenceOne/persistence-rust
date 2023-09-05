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
        if !self.liquid_validators.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lspersistence.v1beta1.GenesisState", len)?;
        if let Some(v) = self.params.as_ref() {
            struct_ser.serialize_field("params", v)?;
        }
        if !self.liquid_validators.is_empty() {
            struct_ser.serialize_field("liquidValidators", &self.liquid_validators)?;
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
        const FIELDS: &[&str] = &["params", "liquid_validators", "liquidValidators"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Params,
            LiquidValidators,
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
                            "liquidValidators" | "liquid_validators" => {
                                Ok(GeneratedField::LiquidValidators)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
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
                formatter.write_str("struct pstake.lspersistence.v1beta1.GenesisState")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut params__ = None;
                let mut liquid_validators__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Params => {
                            if params__.is_some() {
                                return Err(serde::de::Error::duplicate_field("params"));
                            }
                            params__ = map.next_value()?;
                        }
                        GeneratedField::LiquidValidators => {
                            if liquid_validators__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("liquidValidators"),
                                );
                            }
                            liquid_validators__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GenesisState {
                    params: params__,
                    liquid_validators: liquid_validators__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lspersistence.v1beta1.GenesisState",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for LiquidValidator {
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
        let mut struct_ser = serializer
            .serialize_struct("pstake.lspersistence.v1beta1.LiquidValidator", len)?;
        if !self.operator_address.is_empty() {
            struct_ser.serialize_field("operatorAddress", &self.operator_address)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LiquidValidator {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["operator_address", "operatorAddress"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OperatorAddress,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LiquidValidator;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.lspersistence.v1beta1.LiquidValidator")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<LiquidValidator, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut operator_address__ = None;
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
                    }
                }
                Ok(LiquidValidator {
                    operator_address: operator_address__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lspersistence.v1beta1.LiquidValidator",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for LiquidValidatorState {
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
        if !self.weight.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if !self.del_shares.is_empty() {
            len += 1;
        }
        if !self.liquid_tokens.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lspersistence.v1beta1.LiquidValidatorState", len)?;
        if !self.operator_address.is_empty() {
            struct_ser.serialize_field("operatorAddress", &self.operator_address)?;
        }
        if !self.weight.is_empty() {
            struct_ser.serialize_field("weight", &self.weight)?;
        }
        if self.status != 0 {
            let v = ValidatorStatus::from_i32(self.status)
                .ok_or_else(|| serde::ser::Error::custom(
                    format!("Invalid variant {}", self.status),
                ))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if !self.del_shares.is_empty() {
            struct_ser.serialize_field("delShares", &self.del_shares)?;
        }
        if !self.liquid_tokens.is_empty() {
            struct_ser.serialize_field("liquidTokens", &self.liquid_tokens)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for LiquidValidatorState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "operator_address",
            "operatorAddress",
            "weight",
            "status",
            "del_shares",
            "delShares",
            "liquid_tokens",
            "liquidTokens",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OperatorAddress,
            Weight,
            Status,
            DelShares,
            LiquidTokens,
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
                            "weight" => Ok(GeneratedField::Weight),
                            "status" => Ok(GeneratedField::Status),
                            "delShares" | "del_shares" => Ok(GeneratedField::DelShares),
                            "liquidTokens" | "liquid_tokens" => {
                                Ok(GeneratedField::LiquidTokens)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = LiquidValidatorState;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lspersistence.v1beta1.LiquidValidatorState",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<LiquidValidatorState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut operator_address__ = None;
                let mut weight__ = None;
                let mut status__ = None;
                let mut del_shares__ = None;
                let mut liquid_tokens__ = None;
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
                        GeneratedField::Weight => {
                            if weight__.is_some() {
                                return Err(serde::de::Error::duplicate_field("weight"));
                            }
                            weight__ = Some(map.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map.next_value::<ValidatorStatus>()? as i32);
                        }
                        GeneratedField::DelShares => {
                            if del_shares__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delShares"));
                            }
                            del_shares__ = Some(map.next_value()?);
                        }
                        GeneratedField::LiquidTokens => {
                            if liquid_tokens__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("liquidTokens"),
                                );
                            }
                            liquid_tokens__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(LiquidValidatorState {
                    operator_address: operator_address__.unwrap_or_default(),
                    weight: weight__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    del_shares: del_shares__.unwrap_or_default(),
                    liquid_tokens: liquid_tokens__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lspersistence.v1beta1.LiquidValidatorState",
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
            .serialize_struct("pstake.lspersistence.v1beta1.MsgLiquidStake", len)?;
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
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
                formatter.write_str("struct pstake.lspersistence.v1beta1.MsgLiquidStake")
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
                "pstake.lspersistence.v1beta1.MsgLiquidStake",
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
                "pstake.lspersistence.v1beta1.MsgLiquidStakeResponse",
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
        enum GeneratedField {}
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
                        Err(serde::de::Error::unknown_field(value, FIELDS))
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
                        "struct pstake.lspersistence.v1beta1.MsgLiquidStakeResponse",
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
                "pstake.lspersistence.v1beta1.MsgLiquidStakeResponse",
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
            .serialize_struct("pstake.lspersistence.v1beta1.MsgLiquidUnstake", len)?;
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
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
                    .write_str("struct pstake.lspersistence.v1beta1.MsgLiquidUnstake")
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
                "pstake.lspersistence.v1beta1.MsgLiquidUnstake",
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
        let mut len = 0;
        if self.completion_time.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lspersistence.v1beta1.MsgLiquidUnstakeResponse",
                len,
            )?;
        if let Some(v) = self.completion_time.as_ref() {
            struct_ser.serialize_field("completionTime", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgLiquidUnstakeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["completion_time", "completionTime"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            CompletionTime,
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
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
                        "struct pstake.lspersistence.v1beta1.MsgLiquidUnstakeResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgLiquidUnstakeResponse, V::Error>
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
                    }
                }
                Ok(MsgLiquidUnstakeResponse {
                    completion_time: completion_time__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lspersistence.v1beta1.MsgLiquidUnstakeResponse",
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
            .serialize_struct("pstake.lspersistence.v1beta1.MsgUpdateParams", len)?;
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
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
                    .write_str("struct pstake.lspersistence.v1beta1.MsgUpdateParams")
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
                "pstake.lspersistence.v1beta1.MsgUpdateParams",
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
                "pstake.lspersistence.v1beta1.MsgUpdateParamsResponse",
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
        enum GeneratedField {}
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
                        Err(serde::de::Error::unknown_field(value, FIELDS))
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
                        "struct pstake.lspersistence.v1beta1.MsgUpdateParamsResponse",
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
                "pstake.lspersistence.v1beta1.MsgUpdateParamsResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for NetAmountState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.mint_rate.is_empty() {
            len += 1;
        }
        if !self.btoken_total_supply.is_empty() {
            len += 1;
        }
        if !self.net_amount.is_empty() {
            len += 1;
        }
        if !self.total_del_shares.is_empty() {
            len += 1;
        }
        if !self.total_liquid_tokens.is_empty() {
            len += 1;
        }
        if !self.total_remaining_rewards.is_empty() {
            len += 1;
        }
        if !self.total_unbonding_balance.is_empty() {
            len += 1;
        }
        if !self.proxy_acc_balance.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lspersistence.v1beta1.NetAmountState", len)?;
        if !self.mint_rate.is_empty() {
            struct_ser.serialize_field("mintRate", &self.mint_rate)?;
        }
        if !self.btoken_total_supply.is_empty() {
            struct_ser.serialize_field("btokenTotalSupply", &self.btoken_total_supply)?;
        }
        if !self.net_amount.is_empty() {
            struct_ser.serialize_field("netAmount", &self.net_amount)?;
        }
        if !self.total_del_shares.is_empty() {
            struct_ser.serialize_field("totalDelShares", &self.total_del_shares)?;
        }
        if !self.total_liquid_tokens.is_empty() {
            struct_ser.serialize_field("totalLiquidTokens", &self.total_liquid_tokens)?;
        }
        if !self.total_remaining_rewards.is_empty() {
            struct_ser
                .serialize_field(
                    "totalRemainingRewards",
                    &self.total_remaining_rewards,
                )?;
        }
        if !self.total_unbonding_balance.is_empty() {
            struct_ser
                .serialize_field(
                    "totalUnbondingBalance",
                    &self.total_unbonding_balance,
                )?;
        }
        if !self.proxy_acc_balance.is_empty() {
            struct_ser.serialize_field("proxyAccBalance", &self.proxy_acc_balance)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for NetAmountState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "mint_rate",
            "mintRate",
            "btoken_total_supply",
            "btokenTotalSupply",
            "net_amount",
            "netAmount",
            "total_del_shares",
            "totalDelShares",
            "total_liquid_tokens",
            "totalLiquidTokens",
            "total_remaining_rewards",
            "totalRemainingRewards",
            "total_unbonding_balance",
            "totalUnbondingBalance",
            "proxy_acc_balance",
            "proxyAccBalance",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MintRate,
            BtokenTotalSupply,
            NetAmount,
            TotalDelShares,
            TotalLiquidTokens,
            TotalRemainingRewards,
            TotalUnbondingBalance,
            ProxyAccBalance,
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
                            "mintRate" | "mint_rate" => Ok(GeneratedField::MintRate),
                            "btokenTotalSupply" | "btoken_total_supply" => {
                                Ok(GeneratedField::BtokenTotalSupply)
                            }
                            "netAmount" | "net_amount" => Ok(GeneratedField::NetAmount),
                            "totalDelShares" | "total_del_shares" => {
                                Ok(GeneratedField::TotalDelShares)
                            }
                            "totalLiquidTokens" | "total_liquid_tokens" => {
                                Ok(GeneratedField::TotalLiquidTokens)
                            }
                            "totalRemainingRewards" | "total_remaining_rewards" => {
                                Ok(GeneratedField::TotalRemainingRewards)
                            }
                            "totalUnbondingBalance" | "total_unbonding_balance" => {
                                Ok(GeneratedField::TotalUnbondingBalance)
                            }
                            "proxyAccBalance" | "proxy_acc_balance" => {
                                Ok(GeneratedField::ProxyAccBalance)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = NetAmountState;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.lspersistence.v1beta1.NetAmountState")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<NetAmountState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut mint_rate__ = None;
                let mut btoken_total_supply__ = None;
                let mut net_amount__ = None;
                let mut total_del_shares__ = None;
                let mut total_liquid_tokens__ = None;
                let mut total_remaining_rewards__ = None;
                let mut total_unbonding_balance__ = None;
                let mut proxy_acc_balance__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MintRate => {
                            if mint_rate__.is_some() {
                                return Err(serde::de::Error::duplicate_field("mintRate"));
                            }
                            mint_rate__ = Some(map.next_value()?);
                        }
                        GeneratedField::BtokenTotalSupply => {
                            if btoken_total_supply__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("btokenTotalSupply"),
                                );
                            }
                            btoken_total_supply__ = Some(map.next_value()?);
                        }
                        GeneratedField::NetAmount => {
                            if net_amount__.is_some() {
                                return Err(serde::de::Error::duplicate_field("netAmount"));
                            }
                            net_amount__ = Some(map.next_value()?);
                        }
                        GeneratedField::TotalDelShares => {
                            if total_del_shares__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("totalDelShares"),
                                );
                            }
                            total_del_shares__ = Some(map.next_value()?);
                        }
                        GeneratedField::TotalLiquidTokens => {
                            if total_liquid_tokens__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("totalLiquidTokens"),
                                );
                            }
                            total_liquid_tokens__ = Some(map.next_value()?);
                        }
                        GeneratedField::TotalRemainingRewards => {
                            if total_remaining_rewards__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("totalRemainingRewards"),
                                );
                            }
                            total_remaining_rewards__ = Some(map.next_value()?);
                        }
                        GeneratedField::TotalUnbondingBalance => {
                            if total_unbonding_balance__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("totalUnbondingBalance"),
                                );
                            }
                            total_unbonding_balance__ = Some(map.next_value()?);
                        }
                        GeneratedField::ProxyAccBalance => {
                            if proxy_acc_balance__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("proxyAccBalance"),
                                );
                            }
                            proxy_acc_balance__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(NetAmountState {
                    mint_rate: mint_rate__.unwrap_or_default(),
                    btoken_total_supply: btoken_total_supply__.unwrap_or_default(),
                    net_amount: net_amount__.unwrap_or_default(),
                    total_del_shares: total_del_shares__.unwrap_or_default(),
                    total_liquid_tokens: total_liquid_tokens__.unwrap_or_default(),
                    total_remaining_rewards: total_remaining_rewards__
                        .unwrap_or_default(),
                    total_unbonding_balance: total_unbonding_balance__
                        .unwrap_or_default(),
                    proxy_acc_balance: proxy_acc_balance__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lspersistence.v1beta1.NetAmountState",
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
        if !self.liquid_bond_denom.is_empty() {
            len += 1;
        }
        if !self.whitelisted_validators.is_empty() {
            len += 1;
        }
        if !self.stake_fee_rate.is_empty() {
            len += 1;
        }
        if !self.unstake_fee_rate.is_empty() {
            len += 1;
        }
        if !self.redemption_fee_rate.is_empty() {
            len += 1;
        }
        if !self.restake_fee_rate.is_empty() {
            len += 1;
        }
        if !self.min_liquid_staking_amount.is_empty() {
            len += 1;
        }
        if !self.admin_address.is_empty() {
            len += 1;
        }
        if !self.fee_address.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lspersistence.v1beta1.Params", len)?;
        if !self.liquid_bond_denom.is_empty() {
            struct_ser.serialize_field("liquidBondDenom", &self.liquid_bond_denom)?;
        }
        if !self.whitelisted_validators.is_empty() {
            struct_ser
                .serialize_field("whitelistedValidators", &self.whitelisted_validators)?;
        }
        if !self.stake_fee_rate.is_empty() {
            struct_ser.serialize_field("stakeFeeRate", &self.stake_fee_rate)?;
        }
        if !self.unstake_fee_rate.is_empty() {
            struct_ser.serialize_field("unstakeFeeRate", &self.unstake_fee_rate)?;
        }
        if !self.redemption_fee_rate.is_empty() {
            struct_ser.serialize_field("redemptionFeeRate", &self.redemption_fee_rate)?;
        }
        if !self.restake_fee_rate.is_empty() {
            struct_ser.serialize_field("restakeFeeRate", &self.restake_fee_rate)?;
        }
        if !self.min_liquid_staking_amount.is_empty() {
            struct_ser
                .serialize_field(
                    "minLiquidStakingAmount",
                    &self.min_liquid_staking_amount,
                )?;
        }
        if !self.admin_address.is_empty() {
            struct_ser.serialize_field("adminAddress", &self.admin_address)?;
        }
        if !self.fee_address.is_empty() {
            struct_ser.serialize_field("feeAddress", &self.fee_address)?;
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
            "liquid_bond_denom",
            "liquidBondDenom",
            "whitelisted_validators",
            "whitelistedValidators",
            "stake_fee_rate",
            "stakeFeeRate",
            "unstake_fee_rate",
            "unstakeFeeRate",
            "redemption_fee_rate",
            "redemptionFeeRate",
            "restake_fee_rate",
            "restakeFeeRate",
            "min_liquid_staking_amount",
            "minLiquidStakingAmount",
            "admin_address",
            "adminAddress",
            "fee_address",
            "feeAddress",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LiquidBondDenom,
            WhitelistedValidators,
            StakeFeeRate,
            UnstakeFeeRate,
            RedemptionFeeRate,
            RestakeFeeRate,
            MinLiquidStakingAmount,
            AdminAddress,
            FeeAddress,
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
                            "liquidBondDenom" | "liquid_bond_denom" => {
                                Ok(GeneratedField::LiquidBondDenom)
                            }
                            "whitelistedValidators" | "whitelisted_validators" => {
                                Ok(GeneratedField::WhitelistedValidators)
                            }
                            "stakeFeeRate" | "stake_fee_rate" => {
                                Ok(GeneratedField::StakeFeeRate)
                            }
                            "unstakeFeeRate" | "unstake_fee_rate" => {
                                Ok(GeneratedField::UnstakeFeeRate)
                            }
                            "redemptionFeeRate" | "redemption_fee_rate" => {
                                Ok(GeneratedField::RedemptionFeeRate)
                            }
                            "restakeFeeRate" | "restake_fee_rate" => {
                                Ok(GeneratedField::RestakeFeeRate)
                            }
                            "minLiquidStakingAmount" | "min_liquid_staking_amount" => {
                                Ok(GeneratedField::MinLiquidStakingAmount)
                            }
                            "adminAddress" | "admin_address" => {
                                Ok(GeneratedField::AdminAddress)
                            }
                            "feeAddress" | "fee_address" => {
                                Ok(GeneratedField::FeeAddress)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
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
                formatter.write_str("struct pstake.lspersistence.v1beta1.Params")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Params, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut liquid_bond_denom__ = None;
                let mut whitelisted_validators__ = None;
                let mut stake_fee_rate__ = None;
                let mut unstake_fee_rate__ = None;
                let mut redemption_fee_rate__ = None;
                let mut restake_fee_rate__ = None;
                let mut min_liquid_staking_amount__ = None;
                let mut admin_address__ = None;
                let mut fee_address__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LiquidBondDenom => {
                            if liquid_bond_denom__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("liquidBondDenom"),
                                );
                            }
                            liquid_bond_denom__ = Some(map.next_value()?);
                        }
                        GeneratedField::WhitelistedValidators => {
                            if whitelisted_validators__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("whitelistedValidators"),
                                );
                            }
                            whitelisted_validators__ = Some(map.next_value()?);
                        }
                        GeneratedField::StakeFeeRate => {
                            if stake_fee_rate__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("stakeFeeRate"),
                                );
                            }
                            stake_fee_rate__ = Some(map.next_value()?);
                        }
                        GeneratedField::UnstakeFeeRate => {
                            if unstake_fee_rate__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("unstakeFeeRate"),
                                );
                            }
                            unstake_fee_rate__ = Some(map.next_value()?);
                        }
                        GeneratedField::RedemptionFeeRate => {
                            if redemption_fee_rate__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("redemptionFeeRate"),
                                );
                            }
                            redemption_fee_rate__ = Some(map.next_value()?);
                        }
                        GeneratedField::RestakeFeeRate => {
                            if restake_fee_rate__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("restakeFeeRate"),
                                );
                            }
                            restake_fee_rate__ = Some(map.next_value()?);
                        }
                        GeneratedField::MinLiquidStakingAmount => {
                            if min_liquid_staking_amount__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("minLiquidStakingAmount"),
                                );
                            }
                            min_liquid_staking_amount__ = Some(map.next_value()?);
                        }
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
                    }
                }
                Ok(Params {
                    liquid_bond_denom: liquid_bond_denom__.unwrap_or_default(),
                    whitelisted_validators: whitelisted_validators__.unwrap_or_default(),
                    stake_fee_rate: stake_fee_rate__.unwrap_or_default(),
                    unstake_fee_rate: unstake_fee_rate__.unwrap_or_default(),
                    redemption_fee_rate: redemption_fee_rate__.unwrap_or_default(),
                    restake_fee_rate: restake_fee_rate__.unwrap_or_default(),
                    min_liquid_staking_amount: min_liquid_staking_amount__
                        .unwrap_or_default(),
                    admin_address: admin_address__.unwrap_or_default(),
                    fee_address: fee_address__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lspersistence.v1beta1.Params",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryLiquidValidatorsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "pstake.lspersistence.v1beta1.QueryLiquidValidatorsRequest",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryLiquidValidatorsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLiquidValidatorsRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lspersistence.v1beta1.QueryLiquidValidatorsRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryLiquidValidatorsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryLiquidValidatorsRequest {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lspersistence.v1beta1.QueryLiquidValidatorsRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryLiquidValidatorsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.liquid_validators.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "pstake.lspersistence.v1beta1.QueryLiquidValidatorsResponse",
                len,
            )?;
        if !self.liquid_validators.is_empty() {
            struct_ser.serialize_field("liquidValidators", &self.liquid_validators)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryLiquidValidatorsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["liquid_validators", "liquidValidators"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            LiquidValidators,
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
                            "liquidValidators" | "liquid_validators" => {
                                Ok(GeneratedField::LiquidValidators)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryLiquidValidatorsResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lspersistence.v1beta1.QueryLiquidValidatorsResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryLiquidValidatorsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut liquid_validators__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::LiquidValidators => {
                            if liquid_validators__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("liquidValidators"),
                                );
                            }
                            liquid_validators__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(QueryLiquidValidatorsResponse {
                    liquid_validators: liquid_validators__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lspersistence.v1beta1.QueryLiquidValidatorsResponse",
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
            .serialize_struct("pstake.lspersistence.v1beta1.QueryParamsRequest", len)?;
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
        enum GeneratedField {}
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
                        Err(serde::de::Error::unknown_field(value, FIELDS))
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
                    .write_str("struct pstake.lspersistence.v1beta1.QueryParamsRequest")
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
                "pstake.lspersistence.v1beta1.QueryParamsRequest",
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
            .serialize_struct("pstake.lspersistence.v1beta1.QueryParamsResponse", len)?;
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
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
                    .write_str("struct pstake.lspersistence.v1beta1.QueryParamsResponse")
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
                    }
                }
                Ok(QueryParamsResponse {
                    params: params__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lspersistence.v1beta1.QueryParamsResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryStatesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct("pstake.lspersistence.v1beta1.QueryStatesRequest", len)?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryStatesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {}
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
                        Err(serde::de::Error::unknown_field(value, FIELDS))
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryStatesRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.lspersistence.v1beta1.QueryStatesRequest")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryStatesRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(QueryStatesRequest {})
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lspersistence.v1beta1.QueryStatesRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryStatesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.net_amount_state.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lspersistence.v1beta1.QueryStatesResponse", len)?;
        if let Some(v) = self.net_amount_state.as_ref() {
            struct_ser.serialize_field("netAmountState", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryStatesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["net_amount_state", "netAmountState"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NetAmountState,
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
                            "netAmountState" | "net_amount_state" => {
                                Ok(GeneratedField::NetAmountState)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryStatesResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct pstake.lspersistence.v1beta1.QueryStatesResponse")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryStatesResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut net_amount_state__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::NetAmountState => {
                            if net_amount_state__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("netAmountState"),
                                );
                            }
                            net_amount_state__ = map.next_value()?;
                        }
                    }
                }
                Ok(QueryStatesResponse {
                    net_amount_state: net_amount_state__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lspersistence.v1beta1.QueryStatesResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for ValidatorStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "VALIDATOR_STATUS_UNSPECIFIED",
            Self::Active => "VALIDATOR_STATUS_ACTIVE",
            Self::Inactive => "VALIDATOR_STATUS_INACTIVE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ValidatorStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "VALIDATOR_STATUS_UNSPECIFIED",
            "VALIDATOR_STATUS_ACTIVE",
            "VALIDATOR_STATUS_INACTIVE",
        ];
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ValidatorStatus;
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
                    .and_then(ValidatorStatus::from_i32)
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
                    .and_then(ValidatorStatus::from_i32)
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
                    "VALIDATOR_STATUS_UNSPECIFIED" => Ok(ValidatorStatus::Unspecified),
                    "VALIDATOR_STATUS_ACTIVE" => Ok(ValidatorStatus::Active),
                    "VALIDATOR_STATUS_INACTIVE" => Ok(ValidatorStatus::Inactive),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for VotingPower {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.voter.is_empty() {
            len += 1;
        }
        if !self.staking_voting_power.is_empty() {
            len += 1;
        }
        if !self.liquid_staking_voting_power.is_empty() {
            len += 1;
        }
        if !self.validator_voting_power.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("pstake.lspersistence.v1beta1.VotingPower", len)?;
        if !self.voter.is_empty() {
            struct_ser.serialize_field("voter", &self.voter)?;
        }
        if !self.staking_voting_power.is_empty() {
            struct_ser
                .serialize_field("stakingVotingPower", &self.staking_voting_power)?;
        }
        if !self.liquid_staking_voting_power.is_empty() {
            struct_ser
                .serialize_field(
                    "liquidStakingVotingPower",
                    &self.liquid_staking_voting_power,
                )?;
        }
        if !self.validator_voting_power.is_empty() {
            struct_ser
                .serialize_field("validatorVotingPower", &self.validator_voting_power)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for VotingPower {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "voter",
            "staking_voting_power",
            "stakingVotingPower",
            "liquid_staking_voting_power",
            "liquidStakingVotingPower",
            "validator_voting_power",
            "validatorVotingPower",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Voter,
            StakingVotingPower,
            LiquidStakingVotingPower,
            ValidatorVotingPower,
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
                            "voter" => Ok(GeneratedField::Voter),
                            "stakingVotingPower" | "staking_voting_power" => {
                                Ok(GeneratedField::StakingVotingPower)
                            }
                            "liquidStakingVotingPower"
                            | "liquid_staking_voting_power" => {
                                Ok(GeneratedField::LiquidStakingVotingPower)
                            }
                            "validatorVotingPower" | "validator_voting_power" => {
                                Ok(GeneratedField::ValidatorVotingPower)
                            }
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = VotingPower;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct pstake.lspersistence.v1beta1.VotingPower")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<VotingPower, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut voter__ = None;
                let mut staking_voting_power__ = None;
                let mut liquid_staking_voting_power__ = None;
                let mut validator_voting_power__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Voter => {
                            if voter__.is_some() {
                                return Err(serde::de::Error::duplicate_field("voter"));
                            }
                            voter__ = Some(map.next_value()?);
                        }
                        GeneratedField::StakingVotingPower => {
                            if staking_voting_power__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("stakingVotingPower"),
                                );
                            }
                            staking_voting_power__ = Some(map.next_value()?);
                        }
                        GeneratedField::LiquidStakingVotingPower => {
                            if liquid_staking_voting_power__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field(
                                        "liquidStakingVotingPower",
                                    ),
                                );
                            }
                            liquid_staking_voting_power__ = Some(map.next_value()?);
                        }
                        GeneratedField::ValidatorVotingPower => {
                            if validator_voting_power__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("validatorVotingPower"),
                                );
                            }
                            validator_voting_power__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(VotingPower {
                    voter: voter__.unwrap_or_default(),
                    staking_voting_power: staking_voting_power__.unwrap_or_default(),
                    liquid_staking_voting_power: liquid_staking_voting_power__
                        .unwrap_or_default(),
                    validator_voting_power: validator_voting_power__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lspersistence.v1beta1.VotingPower",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for WhitelistedValidator {
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
            .serialize_struct("pstake.lspersistence.v1beta1.WhitelistedValidator", len)?;
        if !self.validator_address.is_empty() {
            struct_ser.serialize_field("validatorAddress", &self.validator_address)?;
        }
        if !self.target_weight.is_empty() {
            struct_ser.serialize_field("targetWeight", &self.target_weight)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for WhitelistedValidator {
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
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = WhitelistedValidator;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct pstake.lspersistence.v1beta1.WhitelistedValidator",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<WhitelistedValidator, V::Error>
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
                    }
                }
                Ok(WhitelistedValidator {
                    validator_address: validator_address__.unwrap_or_default(),
                    target_weight: target_weight__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "pstake.lspersistence.v1beta1.WhitelistedValidator",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
