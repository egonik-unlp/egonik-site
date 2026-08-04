use leptos::prelude::*;

use crate::{
    personal_information::dto::PersonalInformationDto, publications::dto::PublicationItemDto,
};

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
