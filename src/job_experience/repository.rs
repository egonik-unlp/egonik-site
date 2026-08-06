use crate::{
    database::connection::{interact, DbPool},
    job_experience::{
        dto::JobExperienceItemDto,
        model::{JobExperienceItem, JobExperienceItemRow},
    },
};
use anyhow::Context;
use diesel::{
    dsl::insert_into,
    query_dsl::methods::{FilterDsl, SelectDsl},
    Connection, ExpressionMethods, RunQueryDsl, SelectableHelper,
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

impl JobExperienceItemRepository {
    pub async fn get_all(mut self) -> anyhow::Result<Vec<JobExperienceItem>> {
        use crate::schema::job_experiences;
        interact(&self.pool, |conn| {
            conn.transaction(|c| {
                job_experiences::table
                    .select(JobExperienceItem::as_select())
                    .get_results(c)
                    .context("Can't get publication items from db")
            })
        })
        .await
    }
    pub async fn get_by_id(mut self, id: i32) -> anyhow::Result<JobExperienceItem> {
        use crate::schema::job_experiences;
        interact(&self.pool, move |conn| {
            conn.transaction(|c| {
                job_experiences::table
                    .filter(job_experiences::id.eq(id))
                    .select(JobExperienceItem::as_select())
                    .get_result(c)
                    .context("Can't get publication item from db")
            })
        })
        .await
    }
    pub async fn create_article(mut self, article: JobExperienceItemDto) -> anyhow::Result<()> {
        use crate::schema::job_experiences;
        interact(&self.pool, |conn| {
            let job_experience_item: JobExperienceItemRow = article.into();
            conn.transaction(|c| {
                insert_into(job_experiences::table)
                    .values(&job_experience_item)
                    .load::<JobExperienceItem>(c)
                    .context("Inserting job exp");
                Ok(())
            })
        })
        .await
    }
}
