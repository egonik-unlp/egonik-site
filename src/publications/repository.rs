use anyhow::Context;
use diesel::{
    dsl::insert_into,
    query_dsl::methods::{FilterDsl, SelectDsl},
    ExpressionMethods, RunQueryDsl, SelectableHelper,
};

use crate::{
    database::connection::DbPool,
    publications::{
        dto::PublicationItemDto,
        model::{PublicationItem, PublicationItemRow},
    },
};

#[derive(Debug, Clone)]
pub struct PublicationsRepository {
    pool: DbPool,
}

impl PublicationsRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl PublicationsRepository {
    pub fn get_all(&mut self) -> anyhow::Result<Vec<PublicationItem>> {
        let mut conn = self
            .pool
            .get()
            .context("Couldn't acquire connection from pool")?;
        crate::schema::publication_items::table
            .select(PublicationItem::as_select())
            .get_results(&mut conn)
            .context("Can't get publication items from db")
    }
    pub fn get_by_id(&mut self, id: i32) -> anyhow::Result<PublicationItem> {
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

    pub fn create_article(&mut self, new_article: PublicationItemDto) -> anyhow::Result<()> {
        use crate::schema::publication_items;
        use publication_items::dsl::*;
        let mut conn = self.pool.get().context("Couldn't get conn from pool")?;
        let value: PublicationItemRow = new_article.into();
        insert_into(publication_items)
            .values(&value)
            .on_conflict_do_nothing()
            .execute(&mut conn)
            .context("Error creating article")?;
        Ok(())
    }
}
