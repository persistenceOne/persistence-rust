impl serde::Serialize for DataPoint {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.remote_height.is_empty() {
            len += 1;
        }
        if !self.local_height.is_empty() {
            len += 1;
        }
        if !self.value.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("persistence.interchainquery.v1beta1.DataPoint", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.remote_height.is_empty() {
            struct_ser.serialize_field("remoteHeight", &self.remote_height)?;
        }
        if !self.local_height.is_empty() {
            struct_ser.serialize_field("localHeight", &self.local_height)?;
        }
        if !self.value.is_empty() {
            struct_ser
                .serialize_field(
                    "value",
                    pbjson::private::base64::encode(&self.value).as_str(),
                )?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DataPoint {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "remote_height",
            "remoteHeight",
            "local_height",
            "localHeight",
            "value",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            RemoteHeight,
            LocalHeight,
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
                            "id" => Ok(GeneratedField::Id),
                            "remoteHeight" | "remote_height" => {
                                Ok(GeneratedField::RemoteHeight)
                            }
                            "localHeight" | "local_height" => {
                                Ok(GeneratedField::LocalHeight)
                            }
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
            type Value = DataPoint;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str("struct persistence.interchainquery.v1beta1.DataPoint")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<DataPoint, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut remote_height__ = None;
                let mut local_height__ = None;
                let mut value__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::RemoteHeight => {
                            if remote_height__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("remoteHeight"),
                                );
                            }
                            remote_height__ = Some(map.next_value()?);
                        }
                        GeneratedField::LocalHeight => {
                            if local_height__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("localHeight"),
                                );
                            }
                            local_height__ = Some(map.next_value()?);
                        }
                        GeneratedField::Value => {
                            if value__.is_some() {
                                return Err(serde::de::Error::duplicate_field("value"));
                            }
                            value__ = Some(
                                map
                                    .next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(DataPoint {
                    id: id__.unwrap_or_default(),
                    remote_height: remote_height__.unwrap_or_default(),
                    local_height: local_height__.unwrap_or_default(),
                    value: value__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.interchainquery.v1beta1.DataPoint",
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
        if !self.queries.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("persistence.interchainquery.v1beta1.GenesisState", len)?;
        if !self.queries.is_empty() {
            struct_ser.serialize_field("queries", &self.queries)?;
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
        const FIELDS: &[&str] = &["queries"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Queries,
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
                            "queries" => Ok(GeneratedField::Queries),
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
                formatter
                    .write_str("struct persistence.interchainquery.v1beta1.GenesisState")
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<GenesisState, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut queries__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Queries => {
                            if queries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queries"));
                            }
                            queries__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(GenesisState {
                    queries: queries__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.interchainquery.v1beta1.GenesisState",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
// impl serde::Serialize for GetTxWithProofResponse {
//     #[allow(deprecated)]
//     fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         use serde::ser::SerializeStruct;
//         let mut len = 0;
//         if self.tx.is_some() {
//             len += 1;
//         }
//         if self.tx_response.is_some() {
//             len += 1;
//         }
//         if self.proof.is_some() {
//             len += 1;
//         }
//         if self.header.is_some() {
//             len += 1;
//         }
//         let mut struct_ser = serializer
//             .serialize_struct(
//                 "persistence.interchainquery.v1beta1.GetTxWithProofResponse",
//                 len,
//             )?;
//         if let Some(v) = self.tx.as_ref() {
//             struct_ser.serialize_field("tx", v)?;
//         }
//         if let Some(v) = self.tx_response.as_ref() {
//             struct_ser.serialize_field("txResponse", v)?;
//         }
//         if let Some(v) = self.proof.as_ref() {
//             struct_ser.serialize_field("proof", v)?;
//         }
//         if let Some(v) = self.header.as_ref() {
//             struct_ser.serialize_field("header", v)?;
//         }
//         struct_ser.end()
//     }
// }
// impl<'de> serde::Deserialize<'de> for GetTxWithProofResponse {
//     #[allow(deprecated)]
//     fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         const FIELDS: &[&str] = &["tx", "tx_response", "txResponse", "proof", "header"];
//         #[allow(clippy::enum_variant_names)]
//         enum GeneratedField {
//             Tx,
//             TxResponse,
//             Proof,
//             Header,
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
//                             "tx" => Ok(GeneratedField::Tx),
//                             "txResponse" | "tx_response" => {
//                                 Ok(GeneratedField::TxResponse)
//                             }
//                             "proof" => Ok(GeneratedField::Proof),
//                             "header" => Ok(GeneratedField::Header),
//                             _ => Ok(GeneratedField::__SkipField__),
//                         }
//                     }
//                 }
//                 deserializer.deserialize_identifier(GeneratedVisitor)
//             }
//         }
//         struct GeneratedVisitor;
//         impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
//             type Value = GetTxWithProofResponse;
//             fn expecting(
//                 &self,
//                 formatter: &mut std::fmt::Formatter<'_>,
//             ) -> std::fmt::Result {
//                 formatter
//                     .write_str(
//                         "struct persistence.interchainquery.v1beta1.GetTxWithProofResponse",
//                     )
//             }
//             fn visit_map<V>(
//                 self,
//                 mut map: V,
//             ) -> std::result::Result<GetTxWithProofResponse, V::Error>
//             where
//                 V: serde::de::MapAccess<'de>,
//             {
//                 let mut tx__ = None;
//                 let mut tx_response__ = None;
//                 let mut proof__ = None;
//                 let mut header__ = None;
//                 while let Some(k) = map.next_key()? {
//                     match k {
//                         GeneratedField::Tx => {
//                             if tx__.is_some() {
//                                 return Err(serde::de::Error::duplicate_field("tx"));
//                             }
//                             tx__ = map.next_value()?;
//                         }
//                         GeneratedField::TxResponse => {
//                             if tx_response__.is_some() {
//                                 return Err(serde::de::Error::duplicate_field("txResponse"));
//                             }
//                             tx_response__ = map.next_value()?;
//                         }
//                         GeneratedField::Proof => {
//                             if proof__.is_some() {
//                                 return Err(serde::de::Error::duplicate_field("proof"));
//                             }
//                             proof__ = map.next_value()?;
//                         }
//                         GeneratedField::Header => {
//                             if header__.is_some() {
//                                 return Err(serde::de::Error::duplicate_field("header"));
//                             }
//                             header__ = map.next_value()?;
//                         }
//                         GeneratedField::__SkipField__ => {
//                             let _ = map.next_value::<serde::de::IgnoredAny>()?;
//                         }
//                     }
//                 }
//                 Ok(GetTxWithProofResponse {
//                     tx: tx__,
//                     tx_response: tx_response__,
//                     proof: proof__,
//                     header: header__,
//                 })
//             }
//         }
//         deserializer
//             .deserialize_struct(
//                 "persistence.interchainquery.v1beta1.GetTxWithProofResponse",
//                 FIELDS,
//                 GeneratedVisitor,
//             )
//     }
// }
// impl serde::Serialize for MsgSubmitQueryResponse {
//     #[allow(deprecated)]
//     fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         use serde::ser::SerializeStruct;
//         let mut len = 0;
//         if !self.chain_id.is_empty() {
//             len += 1;
//         }
//         if !self.query_id.is_empty() {
//             len += 1;
//         }
//         if !self.result.is_empty() {
//             len += 1;
//         }
//         if self.proof_ops.is_some() {
//             len += 1;
//         }
//         if self.height != 0 {
//             len += 1;
//         }
//         if !self.from_address.is_empty() {
//             len += 1;
//         }
//         let mut struct_ser = serializer
//             .serialize_struct(
//                 "persistence.interchainquery.v1beta1.MsgSubmitQueryResponse",
//                 len,
//             )?;
//         if !self.chain_id.is_empty() {
//             struct_ser.serialize_field("chainId", &self.chain_id)?;
//         }
//         if !self.query_id.is_empty() {
//             struct_ser.serialize_field("queryId", &self.query_id)?;
//         }
//         if !self.result.is_empty() {
//             struct_ser
//                 .serialize_field(
//                     "result",
//                     pbjson::private::base64::encode(&self.result).as_str(),
//                 )?;
//         }
//         if let Some(v) = self.proof_ops.as_ref() {
//             struct_ser.serialize_field("proofOps", v)?;
//         }
//         if self.height != 0 {
//             struct_ser
//                 .serialize_field("height", ToString::to_string(&self.height).as_str())?;
//         }
//         if !self.from_address.is_empty() {
//             struct_ser.serialize_field("fromAddress", &self.from_address)?;
//         }
//         struct_ser.end()
//     }
// }
// impl<'de> serde::Deserialize<'de> for MsgSubmitQueryResponse {
//     #[allow(deprecated)]
//     fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de>,
//     {
//         const FIELDS: &[&str] = &[
//             "chain_id",
//             "chainId",
//             "query_id",
//             "queryId",
//             "result",
//             "proof_ops",
//             "proofOps",
//             "height",
//             "from_address",
//             "fromAddress",
//         ];
//         #[allow(clippy::enum_variant_names)]
//         enum GeneratedField {
//             ChainId,
//             QueryId,
//             Result,
//             ProofOps,
//             Height,
//             FromAddress,
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
//                             "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
//                             "queryId" | "query_id" => Ok(GeneratedField::QueryId),
//                             "result" => Ok(GeneratedField::Result),
//                             "proofOps" | "proof_ops" => Ok(GeneratedField::ProofOps),
//                             "height" => Ok(GeneratedField::Height),
//                             "fromAddress" | "from_address" => {
//                                 Ok(GeneratedField::FromAddress)
//                             }
//                             _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
//                         }
//                     }
//                 }
//                 deserializer.deserialize_identifier(GeneratedVisitor)
//             }
//         }
//         struct GeneratedVisitor;
//         impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
//             type Value = MsgSubmitQueryResponse;
//             fn expecting(
//                 &self,
//                 formatter: &mut std::fmt::Formatter<'_>,
//             ) -> std::fmt::Result {
//                 formatter
//                     .write_str(
//                         "struct persistence.interchainquery.v1beta1.MsgSubmitQueryResponse",
//                     )
//             }
//             fn visit_map<V>(
//                 self,
//                 mut map: V,
//             ) -> std::result::Result<MsgSubmitQueryResponse, V::Error>
//             where
//                 V: serde::de::MapAccess<'de>,
//             {
//                 let mut chain_id__ = None;
//                 let mut query_id__ = None;
//                 let mut result__ = None;
//                 let mut proof_ops__ = None;
//                 let mut height__ = None;
//                 let mut from_address__ = None;
//                 while let Some(k) = map.next_key()? {
//                     match k {
//                         GeneratedField::ChainId => {
//                             if chain_id__.is_some() {
//                                 return Err(serde::de::Error::duplicate_field("chainId"));
//                             }
//                             chain_id__ = Some(map.next_value()?);
//                         }
//                         GeneratedField::QueryId => {
//                             if query_id__.is_some() {
//                                 return Err(serde::de::Error::duplicate_field("queryId"));
//                             }
//                             query_id__ = Some(map.next_value()?);
//                         }
//                         GeneratedField::Result => {
//                             if result__.is_some() {
//                                 return Err(serde::de::Error::duplicate_field("result"));
//                             }
//                             result__ = Some(
//                                 map
//                                     .next_value::<::pbjson::private::BytesDeserialize<_>>()?
//                                     .0,
//                             );
//                         }
//                         GeneratedField::ProofOps => {
//                             if proof_ops__.is_some() {
//                                 return Err(serde::de::Error::duplicate_field("proofOps"));
//                             }
//                             proof_ops__ = map.next_value()?;
//                         }
//                         GeneratedField::Height => {
//                             if height__.is_some() {
//                                 return Err(serde::de::Error::duplicate_field("height"));
//                             }
//                             height__ = Some(
//                                 map
//                                     .next_value::<::pbjson::private::NumberDeserialize<_>>()?
//                                     .0,
//                             );
//                         }
//                         GeneratedField::FromAddress => {
//                             if from_address__.is_some() {
//                                 return Err(
//                                     serde::de::Error::duplicate_field("fromAddress"),
//                                 );
//                             }
//                             from_address__ = Some(map.next_value()?);
//                         }
//                     }
//                 }
//                 Ok(MsgSubmitQueryResponse {
//                     chain_id: chain_id__.unwrap_or_default(),
//                     query_id: query_id__.unwrap_or_default(),
//                     result: result__.unwrap_or_default(),
//                     proof_ops: proof_ops__,
//                     height: height__.unwrap_or_default(),
//                     from_address: from_address__.unwrap_or_default(),
//                 })
//             }
//         }
//         deserializer
//             .deserialize_struct(
//                 "persistence.interchainquery.v1beta1.MsgSubmitQueryResponse",
//                 FIELDS,
//                 GeneratedVisitor,
//             )
//     }
// }
impl serde::Serialize for MsgSubmitQueryResponseResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let len = 0;
        let struct_ser = serializer
            .serialize_struct(
                "persistence.interchainquery.v1beta1.MsgSubmitQueryResponseResponse",
                len,
            )?;
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MsgSubmitQueryResponseResponse {
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
            type Value = MsgSubmitQueryResponseResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.interchainquery.v1beta1.MsgSubmitQueryResponseResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<MsgSubmitQueryResponseResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                while map.next_key::<GeneratedField>()?.is_some() {
                    let _ = map.next_value::<serde::de::IgnoredAny>()?;
                }
                Ok(MsgSubmitQueryResponseResponse {})
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.interchainquery.v1beta1.MsgSubmitQueryResponseResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for Query {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.id.is_empty() {
            len += 1;
        }
        if !self.connection_id.is_empty() {
            len += 1;
        }
        if !self.chain_id.is_empty() {
            len += 1;
        }
        if !self.query_type.is_empty() {
            len += 1;
        }
        if !self.request.is_empty() {
            len += 1;
        }
        if !self.period.is_empty() {
            len += 1;
        }
        if !self.last_height.is_empty() {
            len += 1;
        }
        if !self.callback_id.is_empty() {
            len += 1;
        }
        if self.ttl != 0 {
            len += 1;
        }
        if !self.last_emission.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct("persistence.interchainquery.v1beta1.Query", len)?;
        if !self.id.is_empty() {
            struct_ser.serialize_field("id", &self.id)?;
        }
        if !self.connection_id.is_empty() {
            struct_ser.serialize_field("connectionId", &self.connection_id)?;
        }
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        if !self.query_type.is_empty() {
            struct_ser.serialize_field("queryType", &self.query_type)?;
        }
        if !self.request.is_empty() {
            struct_ser
                .serialize_field(
                    "request",
                    pbjson::private::base64::encode(&self.request).as_str(),
                )?;
        }
        if !self.period.is_empty() {
            struct_ser.serialize_field("period", &self.period)?;
        }
        if !self.last_height.is_empty() {
            struct_ser.serialize_field("lastHeight", &self.last_height)?;
        }
        if !self.callback_id.is_empty() {
            struct_ser.serialize_field("callbackId", &self.callback_id)?;
        }
        if self.ttl != 0 {
            struct_ser.serialize_field("ttl", ToString::to_string(&self.ttl).as_str())?;
        }
        if !self.last_emission.is_empty() {
            struct_ser.serialize_field("lastEmission", &self.last_emission)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Query {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "id",
            "connection_id",
            "connectionId",
            "chain_id",
            "chainId",
            "query_type",
            "queryType",
            "request",
            "period",
            "last_height",
            "lastHeight",
            "callback_id",
            "callbackId",
            "ttl",
            "last_emission",
            "lastEmission",
        ];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Id,
            ConnectionId,
            ChainId,
            QueryType,
            Request,
            Period,
            LastHeight,
            CallbackId,
            Ttl,
            LastEmission,
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
                            "connectionId" | "connection_id" => {
                                Ok(GeneratedField::ConnectionId)
                            }
                            "chainId" | "chain_id" => Ok(GeneratedField::ChainId),
                            "queryType" | "query_type" => Ok(GeneratedField::QueryType),
                            "request" => Ok(GeneratedField::Request),
                            "period" => Ok(GeneratedField::Period),
                            "lastHeight" | "last_height" => {
                                Ok(GeneratedField::LastHeight)
                            }
                            "callbackId" | "callback_id" => {
                                Ok(GeneratedField::CallbackId)
                            }
                            "ttl" => Ok(GeneratedField::Ttl),
                            "lastEmission" | "last_emission" => {
                                Ok(GeneratedField::LastEmission)
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
            type Value = Query;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter.write_str("struct persistence.interchainquery.v1beta1.Query")
            }
            fn visit_map<V>(self, mut map: V) -> std::result::Result<Query, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut id__ = None;
                let mut connection_id__ = None;
                let mut chain_id__ = None;
                let mut query_type__ = None;
                let mut request__ = None;
                let mut period__ = None;
                let mut last_height__ = None;
                let mut callback_id__ = None;
                let mut ttl__ = None;
                let mut last_emission__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Id => {
                            if id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("id"));
                            }
                            id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ConnectionId => {
                            if connection_id__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("connectionId"),
                                );
                            }
                            connection_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::ChainId => {
                            if chain_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("chainId"));
                            }
                            chain_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::QueryType => {
                            if query_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queryType"));
                            }
                            query_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::Request => {
                            if request__.is_some() {
                                return Err(serde::de::Error::duplicate_field("request"));
                            }
                            request__ = Some(
                                map
                                    .next_value::<::pbjson::private::BytesDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::Period => {
                            if period__.is_some() {
                                return Err(serde::de::Error::duplicate_field("period"));
                            }
                            period__ = Some(map.next_value()?);
                        }
                        GeneratedField::LastHeight => {
                            if last_height__.is_some() {
                                return Err(serde::de::Error::duplicate_field("lastHeight"));
                            }
                            last_height__ = Some(map.next_value()?);
                        }
                        GeneratedField::CallbackId => {
                            if callback_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("callbackId"));
                            }
                            callback_id__ = Some(map.next_value()?);
                        }
                        GeneratedField::Ttl => {
                            if ttl__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ttl"));
                            }
                            ttl__ = Some(
                                map
                                    .next_value::<::pbjson::private::NumberDeserialize<_>>()?
                                    .0,
                            );
                        }
                        GeneratedField::LastEmission => {
                            if last_emission__.is_some() {
                                return Err(
                                    serde::de::Error::duplicate_field("lastEmission"),
                                );
                            }
                            last_emission__ = Some(map.next_value()?);
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(Query {
                    id: id__.unwrap_or_default(),
                    connection_id: connection_id__.unwrap_or_default(),
                    chain_id: chain_id__.unwrap_or_default(),
                    query_type: query_type__.unwrap_or_default(),
                    request: request__.unwrap_or_default(),
                    period: period__.unwrap_or_default(),
                    last_height: last_height__.unwrap_or_default(),
                    callback_id: callback_id__.unwrap_or_default(),
                    ttl: ttl__.unwrap_or_default(),
                    last_emission: last_emission__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.interchainquery.v1beta1.Query",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryRequestsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.pagination.is_some() {
            len += 1;
        }
        if !self.chain_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.interchainquery.v1beta1.QueryRequestsRequest",
                len,
            )?;
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        if !self.chain_id.is_empty() {
            struct_ser.serialize_field("chainId", &self.chain_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRequestsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["pagination", "chain_id", "chainId"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Pagination,
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
                            "pagination" => Ok(GeneratedField::Pagination),
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
            type Value = QueryRequestsRequest;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.interchainquery.v1beta1.QueryRequestsRequest",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryRequestsRequest, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut pagination__ = None;
                let mut chain_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
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
                Ok(QueryRequestsRequest {
                    pagination: pagination__,
                    chain_id: chain_id__.unwrap_or_default(),
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.interchainquery.v1beta1.QueryRequestsRequest",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
impl serde::Serialize for QueryRequestsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.queries.is_empty() {
            len += 1;
        }
        if self.pagination.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer
            .serialize_struct(
                "persistence.interchainquery.v1beta1.QueryRequestsResponse",
                len,
            )?;
        if !self.queries.is_empty() {
            struct_ser.serialize_field("queries", &self.queries)?;
        }
        if let Some(v) = self.pagination.as_ref() {
            struct_ser.serialize_field("pagination", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for QueryRequestsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &["queries", "pagination"];
        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Queries,
            Pagination,
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
                            "queries" => Ok(GeneratedField::Queries),
                            "pagination" => Ok(GeneratedField::Pagination),
                            _ => Ok(GeneratedField::__SkipField__),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = QueryRequestsResponse;
            fn expecting(
                &self,
                formatter: &mut std::fmt::Formatter<'_>,
            ) -> std::fmt::Result {
                formatter
                    .write_str(
                        "struct persistence.interchainquery.v1beta1.QueryRequestsResponse",
                    )
            }
            fn visit_map<V>(
                self,
                mut map: V,
            ) -> std::result::Result<QueryRequestsResponse, V::Error>
            where
                V: serde::de::MapAccess<'de>,
            {
                let mut queries__ = None;
                let mut pagination__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Queries => {
                            if queries__.is_some() {
                                return Err(serde::de::Error::duplicate_field("queries"));
                            }
                            queries__ = Some(map.next_value()?);
                        }
                        GeneratedField::Pagination => {
                            if pagination__.is_some() {
                                return Err(serde::de::Error::duplicate_field("pagination"));
                            }
                            pagination__ = map.next_value()?;
                        }
                        GeneratedField::__SkipField__ => {
                            let _ = map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                Ok(QueryRequestsResponse {
                    queries: queries__.unwrap_or_default(),
                    pagination: pagination__,
                })
            }
        }
        deserializer
            .deserialize_struct(
                "persistence.interchainquery.v1beta1.QueryRequestsResponse",
                FIELDS,
                GeneratedVisitor,
            )
    }
}
