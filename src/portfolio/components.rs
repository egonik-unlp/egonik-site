use leptos::prelude::*;

use crate::portfolio::dto::PortfolioItemWithMetadataDto;
use crate::portfolio::metadata::ProjectMetadataDto;
use crate::portfolio::server::{get_all_portfolio_items, get_all_portfolio_items_with_metadata};

/// Topics printed per row before the list is cut.
const MAX_TOPICS: usize = 2;
/// Bucket for a project whose metadata records no language at all.
const NO_LANGUAGE: &str = "Unspecified";

/// Word stems that mark a project's subject matter as laboratory science.
///
/// Matched across the repository name, display title, description, `domains`
/// and `categories`. An earlier version read `domains` alone, which worked
/// while every entry in `projects.json` was hand-written. It stopped working
/// the moment the file grew to cover the whole account: the generated entries
/// carry `domains` on 1 of 82, so `glotaran_converter`, `mof_xrd`,
/// `QuenchingLFP` and a dozen other laboratory projects were all landing on the
/// general side of the axis. The description is the signal those entries do
/// have, so the classifier reads it.
///
/// Stems, not exact values, so a repository described as "electrochemistry" or
/// "crystallography" lands correctly with no edit here.
///
/// `materials-science` and `nanomaterial` rather than a bare `material`: the
/// short form fires on ordinary prose — "domain-specific source material",
/// "teaching notebooks and materials" — and quietly classifies by accident.
const RESEARCH_STEMS: &[&str] = &[
    "chem",
    "spectro",
    "glotaran",
    "instrument",
    "materials-science",
    "material-science",
    "nanomaterial",
    "metal-organic",
    "molecul",
    "physic",
    "diffraction",
    "crystallograph",
    "calorimetr",
    "equilibri",
    "thermodynamic",
    "fluorescen",
    "photolysis",
    "calibration",
    "kinetic",
    "cinetica",
    "quenching",
    "docking",
    "deconv",
    "mof",
    "xrd",
    "emission",
    "titration",
    "laborator",
    "fluid-dynamic",
];

/// Deployed instances, keyed by GitHub repository name.
///
/// The published `projects.json` carries these under `links.demo`, but
/// `ProjectMetadataDto` has no `links` field, so serde drops the object before it
/// reaches the UI. Adding that field is a small change to
/// `src/portfolio/metadata.rs`; until then the URLs live here, and this table has
/// to be kept in step by hand.
///
/// Both were requested before shipping — a badge reading "Live" that lands on a
/// 404 is worse than no badge. The first deliberately disagrees with the
/// published `links.demo`, which points at `convert-ffi.onrender.com` and returns
/// 404; the running deployment is `convert-ffi-latest`. See UI_REQUIREMENTS.md §9
/// for that correction and for why `spotify-next-track` cannot render yet.
const LIVE_APPS: &[(&str, &str)] = &[
    ("convert-ffi", "https://convert-ffi-latest.onrender.com/"),
    (
        "spotify-next-track",
        "https://infinite-playlist.eduardo-gonik.workers.dev/",
    ),
];

/// "systems-programming" -> "Systems Programming"
fn display_name(value: &str) -> String {
    value
        .split(['-', '_'])
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            chars
                .next()
                .map(|first| first.to_uppercase().collect::<String>() + chars.as_str())
                .unwrap_or_default()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

fn non_empty(value: &str) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_owned())
}

/// The deployed URL for a repository, if there is one.
///
/// Matched case-insensitively against the *repository* name, not the display
/// title: `metadata.title` renames `app` to "Pathfinder", so a lookup after the
/// fallback chain would miss every renamed project.
fn live_url(repo_name: &str) -> Option<&'static str> {
    LIVE_APPS
        .iter()
        .find(|(name, _)| name.eq_ignore_ascii_case(repo_name))
        .map(|(_, url)| *url)
}

/// "https://convert-ffi.onrender.com/" -> "convert-ffi.onrender.com"
fn display_host(url: &str) -> String {
    url.trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/')
        .to_owned()
}

/// Subjects for one project, broadest first, de-duplicated across the two
/// metadata vocabularies that describe what a thing *is about*.
fn project_topics(metadata: &ProjectMetadataDto) -> Vec<String> {
    let mut topics = Vec::new();
    for source in [&metadata.domains, &metadata.categories] {
        for topic in source.iter().flatten() {
            let topic = topic.trim();
            if !topic.is_empty() && !topics.iter().any(|current| current == topic) {
                topics.push(topic.to_owned());
            }
        }
    }
    topics
}

