#![allow(clippy::from_over_into)]

use std::{path::PathBuf, str::FromStr};

use egonik_site::{
    database::connection::get_connection_pool,
    personal_information::{
        repository::PersonalInformationRepository, service::PersonalInformationService,
    },
    publications::{repository::PublicationsRepository, service::PublicationsService},
};

#[tokio::main]
async fn main() {
    let pool = get_connection_pool();
    let pub_repo = PublicationsRepository::new(pool.clone());
    let pers_repo = PersonalInformationRepository::new(pool);
    let mut pub_service = PublicationsService::new(pub_repo);
    let pers_service = PersonalInformationService::new(pers_repo);
    pub_service.sync_publication_history().await;
    pers_service
        .load_config_from_toml(PathBuf::from_str("config.toml").unwrap())
        .unwrap();
}
