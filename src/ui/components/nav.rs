use leptos::prelude::*;

#[component]
pub fn Nav() -> impl IntoView {
    view! {
        <header class="site-nav">
            <div class="container nav-inner">
                <a class="brand" href="#top">
                    <span class="brand-mark" aria-hidden="true">
                        "◎"
                    </span>
                    <span class="brand-name">"eduardo gonik"</span>
                </a>
                <nav class="nav-links" aria-label="Primary">
                    <a href="#work">"work"</a>
                    <a href="#skills">"skills"</a>
                    <a href="#research">"research"</a>
                    <a href="#contact">"contact"</a>
                </nav>
            </div>
        </header>
    }
}
