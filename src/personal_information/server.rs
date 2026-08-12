use crate::personal_information::dto::{ContactInformationDto, PersonalInformationDto};
use leptos::prelude::*;

#[server]
pub async fn get_personal_info() -> Result<PersonalInformationDto, ServerFnError> {
    use crate::core::server_helpers::with_extractor_and_service;
    with_extractor_and_service(
        |app_state| app_state.personal_information_service.clone(),
        async move |service| service.get_personal_information().await,
    )
    .await
}

#[server]
pub async fn get_full_personal_info(
) -> Result<(PersonalInformationDto, ContactInformationDto), ServerFnError> {
    use crate::core::server_helpers::with_extractor_and_service;
    with_extractor_and_service(
        |app_state| app_state.personal_information_service.clone(),
        async move |service| service.get_full_personal_information().await,
    )
    .await
}
