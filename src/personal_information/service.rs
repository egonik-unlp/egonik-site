use std::{fs::OpenOptions, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    database::connection::DbPool,
    personal_information::{
        model::{ContactInformationRow, InformationConfigFile, PersonalInformationRow},
        repository::PersonalInformationRepository,
    },
};

#[derive(Debug, Clone)]
pub struct PersonalInformationService {
    repo: PersonalInformationRepository,
}

impl PersonalInformationService {
    pub fn new(repo: PersonalInformationRepository) -> Self {
        Self { repo }
    }

    pub fn load_config_from_toml(mut self, path: PathBuf) -> anyhow::Result<()> {
        let file_string = std::fs::read_to_string(path)?;
        let InformationConfigFile {
            personal_information_row,
            contact_information_row,
        } = toml::from_str(&file_string)?;
        self.repo
            .create_article(personal_information_row, contact_information_row)?;
        Ok(())
    }
}
