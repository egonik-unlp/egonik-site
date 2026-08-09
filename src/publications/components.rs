use leptos::prelude::*;

use crate::publications::dto::PublicationItemWithMetadataDto;
use crate::publications::server::get_all_publications_mapped;

/// The site owner — marked inside every author line.
const SITE_AUTHOR: &str = "Eduardo Gonik";
/// Authors printed before the list is collapsed into "+n more".
const MAX_AUTHORS: usize = 3;
/// Subject chips printed per entry.
const MAX_TAGS: usize = 4;

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

/// One entry of the author line: a name, a jump over the middle of the list, or
/// a count of the names left off the end.
enum AuthorChunk {
    Name { name: String, is_site_author: bool },
    Gap,
    More(usize),
}

/// Collapses a long author list while keeping the site owner visible. These are
/// group papers with up to sixteen names; the point of the marking is to show
/// where in the order he actually sits, not to move him to the front.
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

#[component]
pub fn PublicationsWithMetadata() -> impl IntoView {
    let resource = Resource::new(
        || (),
        |_| async move { get_all_publications_mapped().await },
    );

    view! {
        <section id="papers" class="section section-band">
            <div class="container">
                // Must be read inside <Suspense/>: outside it the server streams the
                // fallback while the hydrating client already holds the data, and the
                // mismatched trees abort hydration instead of recovering.
                <Suspense fallback=|| {
                    view! {
                        <>
                            <div class="section-head">
                                <h2 class="section-title">"Papers"</h2>
                            </div>
                            <div class="skeleton" role="status" aria-label="Loading publications">
                                <span aria-hidden="true"></span>
                                <span aria-hidden="true"></span>
                                <span aria-hidden="true"></span>
                            </div>
                        </>
                    }
                }>
                    {move || match resource.get() {
                        None => None,
                        Some(Err(error)) => {
                            Some(
                                view! {
                                    <>
                                        <div class="section-head">
                                            <h2 class="section-title">"Papers"</h2>
                                        </div>
                                        <p class="notice notice-error">
                                            {format!("Publications could not be loaded: {error}")}
                                        </p>
                                    </>
                                }
                                    .into_any(),
                            )
                        }
                        Some(Ok(publications)) => {
                            Some(view! { <Catalogue publications /> }.into_any())
                        }
                    }}
                </Suspense>
            </div>
        </section>
    }
}

#[component]
fn Catalogue(publications: Vec<PublicationItemWithMetadataDto>) -> impl IntoView {
    let mut publications = publications;

    // Newest first, then by the curated priority inside a year. The previous build
    // ranked the whole list by citation count, which quietly sorted a short
    // bibliography by how well each entry had done. A catalogue is chronological.
    publications.sort_by_key(|item| {
        let metadata = &item.metadata;
        (
            std::cmp::Reverse(metadata.year.unwrap_or(item.publication.year as i64)),
            metadata
                .display
                .as_ref()
                .and_then(|display| display.priority)
                .unwrap_or(i64::MAX),
        )
    });

    let total = publications.len();

    if publications.is_empty() {
        return view! {
            <>
                <div class="section-head">
                    <h2 class="section-title">"Papers"</h2>
                </div>
                <p class="row-empty">"No publication metadata available right now."</p>
            </>
        }
        .into_any();
    }

    // Runs of equal years become groups; the list is already sorted, so a single
    // pass is enough and the year axis stays in step with the entries under it.
    let mut groups: Vec<(i64, Vec<PublicationItemWithMetadataDto>)> = Vec::new();
    for item in publications {
        let year = item.metadata.year.unwrap_or(item.publication.year as i64);
        match groups.last_mut() {
            Some((current, bucket)) if *current == year => bucket.push(item),
            _ => groups.push((year, vec![item])),
        }
    }

    let groups = groups
        .into_iter()
        .map(|(year, items)| {
            let count = items.len();
            let entries = items
                .into_iter()
                .map(|item| view! { <Work item /> })
                .collect::<Vec<_>>();
            view! {
                <div class="year-group" data-reveal="">
                    <h3 class="year-mark">
                        <time datetime=year.to_string()>{year.to_string()}</time>
                        <span>
                            {if count == 1 {
                                "1 work".to_owned()
                            } else {
                                format!("{count} works")
                            }}
                        </span>
                    </h3>
                    <div>{entries}</div>
                </div>
            }
        })
        .collect::<Vec<_>>();

    view! {
        <>
            <div class="section-head">
                <h2 class="section-title">"Papers"</h2>
                <p class="section-count">
                    {if total == 1 { "1 work".to_owned() } else { format!("{total} works") }}
                </p>
            </div>
            <p class="section-lede">
                "Nanomaterials and photochemistry, nearly all of it group work with colleagues at
                 INIFTA. Several of the repositories above exist because one of these needed
                 something first. Everything links out to the source."
            </p>
            <div class="years">{groups}</div>
        </>
    }
    .into_any()
}

