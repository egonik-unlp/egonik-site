use leptos::{attr::Data, prelude::*};

use crate::portfolio::dto::{PortfolioItemDto, PortfolioItemWithMetadataDto};

#[server]
pub async fn get_all_portfolio_items() -> Result<Vec<PortfolioItemDto>, ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let service = state.portfolio_service.clone();
    service.get_all().await.map_err(ServerFnError::new)
}

#[server]
pub async fn get_all_portfolio_items_with_metadata(
) -> Result<Vec<PortfolioItemWithMetadataDto>, ServerFnError> {
    use crate::entrypoint::application_state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let service = state.portfolio_service.clone();
    service
        .get_all_with_metadata()
        .await
        .map_err(ServerFnError::new)
}
