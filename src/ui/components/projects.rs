use leptos::prelude::*;

use crate::ui::data::projects::{projects, Category, Project};

#[component]
pub fn Projects() -> impl IntoView {
    let all = projects();

    let groups = Category::ordered()
        .into_iter()
        .map(|cat| {
            let rows = all
                .iter()
                .filter(|p| p.category == cat)
                .cloned()
                .enumerate()
                .map(|(i, p)| work_row(p, i))
                .collect::<Vec<_>>();
            view! {
                <div class="work-group">
                    <p class="group-label">{cat.label()}</p>
                    {rows}
                </div>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <section id="work" class="section">
            <div class="container">
                <div class="section-head" data-reveal="">
                    <h2 class="section-title">"Selected " <span class="lit">"work"</span></h2>
                    <p class="section-lede">
                        "One end-to-end thread — from data acquisition and embedding pipelines to a \
                         prediction framework and the applications that ship it."
                    </p>
                </div>
                {groups}
            </div>
        </section>
    }
}

fn work_row(p: Project, i: usize) -> impl IntoView {
    let is_flag = p.title == "Lensing";
    let delay = format!("{}", i * 70);

    let links = p
        .links
        .iter()
        .map(|l| {
            view! {
                <a class="work-link" href=l.url target="_blank" rel="noopener noreferrer">
                    {l.label}
                    <span class="arr" aria-hidden="true">
                        " ↗"
                    </span>
                </a>
            }
        })
        .collect::<Vec<_>>();

    let tech = p
        .tech
        .iter()
        .map(|t| view! { <li>{*t}</li> })
        .collect::<Vec<_>>();

    view! {
        <article class="work-row" class:is-flagship=is_flag data-reveal="" data-reveal-delay=delay>
            <div class="work-head">
                <h3 class="work-name">{p.title}</h3>
                {p.private.then(|| view! { <span class="badge">"private"</span> })}
                {links}
            </div>
            <div class="work-body">
                <p class="work-tagline">{p.tagline}</p>
                <p class="work-desc">{p.description}</p>
                <ul class="tech" aria-label="Tech stack">
                    {tech}
                </ul>
            </div>
        </article>
    }
}
