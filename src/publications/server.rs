use leptos::prelude::*;

use crate::publications::dto::{PublicationItemDto, PublicationItemWithMetadataDto};

#[server]
pub async fn get_all_publications() -> Result<Vec<PublicationItemDto>, ServerFnError> {
    use crate::core::server_helpers::with_extractor_and_service;
    with_extractor_and_service(
        |app_state| app_state.publications_service.clone(),
        async move |service| service.get_all_publications().await,
    )
    .await
}

#[server]
pub async fn get_all_publications_mapped(
) -> Result<Vec<PublicationItemWithMetadataDto>, ServerFnError> {
    use crate::core::server_helpers::with_extractor_and_service;
    with_extractor_and_service(
        |app_data| app_data.publications_service.clone(),
        async move |service| service.get_publications_with_metadata().await,
    )
    .await
}
