use leptos::prelude::*;

#[server]
pub async fn get_all_publications() -> Result<Vec<String>, ServerFnError> {
    use crate::core::Repository;
    use crate::entrypoint::application_state::AppState;
    use actix_web::web::Data;
    use leptos_actix::extract;
    let state: Data<AppState> = extract().await?;
    let mut repo = state.publications_repository.clone();
    let results = repo.get_all().unwrap();

    Ok(results.iter().map(|r| r.title.clone()).collect())
}
