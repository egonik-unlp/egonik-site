#![allow(clippy::from_over_into)]

use std::{path::PathBuf, str::FromStr};

use anyhow::Context;
use egonik_site::{
    database::connection::get_connection_pool, entrypoint::application_state::AppState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let pool = get_connection_pool();
    let config_path = PathBuf::from_str("config.toml").context("Problems instantiating path")?;
    let appstate = AppState::new(pool);
    appstate
        .publications_service
        .sync_publication_history()
        .await;
    appstate
        .personal_information_service
        .load_config_from_toml(config_path)
        .await
        .context("Can't load data from toml into appliation")?;
    appstate
        .portfolio_service
        .sync_from_github()
        .await
        .context("Couldn't sync repos from gh")?;
    Ok(())
}
