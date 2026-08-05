use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PersonalInformationDto {
    pub id: i32,
    pub name: String,
    pub surname: String,
    pub image_url: String,
    pub birth_date: String,
    // pub birth_date:DefaultHeaders ,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInformationDto {
    pub github: String,
    pub email: String,
    pub instagram: String,
    pub linked_in: String,
}
