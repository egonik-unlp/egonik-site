use chrono::NaiveDate;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Associations)]
#[diesel(table_name = crate::schema::job_institutions )]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(belongs_to(JobExperienceItem, foreign_key = job_experience_id))]
pub struct JobInstitution {
    id: i32,
    job_experience_id: i32,
    name: String,
    url: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = crate::schema::job_experiences )]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct JobExperienceItem {
    id: i32,
    date_from: NaiveDate,
    date_to: Option<NaiveDate>,
    job_title: String,
    accomplishments: String,
    responsabilities: String,
}
