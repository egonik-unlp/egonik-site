use anyhow::Context;
use diesel::{
    delete, insert_into,
    query_dsl::methods::{FilterDsl, SelectDsl},
    upsert::excluded,
    BelongingToDsl, Connection, ExpressionMethods, GroupedBy, RunQueryDsl, SelectableHelper,
};

use crate::{
    database::connection::{interact, DbPool},
    portfolio::{
        dto::PortfolioItemDto,
        model::{PortfolioItem, PortfolioItemRow, PortfolioItemWithTags, Tag, TagRow},
    },
};

#[derive(Debug, Clone)]
pub struct PortfolioItemsRepository {
    pool: DbPool,
}

pub type PortfolioTuple = (PortfolioItemRow, Vec<TagRow>);

impl PortfolioItemsRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl PortfolioItemsRepository {
    pub async fn get_all(mut self) -> anyhow::Result<Vec<PortfolioItemDto>> {
        use crate::schema::{portfolio_items, tags};
        interact(&self.pool, |conn| {
            conn.transaction(|c| {
                // `public` is the per-row visibility flag; this listing feeds the public
                // site, so unpublished rows must never leave the database.
                let portfolio_items = portfolio_items::table
                    .filter(portfolio_items::public.eq(true))
                    .load::<PortfolioItem>(c)
                    .context("Couldn't fetch from db")?;
                let tags_per_item = Tag::belonging_to(&portfolio_items)
                    .load::<Tag>(c)
                    .context("Couldnt fetch from db")?;
                let piwt = tags_per_item
                    .grouped_by(&portfolio_items)
                    .into_iter()
                    .zip(portfolio_items)
                    .map(|(tags, portfolio_item)| {
                        PortfolioItemWithTags::new(portfolio_item, tags).into()
                    })
                    .collect();
                Ok(piwt)
            })
        })
        .await
        .context("back from transaction")
    }
    pub async fn get_by_id(&mut self, id: i32) -> anyhow::Result<PortfolioItemDto> {
        use crate::schema::portfolio_items;
        interact(&self.pool, move |conn| {
            conn.transaction(|c| {
                let portfolio_item = portfolio_items::table
                    .filter(portfolio_items::id.eq(id))
                    .select(PortfolioItem::as_select())
                    .get_result(c)
                    .context("Can't get publication item from db")?;
                let tags = Tag::belonging_to(&portfolio_item)
                    .load::<Tag>(c)
                    .context("Cant get tags from db")?;
                Ok(PortfolioItemWithTags::new(portfolio_item, tags).into())
            })
        })
        .await
    }
    /// Inserts a repository, or refreshes the row that is already there.
    ///
    /// `portfolio_items.title` is `UNIQUE`, so a plain insert made the sync a
    /// once-only operation: the second run aborted on the first repository that
    /// already existed, which is why the pagination fix never reached the
    /// database. Upserting on `title` makes the whole sync idempotent, and
    /// therefore safe to re-run or schedule.
    pub async fn create_article(mut self, article: PortfolioTuple) -> anyhow::Result<()> {
        use crate::schema::{portfolio_items, tags};
        interact(&self.pool, move |conn| {
            conn.transaction(|c| {
                let portfolio_item = insert_into(portfolio_items::table)
                    .values(&article.0)
                    .on_conflict(portfolio_items::title)
                    .do_update()
                    // `excluded` is the row the insert proposed -- so a renamed
                    // description or a repository flipped to private is picked up
                    // rather than silently kept at its first-sync value.
                    .set((
                        portfolio_items::description.eq(excluded(portfolio_items::description)),
                        portfolio_items::public.eq(excluded(portfolio_items::public)),
                        portfolio_items::public_url.eq(excluded(portfolio_items::public_url)),
                    ))
                    .returning(PortfolioItem::as_returning())
                    .get_result(c)
                    .context("Couldn't load pfitem to db")?;

                // Tags are replaced, not appended: re-running the sync would
                // otherwise duplicate every topic, and a topic removed on GitHub
                // would linger forever.
                delete(tags::table.filter(tags::portfolio_item_id.eq(portfolio_item.id)))
                    .execute(c)
                    .context("Couldn't clear stale tags")?;

                let mut tagf = article.1;
                tagf.iter_mut()
                    .for_each(|row| row.portfolio_item_id = Some(portfolio_item.id));
                // A repository with no GitHub topics yields no tag rows, and Diesel builds
                // a syntactically invalid `VALUES ()` for an empty slice.
                if !tagf.is_empty() {
                    // The `?` matters: this used to end in a `;`, so a failed tag insert was
                    // discarded and the transaction still committed the article without tags.
                    insert_into(tags::table)
                        .values(&tagf)
                        .execute(c)
                        .context("Couldn't load into db t")?;
                }
                Ok(())
            })
        })
        .await
    }
}
