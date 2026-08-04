use anyhow::Context;
use diesel::{
    query_dsl::methods::{FilterDsl, SelectDsl},
    ExpressionMethods, RunQueryDsl, SelectableHelper,
};

use crate::{core::Repository, database::connection::DbPool, publications::model::PublicationItem};

#[derive(Debug)]
pub struct PublicationsRepository {
    pool: DbPool,
}

impl PublicationsRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl Repository<PublicationItem> for PublicationsRepository {
    fn get_all(&mut self) -> anyhow::Result<Vec<PublicationItem>> {
        let mut conn = self
            .pool
            .get()
            .context("Couldn't acquire connection from pool")?;
        crate::schema::publication_items::table
            .select(PublicationItem::as_select())
            .get_results(&mut conn)
            .context("Can't get publication items from db")
    }
    fn get_by_id(&mut self, id: i32) -> anyhow::Result<PublicationItem> {
        use crate::schema::publication_items;
        let mut conn = self
            .pool
            .get()
            .context("Couldn't acquire connection from pool")?;
        publication_items::table
            .filter(publication_items::id.eq(id))
            .select(PublicationItem::as_select())
            .get_result(&mut conn)
            .context("Can't get publication item from db")
    }
}
