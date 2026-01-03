use serde::{Deserialize, Deserializer, de};
use serde_json::Value;

#[derive(Deserialize)]
#[serde(untagged)]
enum FlexiBool {
    Bool(bool),
    String(String),
}

#[derive(Deserialize)]
#[serde(untagged)]
enum FlexiString {
    String(String),
    Number(u64),
}

pub fn empty_is_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.filter(|s| !s.is_empty()))
}

pub fn optional_u64<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<FlexiString>::deserialize(deserializer)? {
        Some(FlexiString::String(s)) => {
            if s.trim().is_empty() {
                Ok(None)
            } else {
                Ok(Some(s.parse::<u64>().unwrap()))
            }
        }
        Some(FlexiString::Number(n)) => Ok(Some(n)),
        None => Ok(None),
    }
}

pub fn flexible_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::<FlexiBool>::deserialize(deserializer)? {
        Some(FlexiBool::Bool(b)) => Ok(Some(b)),
        Some(FlexiBool::String(s)) => {
            match s.to_lowercase().as_str() {
                "true" | "yes" | "1" => Ok(Some(true)),
                "false" | "no" | "0" => Ok(Some(false)),
                "" => Ok(None), // Empty string becomes None
                _ => Err(de::Error::invalid_value(
                    de::Unexpected::Str(&s),
                    &"a valid boolean (true/false, yes/no, 1/0)",
                )),
            }
        }
        None => Ok(None),
    }
}

pub fn normalize_resource<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let v: Option<Value> = Option::deserialize(deserializer)?;
    match v {
        Some(Value::String(s)) => {
            if s.trim().is_empty() {
                Ok(None)
            } else {
                Ok(Some(s))
            }
        }
        Some(Value::Object(map)) => {
            if let Some(Value::String(url)) = map.get("url") {
                Ok(Some(url.clone()))
            } else {
                Ok(None)
            }
        }
        _ => Ok(None),
    }
}
