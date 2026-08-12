use leptos::prelude::*;

use crate::{
    core::with_extractor_and_service,
    portfolio::dto::{PortfolioItemDto, PortfolioItemWithMetadataDto},
};

#[server]
pub async fn get_all_portfolio_items() -> Result<Vec<PortfolioItemDto>, ServerFnError> {
    use crate::core::with_extractor_and_service;
    with_extractor_and_service(
        |app_state| app_state.portfolio_service.clone(),
        async move |service| service.get_all().await,
    )
    .await

    // service.get_all().await.map_err(ServerFnError::new)
}

#[server]
pub async fn get_all_portfolio_items_with_metadata(
) -> Result<Vec<PortfolioItemWithMetadataDto>, ServerFnError> {
    use crate::core::with_extractor_and_service;
    with_extractor_and_service(
        |app_state| app_state.portfolio_service.clone(),
        async move |service| service.get_all_with_metadata().await,
    )
    .await
}
