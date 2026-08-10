use leptos::prelude::*;

/// One row: what the work is, and what it is done with.
struct Kit {
    name: &'static str,
    items: &'static [&'static str],
}

/// Ordered along the same axis as the rest of the page — the laboratory end
/// first, the general software end last — rather than by "languages /
/// frameworks / tools", which would say nothing about what the work is.
///
/// Everything here is evidenced by the public repositories or the papers. It is
/// hand-maintained: there is no server function behind it (see UI_REQUIREMENTS.md).
const KIT: &[Kit] = &[
    Kit {
        name: "In the lab",
        items: &[
            "Spectroscopy file formats",
            "Flash photolysis / Glotaran",
            "IR and XRD data",
            "Molecular dynamics",
            "Calibration and curve fitting",
        ],
    },
    Kit {
        name: "Numerical work",
        items: &["Julia", "Python", "Jupyter", "Simulation", "Data analysis"],
    },
    Kit {
        name: "Machine learning",
        items: &[
            "Embedding pipelines",
            "Vector search",
            "Retrieval-augmented generation",
            "Structural bioinformatics",
        ],
    },
    Kit {
        name: "Systems",
        items: &["Rust", "Zig", "OCaml", "CLI tooling", "WebAssembly"],
    },
    Kit {
        name: "Web and infrastructure",
        items: &[
            "Leptos (Rust/WASM)",
            "TypeScript",
            "Actix",
            "PostgreSQL",
            "Qdrant",
            "Docker",
        ],
    },
];

#[component]
pub fn Toolkit() -> impl IntoView {
    let rows = KIT
        .iter()
        .map(|kit| {
            view! {
                <div class="kit-row">
                    <h3 class="kit-name">{kit.name}</h3>
                    <ul class="kit-items">
                        {kit
                            .items
                            .iter()
                            .map(|item| view! { <li>{*item}</li> })
                            .collect::<Vec<_>>()}
                    </ul>
                </div>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <section id="tools" class="section">
            <div class="container">
                <div class="section-head">
                    <h2 class="section-title">"Tools"</h2>
                </div>
                <p class="section-lede">
                    "What I tend to reach for. A good share of the work is reading someone else's
                     undocumented file format and getting the numbers back out of it."
                </p>
                <div class="kit" data-reveal="">
                    {rows}
                </div>
            </div>
        </section>
    }
}
