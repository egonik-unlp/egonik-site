use crate::publications::dto::PublicationItemWithMetadataDto;
use crate::publications::server::{get_all_publications, get_all_publications_mapped};
use futures_timer::Delay;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use std::cmp::Reverse;

/// The site owner — highlighted inside the author list of every card.
const SITE_AUTHOR: &str = "Eduardo Gonik";
/// How many publications make it into the highlight grid.
const MAX_HIGHLIGHTS: usize = 6;
/// Authors printed before the list is collapsed into "+n more".
const MAX_AUTHORS: usize = 3;
/// Subject chips printed per card.
const MAX_TAGS: usize = 5;

#[component]
pub fn ServerButton() -> impl IntoView {
    let request = Action::new(|_: &()| async move {
        Delay::new(std::time::Duration::new(1, 0));
        get_all_publications().await
    });

    let pending = request.pending();
    let response = request.value();

    view! {
        <div class="space-y-4">
            <button
                class="btn btn-solid"
                on:click=move |_| {
                    request.dispatch(());
                }
                disabled=move || pending.get()
            >
                {move || { if pending.get() { "Loading..." } else { "Call server" } }}
            </button>

            <div>
                {move || {
                    match response.get() {
                        None => {
                            view! { <p class="muted">"The server has not been called yet."</p> }
                                .into_any()
                        }
                        Some(Ok(publications)) => {

                            view! {
                                <div class="table-wrap">
                                    <table class="data-table">
                                        <tr>
                                            <th>"title"</th>
                                            <th>"abs"</th>
                                            <th>"year"</th>
                                            <th>"journal"</th>
                                            <th>"link"</th>
                                        </tr>
                                        {publications
                                            .into_iter()
                                            .map(|publication| {
                                                view! {
                                                    <tr>
                                                        <td class="max-w-sm font-medium">{publication.title}</td>
                                                        <td class="muted max-w-md">{publication.abs}</td>
                                                        <td class="tabular-nums whitespace-nowrap">
                                                            {publication.year}
                                                        </td>
                                                        <td class="muted whitespace-nowrap">
                                                            {publication.journal}
                                                        </td>
                                                        <td>{publication.link}</td>
                                                    </tr>
                                                }
                                            })
                                            .collect_view()}
                                    </table>
                                </div>
                            }
                                .into_any()
                        }
                        Some(Err(error)) => {

                            view! {
                                <p class="notice notice-error">
                                    {format!("Server error: {error}")}
                                </p>
                            }
                                .into_any()
                        }
                    }
                }}
            </div>
        </div>
    }
}

/// Ranks the publications that carry metadata and keeps the strongest few.
///
/// The criterion, applied in order:
/// 1. the hand-curated `featured` flag,
/// 2. the curated `display.priority` (lower number = more prominent),
/// 3. citation count, highest first,
/// 4. year, most recent first.
///
/// Only the top [`MAX_HIGHLIGHTS`] survive — this is a highlight reel, not an index.
fn select_highlights(
    mut items: Vec<PublicationItemWithMetadataDto>,
) -> Vec<PublicationItemWithMetadataDto> {
    items.sort_by_key(|item| {
        let metadata = &item.metadata;
        (
            // `false` sorts before `true`, so negate to float featured work to the top.
            !metadata.featured.unwrap_or(false),
            metadata
                .display
                .as_ref()
                .and_then(|display| display.priority)
                .unwrap_or(i64::MAX),
            Reverse(
                metadata
                    .citations
                    .as_ref()
                    .and_then(|citations| citations.count)
                    .unwrap_or(0),
            ),
            Reverse(metadata.year.unwrap_or(item.publication.year as i64)),
        )
    });
    items.truncate(MAX_HIGHLIGHTS);
    items
}

/// One entry of the author line: a name, a jump over the middle of the list, or
/// a count of the names left off the end.
enum AuthorChunk {
    Name { name: String, is_site_author: bool },
    Gap,
    More(usize),
}