/// Which side of the axis a project sits on.
///
/// One lowercased haystack built from everything that describes the subject,
/// so a stub entry with nothing but a one-line description still classifies.
fn is_research(repo_name: &str, metadata: &ProjectMetadataDto) -> bool {
    let mut hay = repo_name.to_lowercase();
    for text in [metadata.title.as_deref(), metadata.description.as_deref()]
        .into_iter()
        .flatten()
    {
        hay.push(' ');
        hay.push_str(&text.to_lowercase());
    }
    for source in [&metadata.domains, &metadata.categories] {
        for value in source.iter().flatten() {
            hay.push(' ');
            hay.push_str(&value.to_lowercase());
        }
    }
    RESEARCH_STEMS.iter().any(|stem| hay.contains(stem))
}

/// Whether an entry was written up by hand or generated from the repository.
///
/// `categories` is the discriminator: present on every curated entry in
/// `projects.json` and on none of the generated ones. Generated entries have a
/// name, a one-line description and a language — enough for an index row, not
/// enough for a full one.
fn is_documented(metadata: &ProjectMetadataDto) -> bool {
    metadata
        .categories
        .as_ref()
        .is_some_and(|categories| !categories.is_empty())
}

/// A project plus everything the section needs to sort, group, filter and label
/// it, computed once rather than on every re-render.
#[derive(Clone)]
struct Entry {
    title: String,
    description: String,
    url: Option<String>,
    live: Option<&'static str>,
    /// The first curated language. Exactly one per project, which is what makes
    /// the language facet a partition rather than a stack of overlapping tags.
    language: String,
    languages: Vec<String>,
    status: Option<String>,
    maturity: Option<String>,
    highlights: Vec<String>,
    topics: Vec<String>,
    research: bool,
    documented: bool,
}

impl Entry {
    fn build(item: PortfolioItemWithMetadataDto) -> Self {
        let research = is_research(&item.portfolio_item.title, &item.metadata);
        let documented = is_documented(&item.metadata);
        let topics = project_topics(&item.metadata);
        let PortfolioItemWithMetadataDto {
            portfolio_item,
            metadata,
        } = item;

        // Resolved before `title` is overwritten by the display name.
        let live = live_url(&portfolio_item.title);

        let languages = metadata
            .languages
            .unwrap_or_default()
            .into_iter()
            .filter_map(|language| non_empty(&language))
            .collect::<Vec<_>>();

        Self {
            live,
            language: languages
                .first()
                .cloned()
                .unwrap_or_else(|| NO_LANGUAGE.to_owned()),
            title: metadata
                .title
                .as_deref()
                .and_then(non_empty)
                .unwrap_or(portfolio_item.title),
            description: metadata
                .description
                .as_deref()
                .and_then(non_empty)
                .unwrap_or(portfolio_item.description),
            url: portfolio_item.public_url.as_deref().and_then(non_empty),
            languages,
            status: metadata.status.as_deref().and_then(non_empty),
            maturity: metadata.maturity.as_deref().and_then(non_empty),
            highlights: metadata.highlights.unwrap_or_default(),
            topics,
            research,
            documented,
        }
    }
}

/// The three filter dimensions, combined with AND.
///
/// Each is `None`/`false` when inactive, so the default state is "everything"
/// and no control has to be reset before another one is useful.
#[derive(Clone, Copy, PartialEq, Eq)]
struct Facets {
    axis: Option<bool>,
    documented: bool,
}

impl Facets {
    const OFF: Self = Self {
        axis: None,
        documented: false,
    };

    fn is_active(&self, language: &Option<String>) -> bool {
        self.axis.is_some() || self.documented || language.is_some()
    }
}

