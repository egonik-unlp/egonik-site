use std::future::Future;

use actix_web::web::Data;
use diesel::serialize::Output;
use leptos::prelude::ServerFnError;

use crate::entrypoint::application_state::AppState;

pub async fn with_extractor_and_service<FS, F, Fut, R, T>(
    mut service_extractor: FS,
    mut f: F,
) -> Result<T, ServerFnError>
where
    F: FnOnce(R) -> Fut,
    FS: Fn(Data<AppState>) -> R,
    T:,
    Fut: Future<Output = Result<T, anyhow::Error>>,
    R: Clone,
{
    use crate::entrypoint::application_state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;

    let service = service_extractor(state);
    f(service).await.map_err(ServerFnError::new)
}
