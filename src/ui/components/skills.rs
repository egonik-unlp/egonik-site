use leptos::prelude::*;

struct Group {
    name: &'static str,
    items: &'static [&'static str],
}

const GROUPS: &[Group] = &[
    Group {
        name: "Languages",
        items: &["Rust", "Python", "TypeScript / JavaScript", "Julia", "Zig", "OCaml", "Go"],
    },
    Group {
        name: "ML & AI",
        items: &[
            "Embeddings",
            "RAG",
            "Vector search (Qdrant, Chroma)",
            "LLM orchestration (LangChain, LlamaIndex)",
            "Structured outputs",
            "PyTorch",
            "burn",
            "Flux.jl",
            "scikit-learn / XGBoost",
            "ONNX",
            "Langfuse",
        ],
    },
    Group {
        name: "Data",
        items: &[
            "ETL pipelines",
            "pandas / pyarrow",
            "Postgres",
            "Redis",
            "Job queues",
            "Web scraping",
        ],
    },
    Group {
        name: "Systems & Backend",
        items: &[
            "Axum",
            "Actix",
            "Diesel",
            "tokio / async",
            "OpenTelemetry / Jaeger",
            "Multi-worker systems",
        ],
    },
    Group {
        name: "Frontend",
        items: &["Leptos (Rust/WASM)", "React", "Three.js", "Web Audio", "Vite"],
    },
    Group {
        name: "Infra & Deploy",
        items: &["Docker & Compose", "Cloudflare Workers", "Self-hosted (Portainer)"],
    },
];

#[component]
pub fn Skills() -> impl IntoView {
    let rows = GROUPS
        .iter()
        .map(|g| {
            let items = g
                .items
                .iter()
                .map(|i| view! { <li>{*i}</li> })
                .collect::<Vec<_>>();
            view! {
                <div class="legend-row" data-reveal="">
                    <div class="legend-name">{g.name}</div>
                    <ul class="legend-tags">{items}</ul>
                </div>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <section id="skills" class="section section-surface">
            <div class="container">
                <div class="section-head" data-reveal="">
                    <h2 class="section-title">"Skills & " <span class="lit">"tools"</span></h2>
                </div>
                <div class="legend">{rows}</div>
            </div>
        </section>
    }
}
