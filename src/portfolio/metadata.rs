use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProjectMetadataDto {
    pub categories: Option<Vec<String>>,
    pub description: Option<String>,
    pub display: Option<ProjectDisplayDto>,
    pub domains: Option<Vec<String>>,
    pub featured: Option<bool>,
    pub highlights: Option<Vec<String>>,
    pub languages: Option<Vec<String>>,
    pub maturity: Option<String>,
    #[serde(rename = "projectTypes", skip_serializing_if = "Option::is_none")]
    pub project_types: Option<Vec<String>>,
    pub role: Option<String>,
    pub status: Option<String>,
    pub title: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectMetadataTableDto {
    pub owner: String,
    pub repositories: HashMap<String, ProjectMetadataDto>,
    #[serde(rename = "schemaVersion")]
    pub schema_version: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProjectDisplayDto {
    pub priority: Option<i64>,
}
