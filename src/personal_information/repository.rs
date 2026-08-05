use anyhow::Context;
use diesel::{
    insert_into,
    query_dsl::methods::{FilterDsl, SelectDsl},
    ExpressionMethods, RunQueryDsl, SelectableHelper,
};

use crate::{
    database::connection::DbPool,
    personal_information::{
        dto::PersonalInformationDto,
        model::{
            ContactInformation, ContactInformationRow, PersonalInformation, PersonalInformationRow,
        },
    },
    schema::contact_informations,
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

impl PersonalInformationRepository {
    pub fn get_all(&mut self) -> anyhow::Result<Vec<PersonalInformation>> {
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
    pub fn get_by_id(&mut self, id: i32) -> anyhow::Result<PersonalInformation> {
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
    pub fn create_article(
        &mut self,
        personal_information_row: PersonalInformationRow,
        contact_information_row: ContactInformationRow,
    ) -> anyhow::Result<()> {
        use crate::schema::contact_informations;
        use crate::schema::personal_informations;
        use contact_informations::dsl::*;
        use personal_informations::dsl::*;

        let mut conn = self
            .pool
            .get()
            .context("Couldn't acquire connection from pool")?;
        let personal_information = insert_into(personal_informations)
            .values(personal_information_row)
            .returning(PersonalInformation::as_returning())
            .on_conflict_do_nothing()
            .get_result(&mut conn)?;
        let contact_information = ContactInformationRow {
            personal_information_id: personal_information.id,
            ..contact_information_row
        };
        insert_into(contact_informations)
            .values(contact_informations)
            .execute(&mut conn)?;
        Ok(())
    }
}
