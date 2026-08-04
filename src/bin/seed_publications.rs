#![allow(clippy::from_over_into)]

use egonik_site::{
    database::connection::get_connection_pool,
    publications::{repository::PublicationsRepository, service::PublicationsService},
};

#[tokio::main]
async fn main() {
    let repo = PublicationsRepository::new(get_connection_pool());
    let mut service = PublicationsService::new(repo);
    service.sync_publication_history().await;
}
