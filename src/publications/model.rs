use crate::publications::dto::PublicationItemDto;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::publication_items )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct PublicationItem {
    pub id: i32,
    pub title: String,
    pub abs: String,
    pub year: i32,
    pub journal: String,
    pub link: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = crate::schema::publication_items )]
pub struct PublicationItemRow {
    pub title: String,
    pub abs: String,
    pub year: i32,
    pub journal: String,
    pub link: String,
}

impl From<PublicationItemDto> for PublicationItemRow {
    fn from(value: PublicationItemDto) -> Self {
        let PublicationItemDto {
            title,
            year,
            abs,
            journal,
            link,
        } = value;
        Self {
            title,
            abs,
            year,
            journal,
            link,
        }
    }
}
