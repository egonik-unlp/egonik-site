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
impl PortfolioItemDto {
    pub fn is_its_metadata(&self, other: &ProjectMetadataDto) -> bool {
        other
            .clone()
            .title
            .is_some_and(|other_title| self.title.eq(&other_title))
    }
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
