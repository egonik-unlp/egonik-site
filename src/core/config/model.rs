use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::personal_information::model::{ContactInformationRow, PersonalInformationRow};

pub struct Config {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub image_url: String,
    pub birth_date: NaiveDate,
    pub personal_information_id: i32,
    pub github: String,
    pub email: String,
    pub instagram: String,
    pub linked_in: String,
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
