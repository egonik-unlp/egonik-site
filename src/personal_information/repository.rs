use anyhow::Context;
use diesel::{
    query_dsl::methods::{FilterDsl, SelectDsl},
    ExpressionMethods, RunQueryDsl, SelectableHelper,
};

use crate::{
    core::Repository,
    database::connection::DbPool,
    personal_information::{dto::PersonalInformationDto, model::PersonalInformation},
};

#[derive(Debug, Clone)]
pub struct PersonalInformationRepository {
    pool: DbPool,
}

impl PersonalInformationRepository {
    pub fn new(pool: DbPool) -> Self {
        Self { pool }
    }
}

impl Repository<PersonalInformation, PersonalInformationDto> for PersonalInformationRepository {
    fn get_all(&mut self) -> anyhow::Result<Vec<PersonalInformation>> {
        use crate::schema::personal_informations;
        let mut conn = self
            .pool
            .get()
            .context("Couldn't acquire connection from pool")?;
        personal_informations::table
            .select(PersonalInformation::as_select())
            .get_results(&mut conn)
            .context("Can't get publication items from db")
    }
    fn get_by_id(&mut self, id: i32) -> anyhow::Result<PersonalInformation> {
        use crate::schema::personal_informations;
        let mut conn = self
            .pool
            .get()
            .context("Couldn't acquire connection from pool")?;
        personal_informations::table
            .filter(personal_informations::id.eq(id))
            .select(PersonalInformation::as_select())
            .get_result(&mut conn)
            .context("Can't get publication item from db")
    }
    fn create_article(&mut self, article: PersonalInformationDto) -> anyhow::Result<()> {
        todo!()
    }
}
