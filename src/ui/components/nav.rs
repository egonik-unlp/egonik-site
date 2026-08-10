use leptos::prelude::*;

/// The nav rail.
///
/// Opaque and sticky from the first paint, deliberately. The nav this replaces
/// was transparent and relied on a scroll script to add `.is-solid` once the
/// hero passed; that script was never loaded, so every link sat near-white on
/// the near-white body for the entire page. Nothing here depends on
/// JavaScript, so that failure mode cannot recur.
///
/// Anchors, not `<A/>`: every target is a fragment on the page this renders in,
/// so the router has nothing to resolve.
#[component]
pub fn Rail() -> impl IntoView {
    view! {
        <header class="rail">
            <div class="container rail-inner">
                <a class="rail-brand" href="#top">
                    <span class="mark" aria-hidden="true">
                        "EG"
                    </span>
                    <span class="name">"Eduardo Gonik"</span>
                </a>
                <nav class="rail-nav" aria-label="Sections">
                    <a href="#code">"Code"</a>
                    <a href="#papers">"Papers"</a>
                    <a href="#tools">"Tools"</a>
                    <a href="#contact">"Contact"</a>
                </nav>
            </div>
        </header>
    }
}