/// Collapses a long author list while guaranteeing the site owner stays visible.
fn author_chunks(authors: &[String]) -> Vec<AuthorChunk> {
    let name_chunk = |name: &String| AuthorChunk::Name {
        name: name.clone(),
        is_site_author: name.eq_ignore_ascii_case(SITE_AUTHOR),
    };

    if authors.len() <= MAX_AUTHORS + 1 {
        return authors.iter().map(name_chunk).collect();
    }

    let site_author_at = authors
        .iter()
        .position(|author| author.eq_ignore_ascii_case(SITE_AUTHOR));

    match site_author_at {
        // The site author is buried in the tail: keep the head, jump to them, count the rest.
        Some(index) if index >= MAX_AUTHORS => {
            let head = MAX_AUTHORS - 1;
            let mut chunks: Vec<AuthorChunk> = authors[..head].iter().map(name_chunk).collect();
            chunks.push(AuthorChunk::Gap);
            chunks.push(name_chunk(&authors[index]));
            let tail = authors.len() - index - 1;
            if tail > 0 {
                chunks.push(AuthorChunk::More(tail));
            }
            chunks
        }
        _ => {
            let mut chunks: Vec<AuthorChunk> =
                authors[..MAX_AUTHORS].iter().map(name_chunk).collect();
            chunks.push(AuthorChunk::More(authors.len() - MAX_AUTHORS));
            chunks
        }
    }
}

/// "journal-article" -> "Journal article"
fn humanize(raw: &str) -> String {
    let spaced = raw.replace(['-', '_'], " ");
    let mut chars = spaced.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => spaced,
    }
}

fn non_empty(value: &str) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

#[component]
pub fn PublicationsWithMetadata() -> impl IntoView {
    let resource = Resource::new(
        || (),
        |_| async move { get_all_publications_mapped().await },
    );

    view! {
        <section id="publications" class="section section-surface">
            <div class="container">
                <div class="section-head" data-reveal="">
                    <h2 class="section-title">
                        "Selected " <span class="lit">"publications"</span>
                    </h2>
                    <p class="section-lede">
                        "Peer-reviewed work, ranked by editorial priority and citation weight, \
                         enriched with metadata pulled from OpenAlex and Crossref."
                    </p>
                </div>

                <Suspense fallback=move || {
                    view! { <PublicationsSkeleton /> }
                }>
                    <ErrorBoundary fallback=move |errors| {
                        view! {
                            <p class="notice notice-error">
                                {move || {
                                    format!(
                                        "Publications could not be loaded: {}",
                                        errors
                                            .get()
                                            .into_iter()
                                            .map(|(_, error)| error.to_string())
                                            .collect::<Vec<_>>()
                                            .join("; "),
                                    )
                                }}
                            </p>
                        }
                    }>
                        {move || {
                            resource
                                .get()
                                .map(|result| {
                                    result
                                        .map(|publications| {
                                            let total = publications.len();
                                            let highlights = select_highlights(publications);
                                            if highlights.is_empty() {
                                                return view! {
                                                    <p class="pub-empty">
                                                        "No publication metadata available right now."
                                                    </p>
                                                }
                                                    .into_any();
                                            }
                                            let cards = highlights
                                                .into_iter()
                                                .enumerate()
                                                .map(|(index, item)| {
                                                    view! { <PublicationItemWithMetadata item lead=index == 0 /> }
                                                })
                                                .collect::<Vec<_>>();
                                            view! {
                                                <p class="pub-count mono">
                                                    {format!("{} of {} indexed works", MAX_HIGHLIGHTS.min(total), total)}
                                                </p>
                                                <div class="pubcard-grid">{cards}</div>
                                            }
                                                .into_any()
                                        })
                                })
                        }}
                    </ErrorBoundary>
                </Suspense>
            </div>
        </section>
    }
}