/// A single publication. Every field is optional upstream, so each one either
/// renders or leaves no trace — there are no empty labels waiting for data.
#[component]
fn Work(item: PublicationItemWithMetadataDto) -> impl IntoView {
    let PublicationItemWithMetadataDto {
        publication,
        metadata,
    } = item;

    let title = metadata
        .title
        .as_deref()
        .and_then(non_empty)
        .unwrap_or_else(|| publication.title.clone());
    let kind = metadata.type_.as_deref().map(humanize);

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

    // The database columns for both of these are written as empty strings by the
    // sync job, so the metadata is the only real source; the fallback is kept for
    // rows that predate it.
    let venue = metadata
        .venue
        .as_deref()
        .and_then(non_empty)
        .or_else(|| non_empty(&publication.journal));
    let description = metadata
        .description
        .as_deref()
        .and_then(non_empty)
        .or_else(|| non_empty(&publication.abs));

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
        (!parts.is_empty()).then(|| parts.join(" · "))
    };

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
        .as_deref()
        .map(author_chunks)
        .filter(|chunks| !chunks.is_empty())
        .map(|chunks| {
            chunks
                .into_iter()
                .map(|chunk| match chunk {
                    AuthorChunk::Name {
                        name,
                        is_site_author,
                    } => {
                        let class = if is_site_author { "is-self" } else { "" };
                        view! { <li class=class>{name}</li> }.into_any()
                    }
                    AuthorChunk::Gap => view! { <li>"…"</li> }.into_any(),
                    AuthorChunk::More(count) => {
                        view! { <li>{format!("+{count} more")}</li> }.into_any()
                    }
                })
                .collect::<Vec<_>>()
        });

    // Subjects, broadest first, de-duplicated across the three vocabularies.
    let tags = {
        let mut tags: Vec<String> = Vec::new();
        for source in [&metadata.keywords, &metadata.domains, &metadata.categories] {
            for tag in source.iter().flatten() {
                let tag = tag.trim().to_lowercase();
                if !tag.is_empty() && !tags.contains(&tag) {
                    tags.push(tag);
                }
            }
        }
        tags.truncate(MAX_TAGS);
        tags
    };

    view! {
        <article class="work-item">
            <div class="work-main">
                <h4 class="work-title">
                    {match primary_url {
                        Some(url) => {
                            view! {
                                <a href=url target="_blank" rel="noopener noreferrer">
                                    {title}
                                </a>
                            }
                                .into_any()
                        }
                        None => view! { <span>{title}</span> }.into_any(),
                    }}
                </h4>

                {venue
                    .map(|venue| {
                        view! {
                            <p class="work-venue">
                                <b>{venue}</b>
                                {locator.map(|locator| format!(" · {locator}"))}
                            </p>
                        }
                    })}

                {authors.map(|authors| view! { <ul class="work-authors">{authors}</ul> })}
                {description
                    .map(|description| view! { <p class="work-abstract">{description}</p> })}

                <div class="work-foot">
                    {kind.map(|kind| view! { <span class="chip">{kind}</span> })}
                    {tags
                        .into_iter()
                        .map(|tag| view! { <span class="chip is-topic">{tag}</span> })
                        .collect::<Vec<_>>()}
                </div>
            </div>

            // The margin: the ways out to the source, and the count set as one more
            // small fact rather than as a figure to be impressed by.
            <div class="work-margin">
                {citation_count
                    .map(|count| {
                        view! {
                            <p class="work-cites" title=citation_source.unwrap_or_default()>
                                <b>{count.to_string()}</b>
                                {if count == 1 { " citation" } else { " citations" }}
                            </p>
                        }
                    })}
                {doi
                    .map(|doi| {
                        view! {
                            <a
                                class="work-link"
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
                            <a class="work-link" href=url target="_blank" rel="noopener noreferrer">
                                "Open access ↗"
                            </a>
                        }
                    })}
            </div>
        </article>
    }
}
