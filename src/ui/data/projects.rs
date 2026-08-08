//! Portfolio content. Editing the site is mostly editing this file.

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Category {
    AiMl,
    Systems,
    Visual,
    Scientific,
}

impl Category {
    pub fn label(&self) -> &'static str {
        match self {
            Category::AiMl => "ai & machine learning",
            Category::Systems => "systems & backend",
            Category::Visual => "visual & creative",
            Category::Scientific => "scientific computing",
        }
    }

    /// Display order for the Projects section.
    pub fn ordered() -> [Category; 3] {
        [Category::AiMl, Category::Systems, Category::Visual]
    }
}

#[derive(Clone, Copy)]
pub struct Link {
    pub label: &'static str,
    pub url: &'static str,
}

#[derive(Clone)]
pub struct Project {
    pub title: &'static str,
    pub tagline: &'static str,
    pub description: &'static str,
    pub tech: &'static [&'static str],
    pub links: &'static [Link],
    pub category: Category,
    /// Repository is private (site is private too, so links still shown).
    pub private: bool,
}

pub fn projects() -> Vec<Project> {
    vec![
        // ---- AI & Machine Learning ---------------------------------------
        Project {
            title: "Lensing",
            tagline: "A prediction-lab framework that reshapes itself around your data.",
            description: "Point Lensing at a Qdrant corpus of embedded documents, declare a target \
                in domain.toml, and the whole lab bends to fit: dataset levers, a predictor zoo, an \
                Axum server with a React \"Control Room\" UI, and ONNX export all take your domain's \
                shape. A Zig build runner drives four toolchains at once. Instances built on it \
                predict Argentine real-estate prices (in production at Snappler), flag listing \
                anomalies, and model Spotify taste-fit.",
            tech: &[
                "Rust", "burn", "Python", "PyTorch", "Julia", "Flux.jl", "scikit-learn", "Qdrant",
                "ONNX", "Axum", "React", "Zig",
            ],
            links: &[Link { label: "GitHub", url: "https://github.com/egonik-unlp/lensing" }],
            category: Category::AiMl,
            private: true,
        },
        Project {
            title: "Pathfinder",
            tagline: "A 3D musical journey through your Spotify taste-space.",
            description: "The application layer of the Lensing ecosystem: a visualizer that plots a \
                path between songs through a learned taste-space, consuming an ONNX scorer exported \
                from the taste-fit prediction instance. Rust compiled to WebAssembly runs the search \
                in the browser, behind a React + Three.js front-end and a Cloudflare Worker backend.",
            tech: &["Rust/WASM", "React", "Three.js", "ONNX", "Cloudflare Workers"],
            links: &[Link { label: "GitHub", url: "https://github.com/egonik-unlp/app" }],
            category: Category::AiMl,
            private: true,
        },
        Project {
            title: "lvv",
            tagline: "An embedding pipeline from raw datasets to a vector database.",
            description: "A Rust pipeline that embeds datasets with LLMs (Ollama or OpenAI) and \
                loads them into a Qdrant vector database, with on-disk caching and a job queue. \
                Pluggable sources (CSV, Postgres) and sinks — the ingestion layer that builds the \
                corpora models learn from.",
            tech: &["Rust", "Qdrant", "OpenAI", "Ollama", "Postgres", "tokio"],
            links: &[Link { label: "GitHub", url: "https://github.com/egonik-unlp/lvv" }],
            category: Category::AiMl,
            private: false,
        },
        Project {
            title: "craig",
            tagline: "A retrieval-augmented chemistry assistant.",
            description: "A RAG assistant over a physical-chemistry textbook: it ingests the PDF, \
                builds embeddings into Qdrant/Chroma, and answers questions with retrieved context \
                over an HTTP API and a Telegram bot — instrumented end-to-end with Langfuse / \
                OpenTelemetry tracing.",
            tech: &[
                "TypeScript", "LangChain", "LlamaIndex", "Qdrant", "Chroma", "OpenAI", "Langfuse",
            ],
            links: &[Link { label: "GitHub", url: "https://github.com/egonik-unlp/craig" }],
            category: Category::AiMl,
            private: false,
        },
        Project {
            title: "Real-estate API discovery",
            tagline: "Scrape the open web, then let an LLM judge what's real.",
            description: "A two-stage data-acquisition pipeline: a Rust async crawler scans \
                real-estate sites for candidate public API endpoints and emits scored CSV reports, \
                then a cost-aware LLM classifier (LangChain.js with structured Zod output, batching \
                and caching) grades each endpoint from \"confirmed agency API\" to \"not an API\". \
                The clean listings feed the price and anomaly models.",
            tech: &["Rust", "tokio", "reqwest", "TypeScript", "LangChain.js", "Zod"],
            links: &[Link {
                label: "GitHub",
                url: "https://github.com/egonik-unlp/detect-open-realstate-apis",
            }],
            category: Category::AiMl,
            private: false,
        },
        // ---- Systems & Backend -------------------------------------------
        Project {
            title: "convert-invert",
            tagline: "A Spotify → Soulseek bridge, built like production infrastructure.",
            description: "A synchronization engine that matches Spotify playlist tracks against the \
                Soulseek P2P network and downloads the best candidate, chosen by a Levenshtein-based \
                judge. A multi-worker pool over Postgres and Redis, API-key auth with rate limiting, \
                and full OpenTelemetry / Jaeger tracing — deployed and running on my home server.",
            tech: &[
                "Rust", "Actix", "Diesel", "Postgres", "Redis", "OpenTelemetry", "Jaeger", "Docker",
            ],
            links: &[Link { label: "GitHub", url: "https://github.com/egonik-unlp/convert-invert" }],
            category: Category::Systems,
            private: false,
        },
        Project {
            title: "convert-songs",
            tagline: "Spotify playlists from a folder of MP3s — hand-built in Zig.",
            description: "A from-scratch Zig CLI that walks a music folder, parses ID3 tags, looks \
                tracks up on Spotify and builds playlists — with the OAuth2 flow, token management \
                and API client all implemented by hand. A study in systems work without a framework.",
            tech: &["Zig", "OAuth2", "Spotify API", "ID3"],
            links: &[Link { label: "GitHub", url: "https://github.com/egonik-unlp/convert-songs" }],
            category: Category::Systems,
            private: false,
        },
        // ---- Visual & Creative -------------------------------------------
        Project {
            title: "songViz",
            tagline: "A 3D spectrogram you can mix in real time.",
            description: "A browser audio visualizer that decodes local tracks, computes their FFT \
                with the Web Audio API, and renders a scrolling 3D spectrogram terrain in Three.js. \
                Mix mode blends two tracks with per-track 3-band EQ and an equal-power crossfader, \
                each drawn as its own colour-mapped wireframe.",
            tech: &["JavaScript", "Three.js", "Web Audio API", "Vite"],
            links: &[Link { label: "GitHub", url: "https://github.com/egonik-unlp/songVizClaude" }],
            category: Category::Visual,
            private: false,
        },
        Project {
            title: "flagen",
            tagline: "Generative art from Monte-Carlo processes.",
            description: "A recreational numerical-computing project in OCaml: parallel random \
                walkers accumulate over a large canvas via a log-integral Monte-Carlo process, \
                producing 2D and 3D surface artwork alongside raw matrices. Built on the Owl \
                numerical library.",
            tech: &["OCaml", "Owl", "Monte Carlo", "Python"],
            links: &[Link { label: "GitHub", url: "https://github.com/egonik-unlp/flagen" }],
            category: Category::Visual,
            private: false,
        },
    ]
}
