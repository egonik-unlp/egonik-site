use crate::personal_information::dto::{ContactInformationDto, PersonalInformationDto};
use leptos::prelude::*;

#[server]
pub async fn get_personal_info() -> Result<PersonalInformationDto, ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    use crate::personal_information::{
        model::PersonalInformation, service::PersonalInformationService,
    };

    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let mut service: PersonalInformationService = state.personal_information_service.clone();
    let result = service.get_personal_information().await.unwrap();
    Ok(result)
}

#[server]
pub async fn get_full_personal_info(
) -> Result<(PersonalInformationDto, ContactInformationDto), ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    use crate::personal_information::{
        model::PersonalInformation, service::PersonalInformationService,
    };

    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let mut service: PersonalInformationService = state.personal_information_service.clone();
    service
        .get_full_personal_information()
        .await
        .map_err(ServerFnError::new)
}
