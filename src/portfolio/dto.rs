use serde::{Deserialize, Serialize};

use crate::{portfolio::metadata::ProjectMetadataDto, publications::dto::PublicationItemDto};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDto {
    pub id: i32,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioItemDto {
    pub title: String,
    pub description: String,
    pub public: bool,
    pub public_url: Option<String>,
    pub tags: Vec<TagDto>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortfolioItemWithMetadataDto {
    pub portfolio_item: PortfolioItemDto,
    pub metadata: ProjectMetadataDto,
}

impl PortfolioItemWithMetadataDto {
    pub fn new(portfolio_item: PortfolioItemDto, metadata: ProjectMetadataDto) -> Self {
        Self {
            portfolio_item,
            metadata,
        }
    }
}
