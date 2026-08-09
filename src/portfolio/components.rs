use leptos::prelude::*;

use crate::portfolio::dto::PortfolioItemWithMetadataDto;
use crate::portfolio::metadata::ProjectMetadataDto;
use crate::portfolio::server::{get_all_portfolio_items, get_all_portfolio_items_with_metadata};

/// Topics printed per row before the list is cut.
const MAX_TOPICS: usize = 2;

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
/// `categories` is the discriminator: it is present on all 22 curated entries in
/// `projects.json` and on none of the 82 generated ones. Generated entries have
/// a name, a one-line description and a language — enough for an index row, not
/// enough for a full one — so the row renders tighter rather than leaving a
/// documented-shaped gap where the missing fields would go.
fn is_documented(metadata: &ProjectMetadataDto) -> bool {
    metadata
        .categories
        .as_ref()
        .is_some_and(|categories| !categories.is_empty())
}

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
///
/// The scheme is noise next to a link that already says where it goes, and the
/// trailing slash makes two otherwise identical hosts look different.
fn display_host(url: &str) -> String {
    url.trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/')
        .to_owned()
}

/// A project plus everything the section needs to sort, group and label it,
/// computed once rather than on every re-render.
#[derive(Clone)]
struct Entry {
    title: String,
    description: String,
    url: Option<String>,
    live: Option<&'static str>,
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

        Self {
            live,
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
            languages: metadata
                .languages
                .unwrap_or_default()
                .into_iter()
                .filter_map(|language| non_empty(&language))
                .collect(),
            status: metadata.status.as_deref().and_then(non_empty),
            maturity: metadata.maturity.as_deref().and_then(non_empty),
            highlights: metadata.highlights.unwrap_or_default(),
            topics,
            research,
            documented,
        }
    }
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

    // The curated `display.priority` alone. The previous build floated `featured`
    // projects to the top and promoted the first three into large tiles, which is a
    // ranking of one's own work presented as a layout. Priority is the author's own
    // ordering and needs no further editorialising.
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
    let research_count = entries.iter().filter(|entry| entry.research).count();
    let general_count = total - research_count;

    // `None` is "both tracks"; `Some(true)` narrows to the research end of the axis.
    let selected = RwSignal::new(None::<bool>);
    let entries = StoredValue::new(entries);

    let segment = move |research: bool, label: &'static str, count: usize| {
        let is_active = move || selected.get() == Some(research);
        view! {
            <button
                type="button"
                class="axis-seg"
                class:is-research=research
                // Must be a string, not a bool: Leptos renders `bool` attributes by
                // presence, so `false` would drop `aria-pressed` entirely and the
                // segments would read as plain buttons rather than a toggle set.
                aria-pressed=move || if is_active() { "true" } else { "false" }
                // flex-grow carries the proportion; `min-width` keeps a small bucket
                // clickable. Both ends print their own count, so the exact number is
                // never inferred from the width.
                style=format!("--seg-share:{count}")
                on:click=move |_| {
                    selected
                        .update(|current| {
                            *current = if *current == Some(research) {
                                None
                            } else {
                                Some(research)
                            };
                        })
                }
            >
                <span class="axis-n">{count}</span>
                <span class="axis-name">{label}</span>
            </button>
        }
    };

    view! {
        <div class="section-head">
            <h2 class="section-title">"Code"</h2>
            // "projects", not "public repositories": this list is the intersection of
            // the GitHub account with the curated `projects.json`, so it is always a
            // subset of what is public. Labelling the subset with the superset's name
            // reads as a claim about the whole account.
            <p class="section-count" aria-live="polite">
                {move || match selected.get() {
                    None => format!("{total} projects"),
                    Some(true) => format!("{research_count} of {total}"),
                    Some(false) => format!("{general_count} of {total}"),
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

        // The axis: one bar, two ends, real proportions.
        <div class="axis" data-reveal="">
            <div
                class="axis-bar"
                role="group"
                aria-label="Filter projects by what they came out of"
            >
                {segment(true, "From the research", research_count)}
                {segment(false, "Everything else", general_count)}
            </div>
            <div class="axis-foot">
                <p class="axis-caption">
                    "Split by subject matter — whether a repository's domain is a laboratory one."
                </p>
                {move || {
                    selected
                        .get()
                        .map(|_| {
                            view! {
                                <button
                                    type="button"
                                    class="axis-reset"
                                    on:click=move |_| selected.set(None)
                                >
                                    "Show both"
                                </button>
                            }
                        })
                }}
            </div>
        </div>

        <div aria-live="polite">
            {move || {
                let active = selected.get();
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
                        tracks
                            .into_iter()
                            .filter(|(research, _, _)| active.is_none_or(|only| only == *research))
                            .map(|(research, name, note)| {
                                let rows = entries
                                    .iter()
                                    .filter(|entry| entry.research == research)
                                    .map(|entry| view! { <Row entry=entry.clone() /> })
                                    .collect::<Vec<_>>();
                                if rows.is_empty() {
                                    return view! {
                                        <p class="row-empty">
                                            "Nothing on this side of the axis yet."
                                        </p>
                                    }
                                        .into_any();
                                }
                                view! {
                                    <section class="track" class:is-research=research>
                                        <div class="track-head">
                                            <h3 class="track-name">{name}</h3>
                                            <p class="track-note">{note}</p>
                                        </div>
                                        {rows}
                                    </section>
                                }
                                    .into_any()
                            })
                            .collect::<Vec<_>>()
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
                // anyone clicks. Accessible name reads "Live, <host>".
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
