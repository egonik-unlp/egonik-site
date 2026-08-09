use leptos::prelude::*;

/// Colophon. Small, factual, and honest about how the page is built — which on
/// a site whose subject is writing software is content, not decoration.
#[component]
pub fn SiteFooter() -> impl IntoView {
    view! {
        <footer class="site-footer">
            <div class="container footer-inner">
                <span>"Eduardo Gonik — La Plata, Argentina"</span>
                <span>
                    "Rust · Leptos · Actix · Postgres. Set in "
                    <a
                        href="https://fonts.google.com/specimen/Archivo"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        "Archivo"
                    </a> "."
                </span>
            </div>
        </footer>
    }
}
