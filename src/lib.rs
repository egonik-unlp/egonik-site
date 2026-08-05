#![allow(dead_code, unused, clippy::from_over_into)]

pub mod app;

pub mod core;
pub mod database;
pub mod entrypoint;
pub mod job_experience;
pub mod personal_information;
pub mod portfolio;
pub mod publications;

#[cfg(feature = "ssr")]
pub mod schema;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
