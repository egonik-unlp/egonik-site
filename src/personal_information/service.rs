use std::{fs::OpenOptions, path::PathBuf};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::{
    database::connection::DbPool,
    personal_information::{
        dto::{ContactInformationDto, PersonalInformationDto},
        model::{
            ContactInformationRow, InformationConfigFile, PersonalInformation,
            PersonalInformationRow,
        },
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

    pub async fn get_personal_information(mut self) -> anyhow::Result<PersonalInformationDto> {
        self.repo.get().await.map(Into::into)
    }

    pub async fn get_full_personal_information(
        mut self,
    ) -> anyhow::Result<(PersonalInformationDto, ContactInformationDto)> {
        let (personal_information, contact_information) = self
            .repo
            .get_full()
            .await
            .context("Can't fetch personal information")?;
        Ok((personal_information.into(), contact_information.into()))
    }

    pub async fn load_config_from_toml(mut self, path: PathBuf) -> anyhow::Result<()> {
        let file_string = std::fs::read_to_string(path)?;
        let InformationConfigFile {
            personal_information_row,
            contact_information_row,
        } = toml::from_str(&file_string)?;
        self.repo
            .create_article(personal_information_row, contact_information_row)
            .await
            .context("Can't create article in db")?;
        Ok(())
    }
}
