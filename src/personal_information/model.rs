use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(table_name = crate::schema::personal_informations)]
pub struct PersonalInformation {
    id: i32,
    name: String,
    surname: String,
    image_url: String,
    birth_date: NaiveDate,
}

#[derive(Debug, Deserialize, Serialize, Associations, Selectable, Queryable)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(PersonalInformation,))]
#[diesel(table_name = crate::schema::contact_informations)]
pub struct ContactInformation {
    id: i32,
    personal_information_id: i32,
    github: String,
    email: String,
    instagram: String,
    linked_in: String,
}
