use anyhow::Context;
use diesel::{
    query_dsl::methods::{FilterDsl, SelectDsl},
    ExpressionMethods, RunQueryDsl, SelectableHelper,
};

use crate::{
    core::Repository,
    database::connection::DbPool,
    portfolio::{dto::PortfolioItemDto, model::PortfolioItem},
};

#[derive(Debug, Clone)]
pub struct PortfolioItemsRepository {
    pool: DbPool,
}

impl PortfolioItemsRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl Repository<PortfolioItem, PortfolioItemDto> for PortfolioItemsRepository {
    fn get_all(&mut self) -> anyhow::Result<Vec<PortfolioItem>> {
        use crate::schema::portfolio_items;
        let mut conn = self
            .pool
            .get()
            .context("Couldn't acquire connection from pool")?;
        portfolio_items::table
            .select(PortfolioItem::as_select())
            .get_results(&mut conn)
            .context("Can't get publication items from db")
    }
    fn get_by_id(&mut self, id: i32) -> anyhow::Result<PortfolioItem> {
        use crate::schema::portfolio_items;
        let mut conn = self
            .pool
            .get()
            .context("Couldn't acquire connection from pool")?;
        portfolio_items::table
            .filter(portfolio_items::id.eq(id))
            .select(PortfolioItem::as_select())
            .get_result(&mut conn)
            .context("Can't get publication item from db")
    }
    fn create_article(&mut self, article: PortfolioItemDto) -> anyhow::Result<()> {
        todo!()
    }
}
