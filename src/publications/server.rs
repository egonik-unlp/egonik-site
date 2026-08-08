use leptos::prelude::*;

use crate::publications::dto::{PublicationItemDto, PublicationItemWithMetadataDto};

#[server]
pub async fn get_all_publications() -> Result<Vec<PublicationItemDto>, ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let mut service = state.publications_service.clone();
    let results = service.get_all_publications().await.unwrap();
    Ok(results)
}

#[server]
pub async fn get_all_publications_mapped(
) -> Result<Vec<PublicationItemWithMetadataDto>, ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let mut service = state.publications_service.clone();
    service
        .get_publications_with_metadata()
        .await
        .map_err(ServerFnError::new)
}
