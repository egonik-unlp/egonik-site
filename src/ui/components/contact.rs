use leptos::prelude::*;

// TODO: replace with Eduardo's real LinkedIn URL.
const LINKEDIN_URL: &str = "https://www.linkedin.com/in/";

#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <section id="contact" class="section contact">
            <div class="container narrow">
                <div class="section-head" data-reveal="">
                    <h2 class="section-title contact-title">"Let's " <span class="lit">"talk."</span></h2>
                    <p class="section-lede">
                        "Data & AI engineering, a collaboration, or just comparing notes on Rust \
                         and vector databases — reach out."
                    </p>
                </div>
                <ul class="contact-links" data-reveal="">
                    <li>
                        <a href="mailto:eduardogonik@gmail.com">
                            <span class="contact-key">"email"</span>
                            <span class="contact-val">"eduardogonik@gmail.com"</span>
                            <span class="contact-arr" aria-hidden="true">"↗"</span>
                        </a>
                    </li>
                    <li>
                        <a href="https://github.com/egonik-unlp" target="_blank" rel="noopener noreferrer">
                            <span class="contact-key">"github"</span>
                            <span class="contact-val">"egonik-unlp"</span>
                            <span class="contact-arr" aria-hidden="true">"↗"</span>
                        </a>
                    </li>
                    <li>
                        <a href=LINKEDIN_URL target="_blank" rel="noopener noreferrer">
                            <span class="contact-key">"linkedin"</span>
                            <span class="contact-val">"connect"</span>
                            <span class="contact-arr" aria-hidden="true">"↗"</span>
                        </a>
                    </li>
                </ul>
            </div>
        </section>
    }
}
