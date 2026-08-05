use diesel::{
    prelude::{AsChangeset, Associations, Identifiable, Insertable, Queryable},
    BelongingToDsl, Selectable,
};
use serde::{Deserialize, Serialize};

use crate::portfolio::dto::{PortfolioItemDto, TagDto};

#[derive(
    Debug, Clone, Serialize, Deserialize, Selectable, Queryable, Associations, Identifiable,
)]
#[diesel(table_name= crate::schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(PortfolioItem, foreign_key=portfolio_item_id))]
pub struct Tag {
    pub id: i32,
    pub portfolio_item_id: i32,
    pub value: String,
}
impl Into<TagDto> for Tag {
    fn into(self) -> TagDto {
        let Tag {
            id,
            portfolio_item_id,
            value,
        } = self;
        TagDto { id, value }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name= crate::schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct TagRow {
    pub portfolio_item_id: Option<i32>,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Selectable, Queryable, Identifiable)]
#[diesel(table_name= crate::schema::portfolio_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PortfolioItem {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub public: bool,
    pub public_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Insertable, AsChangeset)]
#[diesel(table_name= crate::schema::portfolio_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PortfolioItemRow {
    pub title: String,
    pub description: String,
    pub public: bool,
    pub public_url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PortfolioItemWithTags {
    pub portfolio_item: PortfolioItem,
    pub tags: Vec<Tag>,
}

impl PortfolioItemWithTags {
    pub fn new(portfolio_item: PortfolioItem, tags: Vec<Tag>) -> Self {
        Self {
            portfolio_item,
            tags,
        }
    }
}

impl Into<PortfolioItemDto> for PortfolioItemWithTags {
    fn into(self) -> PortfolioItemDto {
        let PortfolioItem {
            id,
            title,
            description,
            public,
            public_url,
        } = self.portfolio_item;
        let tags = self.tags.into_iter().map(Into::into).collect();
        PortfolioItemDto {
            title,
            description,
            public,
            public_url,
            tags,
        }
    }
}