/// Does `entry` survive the given filters?
///
/// `skip` names the one dimension to ignore, which is what makes the counts
/// beside each option honest: the number next to "Rust" is how many projects
/// you get *if you press it*, given everything else already selected — not a
/// static total that stops matching what the page does.
fn matches(
    entry: &Entry,
    facets: &Facets,
    language: &Option<String>,
    skip: Option<Dimension>,
) -> bool {
    let check_axis = skip != Some(Dimension::Axis);
    let check_language = skip != Some(Dimension::Language);
    let check_documented = skip != Some(Dimension::Documented);

    if check_axis {
        if let Some(only) = facets.axis {
            if entry.research != only {
                return false;
            }
        }
    }
    if check_language {
        if let Some(only) = language {
            if &entry.language != only {
                return false;
            }
        }
    }
    if check_documented && facets.documented && !entry.documented {
        return false;
    }
    true
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Dimension {
    Axis,
    Language,
    Documented,
}

#[component]
pub fn PortfolioProjects() -> impl IntoView {
    let resource = Resource::new(
        || (),
        |_| async move {
            // The metadata join is best-effort: it reaches out to a static JSON file and
            // drops any project it cannot match. Falling back to the bare archive keeps
            // the section useful when that lookup comes back empty *or* fails outright --
            // an error is only worth surfacing when the plain listing fails too.
            let with_metadata = get_all_portfolio_items_with_metadata().await;
            if matches!(&with_metadata, Ok(items) if !items.is_empty()) {
                return with_metadata;
            }
            match get_all_portfolio_items().await {
                Ok(items) => Ok(items
                    .into_iter()
                    .map(|portfolio_item| PortfolioItemWithMetadataDto {
                        portfolio_item,
                        metadata: ProjectMetadataDto::default(),
                    })
                    .collect()),
                // Both paths failed -- report the metadata error, it is the specific one.
                Err(plain_error) => with_metadata.or(Err(plain_error)),
            }
        },
    );

    view! {
        <section id="code" class="section">
            <div class="container">
                // The resource has to be read *inside* <Suspense/>. Read outside it, the
                // server and the hydrating client disagree about whether the value has
                // landed yet -- one renders the skeleton, the other the rows -- and
                // tachys aborts hydration on the mismatched tree rather than recovering.
                <Suspense fallback=|| {
                    view! {
                        <>
                            <div class="section-head">
                                <h2 class="section-title">"Code"</h2>
                            </div>
                            <div class="skeleton" role="status" aria-label="Loading projects">
                                <span aria-hidden="true"></span>
                                <span aria-hidden="true"></span>
                                <span aria-hidden="true"></span>
                                <span aria-hidden="true"></span>
                            </div>
                        </>
                    }
                }>
                    {move || {
                        resource
                            .get()
                            .map(|result| match result {
                                Ok(items) => view! { <Archive items /> }.into_any(),
                                Err(error) => {
                                    view! {
                                        <>
                                            <div class="section-head">
                                                <h2 class="section-title">"Code"</h2>
                                            </div>
                                            <p class="notice notice-error">
                                                {format!("Projects could not be loaded: {error}")}
                                            </p>
                                        </>
                                    }
                                        .into_any()
                                }
                            })
                    }}
                </Suspense>
            </div>
        </section>
    }
}

#[component]
fn Archive(items: Vec<PortfolioItemWithMetadataDto>) -> impl IntoView {
    // `public` is a per-row visibility flag; nothing upstream filters on it, so honour
    // it here before anything reaches the DOM.
    let mut items = items
        .into_iter()
        .filter(|item| item.portfolio_item.public)
        .collect::<Vec<_>>();

    // The curated `display.priority` alone, which puts the written-up projects first
    // and the generated tail after. No `featured` float and no promoted tiles: that is
    // a ranking of one's own work presented as a layout.
    //
    // `sort_by_cached_key`, not `sort_by_key`: the key allocates a lowercased title and
    // `sort_by_key` would rebuild it on every one of the O(n log n) comparisons.
    items.sort_by_cached_key(|item| {
        (
            item.metadata
                .display
                .as_ref()
                .and_then(|display| display.priority)
                .unwrap_or(i64::MAX),
            item.portfolio_item.title.to_lowercase(),
        )
    });

    let entries = items.into_iter().map(Entry::build).collect::<Vec<_>>();
    let total = entries.len();

    // Language buckets, largest first, ties alphabetical so the row is stable between
    // deploys. `Unspecified` sinks to the end regardless of size — it is the absence of
    // an answer, not one of the answers.
    let mut languages: Vec<String> = Vec::new();
    for entry in &entries {
        if !languages.contains(&entry.language) {
            languages.push(entry.language.clone());
        }
    }
    {
        let counts = |name: &String| entries.iter().filter(|e| &e.language == name).count();
        languages.sort_by_cached_key(|name| {
            (
                name == NO_LANGUAGE,
                std::cmp::Reverse(counts(name)),
                name.to_lowercase(),
            )
        });
    }

    let facets = RwSignal::new(Facets::OFF);
    let language = RwSignal::new(None::<String>);
    let entries = StoredValue::new(entries);

    // How many entries survive every filter except `skip`.
    let count_for = move |skip: Option<Dimension>, probe: &dyn Fn(&Entry) -> bool| {
        let (f, l) = (facets.get(), language.get());
        entries.with_value(|entries| {
            entries
                .iter()
                .filter(|entry| matches(entry, &f, &l, skip) && probe(entry))
                .count()
        })
    };

    let shown = move || count_for(None, &|_| true);
    let clear = move || {
        facets.set(Facets::OFF);
        language.set(None);
    };

    // ── the axis ────────────────────────────────────────────────────────────
    let segment = move |research: bool, label: &'static str| {
        let count = move || count_for(Some(Dimension::Axis), &|e| e.research == research);
        let is_active = move || facets.get().axis == Some(research);
        view! {
            <button
                type="button"
                class="axis-seg"
                class:is-research=research
                // Must be a string, not a bool: Leptos renders `bool` attributes by
                // presence, so `false` would drop `aria-pressed` entirely and the
                // segments would read as plain buttons rather than a toggle set.
                aria-pressed=move || if is_active() { "true" } else { "false" }
                disabled=move || count() == 0 && !is_active()
                // flex-grow carries the proportion, so the bar re-weights as the other
                // filters narrow the set. Both ends print their own count, so the exact
                // number is never inferred from the width.
                style=move || format!("--seg-share:{}", count())
                on:click=move |_| {
                    facets
                        .update(|f| {
                            f.axis = if f.axis == Some(research) { None } else { Some(research) };
                        })
                }
            >
                <span class="axis-n">{count}</span>
                <span class="axis-name">{label}</span>
            </button>
        }
    };

    // ── the facet rows ──────────────────────────────────────────────────────
    let language_options = languages
        .into_iter()
        .map(|name| {
            let label = name.clone();
            // A `StoredValue` rather than a captured `String`: the count and the
            // active test are each read from several attributes, and a closure
            // holding a `String` is not `Copy`, so it could only be used once.
            let name = StoredValue::new(name);
            let count = move || {
                count_for(Some(Dimension::Language), &|entry| {
                    name.with_value(|value| &entry.language == value)
                })
            };
            let is_active = move || {
                language.with(|current| {
                    name.with_value(|value| current.as_deref() == Some(value.as_str()))
                })
            };
            view! {
                <button
                    type="button"
                    class="facet-opt"
                    aria-pressed=move || if is_active() { "true" } else { "false" }
                    disabled=move || count() == 0 && !is_active()
                    on:click=move |_| {
                        let active = is_active();
                        language.set(if active { None } else { Some(name.get_value()) });
                    }
                >
                    {label}
                    <span class="facet-n">{count}</span>
                </button>
            }
        })
        .collect::<Vec<_>>();

    let documented_count = move || count_for(Some(Dimension::Documented), &|e| e.documented);
    let documented_active = move || facets.get().documented;

    view! {
        <div class="section-head">
            <h2 class="section-title">"Code"</h2>
            <p class="section-count" aria-live="polite">
                {move || {
                    if facets.get().is_active(&language.get()) {
                        format!("{} of {total}", shown())
                    } else {
                        format!("{total} projects")
                    }
                }}
            </p>
        </div>
        <p class="section-lede">
            "Everything public on the account — research tooling, side projects, coursework and a
             fair amount of scratch. A good part of it exists because an experiment needed
             something that wasn't available yet: a file format nobody documents, a method from a
             paper with no implementation. The ones I have written up properly carry more detail;
             the rest are here for completeness."
        </p>

        // The axis stays a bar because it is the one split the page is about; the
        // other two dimensions are plain rows of text, so three filters cost two
        // hairlines rather than three widgets.
        <div class="axis" data-reveal="">
            <div
                class="axis-bar"
                role="group"
                aria-label="Filter projects by what they came out of"
            >
                {segment(true, "From the research")}
                {segment(false, "Everything else")}
            </div>

            <div class="facets">
                <div class="facet-row">
                    <h3 class="facet-name" id="facet-language">
                        "Language"
                    </h3>
                    <div class="facet-opts" role="group" aria-labelledby="facet-language">
                        {language_options}
                    </div>
                </div>
                <div class="facet-row">
                    <h3 class="facet-name" id="facet-detail">
                        "Detail"
                    </h3>
                    <div class="facet-opts" role="group" aria-labelledby="facet-detail">
                        <button
                            type="button"
                            class="facet-opt"
                            aria-pressed=move || if documented_active() { "true" } else { "false" }
                            disabled=move || documented_count() == 0 && !documented_active()
                            on:click=move |_| facets.update(|f| f.documented = !f.documented)
                        >
                            "Written up"
                            <span class="facet-n">{documented_count}</span>
                        </button>
                    </div>
                </div>
            </div>

            <div class="axis-foot">
                <p class="axis-caption">
                    "Counts show what each option would leave you with, given the others."
                </p>
                {move || {
                    facets
                        .get()
                        .is_active(&language.get())
                        .then(|| {
                            view! {
                                <button type="button" class="axis-reset" on:click=move |_| clear()>
                                    "Clear filters"
                                </button>
                            }
                        })
                }}
            </div>
        </div>

        <div aria-live="polite">
            {move || {
                let (f, l) = (facets.get(), language.get());
                entries
                    .with_value(|entries| {
                        let tracks = [
                            (
                                true,
                                "From the research",
                                "Parsers, simulations and analysis code written because the lab work needed it.",
                            ),
                            (
                                false,
                                "Everything else",
                                "Infrastructure, side projects, and things I wrote to understand how they work.",
                            ),
                        ];
                        let rendered = tracks
                            .into_iter()
                            .filter_map(|(research, name, note)| {
                                let rows = entries
                                    .iter()
                                    .filter(|entry| {
                                        entry.research == research && matches(entry, &f, &l, None)
                                    })
                                    .map(|entry| view! { <Row entry=entry.clone() /> })
                                    .collect::<Vec<_>>();
                                (!rows.is_empty())
                                    .then(|| {
                                        // An empty track is dropped rather than captioned: with
                                        // three filters combining, "nothing here" is the normal
                                        // case for one side and says nothing worth a heading.
                                        view! {
                                            <section class="track" class:is-research=research>
                                                <div class="track-head">
                                                    <h3 class="track-name">{name}</h3>
                                                    <p class="track-note">{note}</p>
                                                </div>
                                                {rows}
                                            </section>
                                        }
                                    })
                            })
                            .collect::<Vec<_>>();
                        if rendered.is_empty() {
                            return
                            view! {
                                <p class="row-empty">
                                    "Nothing matches all of those at once. Clearing one of them
                                     should bring something back."
                                </p>
                            }
                                .into_any();
                        }
                        view! { <>{rendered}</> }.into_any()
                    })
            }}
        </div>
    }
}