/// Placeholder cards shown while the server function resolves.
#[component]
fn PublicationsSkeleton() -> impl IntoView {
    let cards = (0..3)
        .map(|_| {
            view! {
                <div class="pubcard is-skeleton" aria-hidden="true">
                    <span class="skeleton-line is-30"></span>
                    <span class="skeleton-line is-90"></span>
                    <span class="skeleton-line is-70"></span>
                    <span class="skeleton-line is-50"></span>
                </div>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <div class="pubcard-grid" role="status" aria-label="Loading publications">
            {cards}
        </div>
    }
}

/// A single publication rendered as a metadata card.
///
/// `lead` promotes the top-ranked entry to a wider, larger card.
#[component]
pub fn PublicationItemWithMetadata(
    item: PublicationItemWithMetadataDto,
    #[prop(optional)] lead: bool,
) -> impl IntoView {
    let PublicationItemWithMetadataDto {
        publication,
        metadata,
    } = item;

    let title = metadata
        .title
        .clone()
        .and_then(|title| non_empty(&title))
        .unwrap_or_else(|| publication.title.clone());
    let year = metadata.year.unwrap_or(publication.year as i64);
    let featured = metadata.featured.unwrap_or(false);
    let kind = metadata.type_.as_deref().map(humanize);
    // Left as-is ("co-author", "first-author") — the hyphen reads better in mono.
    let role = metadata.role.as_deref().and_then(non_empty);

    let doi = metadata
        .identifiers
        .as_ref()
        .and_then(|identifiers| identifiers.doi.clone());
    let urls = metadata.urls.as_ref();
    let primary_url = urls
        .and_then(|urls| urls.primary.clone())
        .or_else(|| doi.as_ref().map(|doi| format!("https://doi.org/{doi}")))
        .or_else(|| non_empty(&publication.link));
    let open_access_url = urls.and_then(|urls| urls.open_access.clone());

    let venue = metadata
        .venue
        .clone()
        .and_then(|venue| non_empty(&venue))
        .or_else(|| non_empty(&publication.journal));

    // "vol. 16 · no. 7 · pp. 6689–6705" — only the parts that exist.
    let locator = {
        let mut parts = Vec::new();
        if let Some(volume) = metadata.volume.as_deref().and_then(non_empty) {
            parts.push(format!("vol. {volume}"));
        }
        if let Some(issue) = metadata.issue.as_deref().and_then(non_empty) {
            parts.push(format!("no. {issue}"));
        }
        if let Some(pages) = metadata.pages.as_deref().and_then(non_empty) {
            parts.push(format!("pp. {pages}"));
        }
        if let Some(article) = metadata.article_number.as_deref().and_then(non_empty) {
            parts.push(format!("art. {article}"));
        }
        if let Some(publisher) = metadata.publisher.as_deref().and_then(non_empty) {
            parts.push(publisher);
        }
        (!parts.is_empty()).then(|| parts.join(" · "))
    };

    let description = metadata
        .description
        .as_deref()
        .and_then(non_empty)
        .or_else(|| non_empty(&publication.abs));

    let date = metadata
        .publication_date
        .clone()
        .or_else(|| metadata.presentation_date.clone())
        .and_then(|date| non_empty(&date));

    let citations = metadata.citations.as_ref();
    let citation_count = citations.and_then(|citations| citations.count);
    let citation_source = citations.and_then(|citations| {
        let source = citations.source.clone()?;
        Some(match citations.as_of.clone() {
            Some(as_of) => format!("{source}, as of {as_of}"),
            None => source,
        })
    });

    let authors = metadata
        .authors
        .clone()
        .map(|authors| author_chunks(&authors))
        .filter(|chunks| !chunks.is_empty())
        .map(|chunks| {
            chunks
                .into_iter()
                .map(|chunk| match chunk {
                    AuthorChunk::Name {
                        name,
                        is_site_author,
                    } => {
                        let class = if is_site_author {
                            "pub-author is-self"
                        } else {
                            "pub-author"
                        };
                        view! { <li class=class>{name}</li> }.into_any()
                    }
                    AuthorChunk::Gap => {
                        view! { <li class="pub-author is-more">"…"</li> }.into_any()
                    }
                    AuthorChunk::More(count) => {
                        view! { <li class="pub-author is-more">{format!("+{count} more")}</li> }
                            .into_any()
                    }
                })
                .collect::<Vec<_>>()
        });

    // Subjects, broadest first, de-duplicated across the three metadata vocabularies.
    let tags = {
        let mut tags: Vec<String> = Vec::new();
        for source in [&metadata.categories, &metadata.domains, &metadata.keywords] {
            for tag in source.iter().flatten() {
                let tag = tag.trim().to_lowercase();
                if !tag.is_empty() && !tags.contains(&tag) {
                    tags.push(tag);
                }
            }
        }
        tags.truncate(MAX_TAGS);
        (!tags.is_empty()).then_some(tags)
    };
    let tags = tags.map(|tags| {
        tags.into_iter()
            .map(|tag| view! { <li class="pub-tag">{humanize(&tag)}</li> })
            .collect::<Vec<_>>()
    });

    let card_class = if lead {
        "pubcard is-lead"
    } else {
        "pubcard"
    };

    // No `data-reveal` here: interactions.js snapshots those elements once at
    // load, and these cards only exist after the Suspense payload streams in —
    // they would never be observed, and would stay at opacity 0 forever.
    view! {
        <article class=card_class>
            <header class="pubcard-top">
                {kind.map(|kind| view! { <span class="pubcard-kind">{kind}</span> })}
                {featured
                    .then(|| {
                        view! {
                            <span class="pubcard-flag">
                                <span aria-hidden="true">"◆ "</span>
                                "featured"
                            </span>
                        }
                    })}
                <time class="pubcard-year" datetime=date.clone().unwrap_or_else(|| year.to_string())>
                    {year.to_string()}
                </time>
            </header>

            <h3 class="pubcard-title">
                {match primary_url.clone() {
                    Some(url) => {
                        view! {
                            <a href=url target="_blank" rel="noopener noreferrer">
                                {title.clone()}
                                <span class="arr" aria-hidden="true">
                                    " ↗"
                                </span>
                            </a>
                        }
                            .into_any()
                    }
                    None => view! { <span>{title.clone()}</span> }.into_any(),
                }}
            </h3>

            {venue
                .map(|venue| {
                    view! {
                        <p class="pubcard-venue">
                            <span class="pubcard-journal">{venue}</span>
                            {locator.map(|locator| view! { <span class="pubcard-locator">{locator}</span> })}
                        </p>
                    }
                })}

            {description.map(|description| view! { <p class="pubcard-desc">{description}</p> })}

            {authors.map(|authors| view! { <ul class="pubcard-authors">{authors}</ul> })}

            {tags.map(|tags| view! { <ul class="pubcard-tags">{tags}</ul> })}

            <footer class="pubcard-foot">
                {citation_count
                    .map(|count| {
                        view! {
                            <span class="pubcard-metric" title=citation_source.unwrap_or_default()>
                                <b class="pubcard-metric-value">{count.to_string()}</b>
                                {if count == 1 { " citation" } else { " citations" }}
                            </span>
                        }
                    })}
                {role.map(|role| view! { <span class="pubcard-role">{role}</span> })}
                {doi
                    .map(|doi| {
                        view! {
                            <a
                                class="pubcard-link"
                                href=format!("https://doi.org/{doi}")
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                {format!("doi:{doi}")}
                            </a>
                        }
                    })}
                {open_access_url
                    .map(|url| {
                        view! {
                            <a
                                class="pubcard-link is-open"
                                href=url
                                target="_blank"
                                rel="noopener noreferrer"
                            >
                                "Open access"
                                <span aria-hidden="true">" ↗"</span>
                            </a>
                        }
                    })}
            </footer>
        </article>
    }
}
