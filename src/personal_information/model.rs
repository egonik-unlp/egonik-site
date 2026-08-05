use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::personal_informations)]
pub struct PersonalInformation {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub image_url: String,
    pub birth_date: NaiveDate,
}

#[derive(Debug, Deserialize, Serialize, Associations, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(PersonalInformation,))]
#[diesel(table_name = crate::schema::contact_informations)]
pub struct ContactInformation {
    pub id: i32,
    pub personal_information_id: i32,
    pub github: String,
    pub email: String,
    pub instagram: String,
    pub linked_in: String,
}

#[derive(Debug, Deserialize, Serialize, Insertable)]
#[diesel(table_name = crate::schema::personal_informations)]
pub struct PersonalInformationRow {
    id: i32,
    name: String,
    surname: String,
    image_url: String,
    birth_date: NaiveDate,
}

impl PersonalInformationRow {
    pub fn new(
        id: i32,
        name: String,
        surname: String,
        image_url: String,
        birth_date: NaiveDate,
    ) -> Self {
        Self {
            id,
            name,
            surname,
            image_url,
            birth_date,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Insertable)]
#[diesel(table_name = crate::schema::contact_informations)]
pub struct ContactInformationRow {
    pub personal_information_id: i32,
    pub github: String,
    pub email: String,
    pub instagram: String,
    pub linked_in: String,
}

impl ContactInformationRow {
    pub fn new(
        personal_information_id: i32,
        github: String,
        email: String,
        instagram: String,
        linked_in: String,
    ) -> Self {
        Self {
            personal_information_id,
            github,
            email,
            instagram,
            linked_in,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InformationConfigFile {
    pub personal_information_row: PersonalInformationRow,
    pub contact_information_row: ContactInformationRow,
}

impl InformationConfigFile {
    pub fn new(
        personal_information_row: PersonalInformationRow,
        contact_information_row: ContactInformationRow,
    ) -> Self {
        Self {
            personal_information_row,
            contact_information_row,
        }
    }
}
