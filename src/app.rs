use crate::ask::components::Ask;
use crate::personal_information::components::{Closing, Masthead};
use crate::portfolio::components::PortfolioProjects;
use crate::ui::components::footer::SiteFooter;
use crate::ui::components::nav::Rail;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Link, Stylesheet, Title};
use leptos_router::path;
use leptos_router::{
    components::{Route, Router, Routes},
    SsrMode, StaticSegment, WildcardSegment,
};

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/egonik-site.css" />

        // One variable font carries the whole page, so it is on the critical path:
        // without the preload the browser only discovers it after the stylesheet
        // parses, and every heading swaps a beat late.
        <Link
            rel="preload"
            href="/fonts/archivo.woff2"
            as_="font"
            type_="font/woff2"
            crossorigin="anonymous"
        />

        <Title text="Eduardo Gonik — chemistry research and the software it needs" />

        <Router>
            <a class="skip-link" href="#code">
                "Skip to content"
            </a>
            <Rail />
            <main>
                <Routes fallback=NotFound>
                    // InOrder, not the default out-of-order streaming: this page has
                    // several <Suspense/> boundaries, and out-of-order resolution
                    // delivers their fragments in completion order. The client then
                    // slots the personal-information payload into the portfolio's
                    // placeholder and aborts hydration. Async also fixes it, at about
                    // 160ms TTFB against InOrder's 100ms.
                    <Route path=StaticSegment("") view=HomePage ssr=SsrMode::InOrder />
                    <Route path=path!("/projects") view=ProjectsPage ssr=SsrMode::InOrder />
                    <Route path=WildcardSegment("any") view=NotFound />
                </Routes>
            </main>
            <SiteFooter />
        </Router>
    }
}

/// The landing page: who, then one prompt over everything he has made, then how
/// to get in touch.
///
/// `<Ask/>` replaces the three separate Code / Papers / Tools sections. It keeps
/// their `#code`, `#papers` and `#tools` anchors on its own group headings, so the
/// nav rail and every existing link still resolve — and it renders from the
/// compiled-in router index, which is why this page no longer needs the
/// `SsrMode::InOrder` workaround described on the route below.
#[component]
fn HomePage() -> impl IntoView {
    view! {
        <Masthead />
        <Ask />
        <Closing />
    }
}

/// `/projects` is the archive on its own, for linking straight at the work.
#[component]
fn ProjectsPage() -> impl IntoView {
    view! {
        <Title text="Code — Eduardo Gonik" />
        <PortfolioProjects />
        <Closing />
    }
}

#[component]
fn NotFound() -> impl IntoView {
    // Setting the status code only makes sense on the server; on the client the
    // response has long since been sent.
    #[cfg(feature = "ssr")]
    {
        let response = expect_context::<leptos_actix::ResponseOptions>();
        response.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <Title text="Not found — Eduardo Gonik" />
        <section class="section">
            <div class="container">
                <div class="section-head">
                    <h1 class="section-title">"Not found"</h1>
                    <p class="section-count">"404"</p>
                </div>
                <p class="section-lede">
                    "That address doesn't exist. The code and the papers are both on the front page."
                </p>
                <div class="masthead-actions">
                    <a class="btn" href="/">
                        "Back to the start"
                    </a>
                </div>
            </div>
        </section>
    }
}
