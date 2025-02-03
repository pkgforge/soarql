use serde::{Deserialize, Deserializer, Serialize, de};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ProvideStrategy {
    KeepTargetOnly,
    KeepBoth,
    Alias,
}

#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct PackageProvide {
    pub name: String,
    pub target: Option<String>,
    pub strategy: Option<ProvideStrategy>,
}

impl PackageProvide {
    pub fn from_string(provide: &str) -> Self {
        if let Some((name, target_name)) = provide.split_once("==") {
            Self {
                name: name.to_string(),
                target: Some(target_name.to_string()),
                strategy: Some(ProvideStrategy::KeepBoth),
            }
        } else if let Some((name, target_name)) = provide.split_once("=>") {
            Self {
                name: name.to_string(),
                target: Some(target_name.to_string()),
                strategy: Some(ProvideStrategy::KeepTargetOnly),
            }
        } else if let Some((name, target_name)) = provide.split_once(":") {
            Self {
                name: name.to_string(),
                target: Some(target_name.to_string()),
                strategy: Some(ProvideStrategy::Alias),
            }
        } else {
            Self {
                name: provide.to_string(),
                target: None,
                strategy: None,
            }
        }
    }
}

fn empty_is_none<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.filter(|s| !s.is_empty()))
}

fn optional_number<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    Ok(s.filter(|s| !s.is_empty())
        .and_then(|s| s.parse::<i64>().ok())
        .filter(|&n| n >= 0)
        .map(|n| n as u64))
}

fn number_from_string<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    s.parse::<i64>()
        .map_err(|_| de::Error::invalid_value(de::Unexpected::Str(&s), &"a valid number"))
        .and_then(|n| {
            if n >= 0 {
                Ok(n as u64)
            } else {
                Err(de::Error::invalid_value(
                    de::Unexpected::Signed(n),
                    &"a non-negative u64",
                ))
            }
        })
}

fn boolean_from_string<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "true" | "yes" | "1" => Ok(true),
        "false" | "no" | "0" => Ok(false),
        _ => Err(de::Error::invalid_value(
            de::Unexpected::Str(&s),
            &"a valid boolean (true/false, yes/no, 1/0)",
        )),
    }
}

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct RemotePackage {
    #[serde(deserialize_with = "boolean_from_string", alias = "_disabled")]
    pub disabled: bool,

    #[serde(alias = "_disabled_reason")]
    pub disabled_reason: Option<serde_json::Value>,

    #[serde(default, deserialize_with = "optional_number")]
    pub rank: Option<u64>,

    pub pkg: String,
    pub pkg_id: String,
    pub pkg_name: String,
    pub pkg_family: String,
    pub pkg_type: String,

    #[serde(default, deserialize_with = "empty_is_none")]
    pub pkg_webpage: Option<String>,

    pub description: String,
    pub version: String,
    pub version_upstream: Option<String>,
    pub download_url: String,

    #[serde(default, deserialize_with = "number_from_string")]
    pub size_raw: u64,

    #[serde(default, deserialize_with = "empty_is_none")]
    pub ghcr_pkg: Option<String>,

    #[serde(default, deserialize_with = "optional_number")]
    pub ghcr_size_raw: Option<u64>,

    pub ghcr_files: Option<Vec<String>>,
    pub ghcr_blob: Option<String>,
    pub ghcr_url: Option<String>,

    #[serde(alias = "src_url")]
    pub src_urls: Vec<String>,

    #[serde(alias = "homepage")]
    pub homepages: Vec<String>,

    #[serde(alias = "license")]
    pub licenses: Option<Vec<String>>,

    #[serde(alias = "maintainer")]
    pub maintainers: Vec<String>,

    #[serde(alias = "note")]
    pub notes: Option<Vec<String>>,

    #[serde(alias = "tag")]
    pub tags: Option<Vec<String>>,

    pub bsum: String,
    pub shasum: String,

    #[serde(default, deserialize_with = "empty_is_none")]
    pub build_id: Option<String>,

    #[serde(default, deserialize_with = "empty_is_none")]
    pub build_date: Option<String>,

    #[serde(default, deserialize_with = "empty_is_none", alias = "build_gha")]
    pub build_action: Option<String>,

    #[serde(default, deserialize_with = "empty_is_none")]
    pub build_script: Option<String>,

    #[serde(default, deserialize_with = "empty_is_none")]
    pub build_log: Option<String>,

    #[serde(alias = "category")]
    pub categories: Vec<String>,

    pub provides: Option<Vec<String>>,

    #[serde(default, deserialize_with = "empty_is_none")]
    pub icon: Option<String>,

    #[serde(default, deserialize_with = "empty_is_none")]
    pub desktop: Option<String>,

    #[serde(default, deserialize_with = "empty_is_none")]
    pub appstream: Option<String>,

    #[serde(default, deserialize_with = "empty_is_none")]
    pub app_id: Option<String>,

    #[serde(default, deserialize_with = "optional_number")]
    pub download_count: Option<u64>,

    #[serde(default, deserialize_with = "optional_number")]
    pub download_count_month: Option<u64>,

    #[serde(default, deserialize_with = "optional_number")]
    pub download_count_week: Option<u64>,

    pub repology: Option<Vec<String>>,
    pub snapshots: Option<Vec<String>>,
}
