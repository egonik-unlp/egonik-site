use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JobExperienceItemDto {
    pub date_from: NaiveDate,
    pub date_to: Option<NaiveDate>,
    pub job_title: String,
    pub accomplishments: String,
    pub responsabilities: String,
}
