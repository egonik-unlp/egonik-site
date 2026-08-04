use diesel::{
    prelude::{Associations, Queryable},
    Selectable,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Selectable, Queryable, Associations)]
#[diesel(table_name= crate::schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(PortfolioItem))]
pub struct Tag {
    pub id: i32,
    pub portfolio_item_id: i32,
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Selectable, Queryable)]
#[diesel(table_name= crate::schema::portfolio_items)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PortfolioItem {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub public: bool,
    pub public_url: Option<String>,
}