/// One project. Every row carries the same weight — the curated order is the
/// only ranking, and it lives in the sequence rather than in the type size.
#[component]
fn Row(entry: Entry) -> impl IntoView {
    let Entry {
        title,
        description,
        url,
        live,
        languages,
        status,
        maturity,
        highlights,
        topics,
        documented,
        ..
    } = entry;

    view! {
        // Generated entries carry a name, a sentence and a language and nothing
        // else, so they occupy about half the height without anything being
        // hidden — the row is short because the record is short.
        <article class="row" class:is-compact=!documented>
            <h4 class="row-title">
                {match url {
                    Some(url) => {
                        view! {
                            <a href=url target="_blank" rel="noopener noreferrer">
                                {title}
                                <span class="arr" aria-hidden="true">
                                    "↗"
                                </span>
                            </a>
                        }
                            .into_any()
                    }
                    None => view! { <span>{title}</span> }.into_any(),
                }}
            </h4>
            <div>
                <p class="row-desc">{description}</p>
                {(!highlights.is_empty())
                    .then(|| {
                        view! {
                            <ul class="row-highlights">
                                {highlights
                                    .into_iter()
                                    .map(|highlight| view! { <li>{highlight}</li> })
                                    .collect::<Vec<_>>()}
                            </ul>
                        }
                    })}
                // A deployed instance gets its own link rather than replacing the
                // repository one: the two destinations answer different questions,
                // and the host is printed so it is clear which is which before
                // anyone clicks.
                {live
                    .map(|url| {
                        let host = display_host(url);
                        // The three spans are flex items, so the whitespace between
                        // them is stripped and the accessible name would run together
                        // as "Liveconvert-ffi-latest.onrender.com". The label restores
                        // the separation, and still contains the visible text.
                        view! {
                            <a
                                class="row-live"
                                href=url
                                target="_blank"
                                rel="noopener noreferrer"
                                aria-label=format!("Live app: {host}")
                            >
                                <span class="row-live-tag">"Live"</span>
                                <span class="row-live-host">{host.clone()}</span>
                                <span aria-hidden="true">"↗"</span>
                            </a>
                        }
                    })}
            </div>
            <div class="row-meta">
                {(!languages.is_empty())
                    .then(|| view! { <span class="chip is-lang">{languages.join(" / ")}</span> })}
                {status.map(|status| view! { <span class="chip">{display_name(&status)}</span> })}
                {maturity
                    .map(|maturity| view! { <span class="chip">{display_name(&maturity)}</span> })}
                {topics
                    .into_iter()
                    .take(MAX_TOPICS)
                    .map(|topic| view! { <span class="chip is-topic">{topic}</span> })
                    .collect::<Vec<_>>()}
            </div>
        </article>
    }
}
