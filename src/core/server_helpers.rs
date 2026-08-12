use std::future::Future;

use actix_web::web::Data;
use diesel::serialize::Output;
use leptos::prelude::ServerFnError;

use crate::entrypoint::application_state::AppState;

pub async fn with_extractor_and_service<T, F, FS, R, Fut>(
    mut service_extractor: FS,
    mut f: F,
) -> Result<T, ServerFnError>
where
    Fut: Future<Output = Result<T, anyhow::Error>>,
    F: FnOnce(R) -> Fut + Send + 'static,
    FS: Fn(Data<AppState>) -> R + Send + 'static,
    T: Send + 'static,
    R: Send + Clone + 'static,
{
    use crate::entrypoint::application_state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;

    let service = service_extractor(state);
    f(service).await.map_err(ServerFnError::new)
}
