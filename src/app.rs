use crate::personal_information::components::{WhoAmI, WhoAmIContact};
use crate::personal_information::server::get_full_personal_info;
use crate::portfolio::components::GithubRepoLoader;
use crate::publications::components::ServerButton;
use crate::ui::components::contact::Contact;
use crate::ui::components::hero::Hero;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/egonik-site.css"/>

        // sets the document title
        <Title text="Holaaa"/>

        // content for this welcome page
        <Router>
            <main class="">
            // <main class="mx-auto max-w-3xl space-y-8 px-6 py-12">
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=HomePage/>
                    <Route path=WildcardSegment("any") view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;
    let personal = Resource::new(
        move || count.get(),
        |_| async move { get_full_personal_info().await },
    );
    view! {
                <h1 class="text-3xl font-semibold tracking-tight">"Welcome to Leptos!"</h1>
                <button class="btn btn-solid" on:click=on_click>"Click Me: " {count}</button>

    <Hero/>

            <ServerButton/>
              <ErrorBoundary
                fallback=move |errors| {
                view! {
                    <p class="notice notice-error">
                {format!("Errors produced during loading: {:?}", errors.get())}
                    </p>
                }
            }
            >
                <Suspense
                fallback=move || view! { <p class="muted">"Loading..."</p> }
                >
        {
                move || personal.and_then(|(personal_information, contact_information)| view! {
                    <WhoAmIContact personal_information = personal_information.clone() contact_information = contact_information.clone()/>
            }  ) }
                </Suspense>

            </ErrorBoundary>
            <GithubRepoLoader/>
            <Contact/>
        }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1 class="text-3xl font-semibold tracking-tight">"Not Found"</h1>
    }
}
