use crate::job_experience::dto::JobExperienceItemDto;
use crate::{
    core::Repository, database::connection::DbPool, job_experience::model::JobExperienceItem,
};
use anyhow::Context;
use diesel::{
    query_dsl::methods::{FilterDsl, SelectDsl},
    ExpressionMethods, RunQueryDsl, SelectableHelper,
};

#[derive(Debug, Clone)]
pub struct JobExperienceItemRepository {
    pool: DbPool,
}

impl JobExperienceItemRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl Repository<JobExperienceItem, JobExperienceItemDto> for JobExperienceItemRepository {
    fn get_all(&mut self) -> anyhow::Result<Vec<JobExperienceItem>> {
        use crate::schema::job_experiences;
        let mut conn = self
            .pool
            .get()
            .context("Couldn't acquire connection from pool")?;
        job_experiences::table
            .select(JobExperienceItem::as_select())
            .get_results(&mut conn)
            .context("Can't get publication items from db")
    }
    fn get_by_id(&mut self, id: i32) -> anyhow::Result<JobExperienceItem> {
        use crate::schema::job_experiences;
        let mut conn = self
            .pool
            .get()
            .context("Couldn't acquire connection from pool")?;
        job_experiences::table
            .filter(job_experiences::id.eq(id))
            .select(JobExperienceItem::as_select())
            .get_result(&mut conn)
            .context("Can't get publication item from db")
    }
    fn create_article(&mut self, article: JobExperienceItemDto) -> anyhow::Result<()> {
        todo!()
    }
}
