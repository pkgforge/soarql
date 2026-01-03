use serde::{Deserialize, Serialize};

use crate::deserializers::{empty_is_none, flexible_bool, normalize_resource, optional_u64};

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

#[derive(Debug, Default, Clone, Deserialize, Serialize)]
pub struct RemotePackage {
    #[serde(default, deserialize_with = "optional_u64")]
    pub rank: Option<u64>,

    pub pkg_id: String,
    pub pkg_name: String,

    #[serde(default, deserialize_with = "empty_is_none")]
    pub pkg_family: Option<String>,

    #[serde(default, deserialize_with = "empty_is_none")]
    pub pkg_type: Option<String>,

    #[serde(default, deserialize_with = "empty_is_none")]
    pub pkg_webpage: Option<String>,

    pub description: String,
    pub version: String,

    pub download_url: String,

    #[serde(default, deserialize_with = "optional_u64")]
    pub size_raw: Option<u64>,

    #[serde(default, deserialize_with = "empty_is_none")]
    pub ghcr_pkg: Option<String>,

    #[serde(default, deserialize_with = "optional_u64")]
    pub ghcr_size_raw: Option<u64>,

    pub ghcr_files: Option<Vec<String>>,

    #[serde(default, deserialize_with = "empty_is_none")]
    pub ghcr_blob: Option<String>,

    #[serde(default, deserialize_with = "empty_is_none")]
    pub ghcr_url: Option<String>,

    #[serde(alias = "src_url")]
    pub src_urls: Option<Vec<String>>,

    #[serde(alias = "homepage")]
    pub homepages: Option<Vec<String>>,

    #[serde(alias = "license")]
    pub licenses: Option<Vec<String>>,

    #[serde(alias = "maintainer")]
    pub maintainers: Option<Vec<String>>,

    #[serde(alias = "note")]
    pub notes: Option<Vec<String>>,

    #[serde(alias = "tag")]
    pub tags: Option<Vec<String>>,

    #[serde(default, deserialize_with = "empty_is_none")]
    pub bsum: Option<String>,

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
    pub categories: Option<Vec<String>>,

    pub provides: Option<Vec<String>>,

    #[serde(default, deserialize_with = "empty_is_none")]
    pub icon: Option<String>,

    #[serde(default, deserialize_with = "normalize_resource")]
    pub desktop: Option<String>,

    #[serde(default, deserialize_with = "empty_is_none")]
    pub appstream: Option<String>,

    #[serde(default, deserialize_with = "empty_is_none")]
    pub app_id: Option<String>,

    #[serde(default, deserialize_with = "flexible_bool")]
    pub soar_syms: Option<bool>,

    #[serde(default, deserialize_with = "flexible_bool")]
    pub deprecated: Option<bool>,

    #[serde(default, deserialize_with = "flexible_bool")]
    pub desktop_integration: Option<bool>,

    #[serde(default, deserialize_with = "flexible_bool")]
    pub portable: Option<bool>,

    #[serde(default, deserialize_with = "flexible_bool")]
    pub recurse_provides: Option<bool>,

    pub repology: Option<Vec<String>>,
    pub snapshots: Option<Vec<String>>,
    pub replaces: Option<Vec<String>>,
}
