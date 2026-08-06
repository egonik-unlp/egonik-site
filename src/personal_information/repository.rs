use anyhow::{Context, Ok};
use diesel::{
    insert_into,
    query_dsl::methods::{FilterDsl, SelectDsl},
    Connection, ExpressionMethods, RunQueryDsl, SelectableHelper,
};

use crate::{
    database::connection::{interact, DbPool},
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
    pub async fn get(&mut self) -> anyhow::Result<PersonalInformation> {
        use crate::schema::personal_informations;
        interact(&self.pool, |conn| {
            personal_informations::table
                .select(PersonalInformation::as_select())
                .get_result(conn)
                .map_err(Into::into)
        })
        .await
    }

    pub async fn get_full(&mut self) -> anyhow::Result<(PersonalInformation, ContactInformation)> {
        use crate::schema::personal_informations;
        use diesel::BelongingToDsl;
        use leptos::prelude::*;
        interact(&self.pool, |conn| {
            conn.transaction(|c| {
                let personal_information = personal_informations::table
                    .select(PersonalInformation::as_select())
                    .get_result(c)
                    .context("Can't fetch personal infomration from db")?;
                let contact_information = ContactInformation::belonging_to(&personal_information)
                    .get_result::<ContactInformation>(c)
                    .context("Can't get contact information from db")?;

                Ok((personal_information, contact_information))
            })
        })
        .await
    }
    pub async fn get_by_id(&mut self, id: i32) -> anyhow::Result<PersonalInformation> {
        use crate::schema::personal_informations;
        interact(&self.pool, move |conn| {
            personal_informations::table
                .filter(personal_informations::id.eq(id))
                .select(PersonalInformation::as_select())
                .get_result(conn)
                .context("Can't get publication item from db")
        })
        .await
    }
    pub async fn create_article(
        &mut self,
        personal_information_row: PersonalInformationRow,
        contact_information_row: ContactInformationRow,
    ) -> anyhow::Result<()> {
        use crate::schema::contact_informations;
        use crate::schema::personal_informations;
        use contact_informations::dsl::*;
        use personal_informations::dsl::*;
        interact(&self.pool, |conn| {
            conn.transaction(|c| {
                let personal_information = insert_into(personal_informations)
                    .values(personal_information_row.clone())
                    .returning(PersonalInformation::as_returning())
                    .on_conflict(personal_informations::id)
                    .do_update()
                    .set(personal_information_row)
                    .get_result(c)
                    .context("Can't create personal information")?;

                let contact_information = ContactInformationRow {
                    personal_information_id: personal_information.id,
                    ..contact_information_row
                };
                insert_into(contact_informations)
                    .values(contact_information.clone())
                    .returning(ContactInformation::as_returning())
                    .get_result(c)
                    .with_context(|| {
                        format!(
                            "Can't create contact information for: {:?}",
                            contact_information
                        )
                    });
                Ok(())
            })
        })
        .await
    }
}
