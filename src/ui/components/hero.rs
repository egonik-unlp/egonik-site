use leptos::prelude::*;

#[component]
pub fn Hero() -> impl IntoView {
    view! {
        <section class="hero" id="top">
            <canvas class="lens-canvas" id="lens-canvas" aria-hidden="true"></canvas>
            <div class="hero-inner container">
                <p class="hero-byline">"Eduardo Gonik"</p>
                <h1 class="hero-title">
                    "I build the data and model "
                    <span class="lit">"infrastructure"</span>
                    " behind "
                    <span class="soft">"intelligent products."</span>
                </h1>
                <p class="hero-lede">
                    "Embedding pipelines, vector search and prediction frameworks — and the \
                     full-stack apps that put models into production."
                </p>
                <div class="hero-actions">
                    <a class="btn btn-primary" href="#work">"See the work"</a>
                    <a class="btn btn-ghost" href="#contact">"Get in touch"</a>
                </div>
                <p class="hero-meta">
                    "data & ai engineer · la plata, ar · rust / python / zig"
                </p>
            </div>
            <div class="hero-scroll" aria-hidden="true">"scroll"</div>
        </section>
    }
}
