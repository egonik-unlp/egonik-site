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
