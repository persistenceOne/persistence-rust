pub mod as_str {
    use serde::{de, Deserialize, Deserializer, Serializer};
    use std::{fmt::Display, str::FromStr};

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: FromStr,
        T::Err: Display,
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        T::from_str(&s).map_err(de::Error::custom)
    }

    pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        T: Display,
    {
        serializer.serialize_str(&value.to_string())
    }
}


pub mod as_map {
    use serde::{Deserialize, Deserializer};
    use serde_cw_value::Value;

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: From<Vec<u8>>,
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;

        match value {
            Value::Bytes(b) => Ok(T::from(b)),
            _ => {
                // convert this to jsons string
                let s = serde_json_wasm::to_string(&value).unwrap();
                Ok(T::from(s.as_bytes().to_vec()))
            }
        }

    }
}