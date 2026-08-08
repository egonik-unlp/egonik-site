use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RootStructWorksItemIdentifiers {
    pub doi: Option<String>,
    #[serde(rename = "openAlex", skip_serializing_if = "Option::is_none")]
    pub open_alex: Option<String>,
    pub pmcid: Option<String>,
    pub pmid: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RootStructWorksItemUrls {
    #[serde(rename = "openAccess", skip_serializing_if = "Option::is_none")]
    pub open_access: Option<String>,
    pub primary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RootStructAuthor {
    pub affiliations: Option<Vec<String>>,
    #[serde(rename = "googleScholarId", skip_serializing_if = "Option::is_none")]
    pub google_scholar_id: Option<String>,
    #[serde(
        rename = "googleScholarProfile",
        skip_serializing_if = "Option::is_none"
    )]
    pub google_scholar_profile: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "openAlexId", skip_serializing_if = "Option::is_none")]
    pub open_alex_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PublicationMetadataDto {
    #[serde(rename = "articleNumber", skip_serializing_if = "Option::is_none")]
    pub article_number: Option<String>,
    pub authors: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub citations: Option<RootStructWorksItemCitations>,
    pub description: Option<String>,
    pub display: Option<RootStructWorksItemDisplay>,
    pub domains: Option<Vec<String>>,
    pub featured: Option<bool>,
    pub id: Option<String>,
    pub identifiers: Option<RootStructWorksItemIdentifiers>,
    pub issue: Option<String>,
    pub keywords: Option<Vec<String>>,
    pub language: Option<String>,
    pub methods: Option<Vec<String>>,
    pub pages: Option<String>,
    #[serde(rename = "presentationDate", skip_serializing_if = "Option::is_none")]
    pub presentation_date: Option<String>,
    #[serde(rename = "publicationDate", skip_serializing_if = "Option::is_none")]
    pub publication_date: Option<String>,
    pub publisher: Option<String>,
    pub role: Option<String>,
    pub title: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    pub urls: Option<RootStructWorksItemUrls>,
    pub venue: Option<String>,
    pub volume: Option<String>,
    pub year: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MetadataTableDto {
    pub author: RootStructAuthor,
    #[serde(rename = "retrievedAt")]
    pub retrieved_at: String,
    #[serde(rename = "schemaVersion")]
    pub schema_version: i64,
    pub works: Vec<PublicationMetadataDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RootStructWorksItemDisplay {
    pub priority: Option<i64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RootStructWorksItemCitations {
    #[serde(rename = "asOf", skip_serializing_if = "Option::is_none")]
    pub as_of: Option<String>,
    pub count: Option<i64>,
    pub source: Option<String>,
}
