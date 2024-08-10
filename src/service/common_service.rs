use serde::{de::Unexpected, Deserialize, Deserializer};

pub struct CommonService;

impl CommonService {
    // int 类型转boolean
    pub fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: Deserializer<'de>,
    {
        match u64::deserialize(deserializer)? {
            0 => Ok(false),
            1 => Ok(true),
            other => Err(serde::de::Error::invalid_value(
                Unexpected::Unsigned(other),
                &"0 or 1",
            )),
        }
    }
}
