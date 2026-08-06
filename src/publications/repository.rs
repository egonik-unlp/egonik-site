use crate::{
    database::connection::{interact, DbPool},
    publications::{
        dto::PublicationItemDto,
        model::{PublicationItem, PublicationItemRow},
    },
};
use anyhow::Context;
use diesel::prelude::*;
use diesel::{dsl::insert_into, ExpressionMethods, RunQueryDsl, SelectableHelper};

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

    pub async fn create_article(&self, new_article: PublicationItemDto) -> anyhow::Result<()> {
        use crate::schema::publication_items;
        use publication_items::dsl::*;

        interact(&self.pool, |conn| {
            conn.transaction(|c| {
                insert_into(publication_items)
                    .values(&PublicationItemRow::from(new_article))
                    .on_conflict_do_nothing()
                    .execute(c)
                    .context("Error creating article")?;
                Ok(())
            })
        })
        .await
    }
}
