#![allow(clippy::from_over_into)]

use std::{path::PathBuf, str::FromStr};

use anyhow::Context;
use egonik_site::{
    database::connection::get_connection_pool,
    personal_information::{
        repository::PersonalInformationRepository, service::PersonalInformationService,
    },
    publications::{repository::PublicationsRepository, service::PublicationsService},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = get_connection_pool();
    let pub_repo = PublicationsRepository::new(pool.clone());
    let pers_repo = PersonalInformationRepository::new(pool);
    let pub_service = PublicationsService::new(pub_repo);
    let pers_service = PersonalInformationService::new(pers_repo);
    let config_path = PathBuf::from_str("config.toml").context("Problems instantiating path")?;
    pub_service.sync_publication_history().await;
    pers_service
        .load_config_from_toml(config_path)
        .await
        .context("Can't load data from toml into appliation")?;
    Ok(())
}
