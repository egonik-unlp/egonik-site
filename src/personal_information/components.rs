use leptos::prelude::*;

use crate::personal_information::dto::{ContactInformationDto, PersonalInformationDto};
use crate::personal_information::server::get_full_personal_info;

/// Google Scholar is the one profile the site links that the database does not
/// store. It is published in the works metadata (`author.googleScholarProfile`)
/// but the service drops that half of the payload, so it is a constant here.
/// See UI_REQUIREMENTS.md — surfacing it properly is a server-side change.
const SCHOLAR_URL: &str = "https://scholar.google.com/citations?user=0CAay5kAAAAJ";

/// The contact rows are handles in the database, not URLs. Every conversion
/// lives here so nothing downstream has to know the shape of a profile link.
fn profile_urls(contact: &ContactInformationDto) -> Vec<(&'static str, String, String)> {
    let handle = |value: &str| value.trim().trim_start_matches('@').to_owned();

    let mut rows = Vec::new();
    let email = contact.email.trim();
    if !email.is_empty() {
        rows.push(("Email", email.to_owned(), format!("mailto:{email}")));
    }
    let github = handle(&contact.github);
    if !github.is_empty() {
        rows.push((
            "GitHub",
            format!("github.com/{github}"),
            format!("https://github.com/{github}"),
        ));
    }
    rows.push((
        "Scholar",
        "Google Scholar".to_owned(),
        SCHOLAR_URL.to_owned(),
    ));
    let linked_in = handle(&contact.linked_in);
    if !linked_in.is_empty() {
        rows.push((
            "LinkedIn",
            format!("linkedin.com/in/{linked_in}"),
            // The previous build concatenated a bare `.../in/` prefix with nothing
            // and shipped a link to LinkedIn's 404 page.
            format!("https://www.linkedin.com/in/{linked_in}"),
        ));
    }
    rows
}

/// The two-tone conversion for the portrait.
///
/// Luminance first, then a two-stop transfer per channel: shadows land on the
/// deep slab colour, highlights on paper. It is the only photograph in the
/// build, and untreated it would be the one element not speaking the palette.
/// Values are `--color-deep-2` and `--color-paper` in 0..1 sRGB.
#[component]
fn DuotoneFilter() -> impl IntoView {
    view! {
        <svg
            class="overflow-hidden absolute w-0 h-0"
            aria-hidden="true"
            focusable="false"
            width="0"
            height="0"
        >
            <filter id="duotone" color-interpolation-filters="sRGB">
                <feColorMatrix
                    type="matrix"
                    values="0.2126 0.7152 0.0722 0 0
                    0.2126 0.7152 0.0722 0 0
                    0.2126 0.7152 0.0722 0 0
                    0 0 0 1 0"
                />
                <feComponentTransfer>
                    <feFuncR type="table" tableValues="0 0.922" />
                    <feFuncG type="table" tableValues="0.075 0.961" />
                    <feFuncB type="table" tableValues="0.051 0.937" />
                </feComponentTransfer>
            </filter>
        </svg>
    }
}

/// The opening slab.
///
/// The headline, lede and readout are static and render immediately; only the
/// name label, portrait and mail link wait on the database. That split means a
/// slow or failed query costs a photograph, never an empty first screen.
#[component]
pub fn Masthead() -> impl IntoView {
    let resource = Resource::new(|| (), |_| async move { get_full_personal_info().await });

    view! {
        <section id="top" class="masthead">
            <DuotoneFilter />
            <div class="container masthead-grid">
                <div>
                    <p class="masthead-name lift-1">
                        <Suspense fallback=|| ()>
                            {move || {
                                resource
                                    .get()
                                    .and_then(Result::ok)
                                    .map(|(personal, _)| {
                                        format!("{} {}", personal.name, personal.surname)
                                    })
                            }}
                        </Suspense>
                    </p>
                    <h1 class="masthead-title lift-2">
                        "Chemistry research, " <em>"and the software it kept needing."</em>
                    </h1>
                    <p class="masthead-lede lift-3">
                        "I work on nanomaterials and photochemistry at INIFTA (UNLP–CONICET). A fair
                         amount of that needs software that doesn't quite exist yet — an instrument
                         format nobody documents, a method from a paper with no implementation — so
                         I end up writing it. Usually in " <strong>"Rust"</strong>
                        ", sometimes Julia or Python."
                    </p>
                    <div class="masthead-actions lift-4">
                        <a class="btn btn-solid" href="#code">
                            "See the code"
                        </a>
                        <a class="btn" href="#papers">
                            "See the papers"
                        </a>
                    </div>
                </div>

                <figure class="m-0 masthead-figure draw-in">
                    <Suspense fallback=|| ()>
                        {move || {
                            resource
                                .get()
                                .and_then(Result::ok)
                                .map(|(personal, _)| {
                                    let alt = format!("{} {}", personal.name, personal.surname);
                                    view! {
                                        <span class="portrait">
                                            <img
                                                src=personal.image_url
                                                alt=alt
                                                width="460"
                                                height="460"
                                                decoding="async"
                                            />
                                        </span>
                                    }
                                })
                        }}
                    </Suspense>
                </figure>
            </div>

            <div class="container">
                // <div> wrappers, not <li>: a <dl> may only contain dt/dd pairs
                // (optionally grouped in divs), and a list item inside one is
                // dropped from the accessibility tree.
                <dl class="readout lift-4">
                    <div>
                        <dt>"Based in"</dt>
                        <dd>"La Plata, Argentina"</dd>
                    </div>
                    <div>
                        <dt>"Affiliation"</dt>
                        <dd>"INIFTA · UNLP · CONICET"</dd>
                    </div>
                    <div>
                        <dt>"Research"</dt>
                        <dd>"Nanomaterials, photochemistry"</dd>
                    </div>
                    <div>
                        <dt>"Mostly writing"</dt>
                        <dd>"Rust, Julia, Python"</dd>
                    </div>
                </dl>
            </div>
        </section>
    }
}

/// The closing slab: the page ends in deep green, its one saturated surface.
#[component]
pub fn Closing() -> impl IntoView {
    let resource = Resource::new(|| (), |_| async move { get_full_personal_info().await });

    view! {
        <section id="contact" class="closing">
            <div class="container">
                <div class="section-head">
                    <h2 class="section-title">"Contact"</h2>
                </div>
                <p class="section-lede">
                    "Glad to hear about research collaborations or tooling work — particularly
                     anything involving an instrument that writes a format nobody has documented.
                     Email is the surest way to reach me."
                </p>

                // Read inside <Suspense/>: read outside it, the server and the hydrating
                // client disagree about whether the value has landed, and tachys aborts
                // hydration on the mismatched tree rather than recovering.
                <Suspense fallback=|| {
                    view! {
                        <div class="skeleton" role="status" aria-label="Loading contact details">
                            <span aria-hidden="true"></span>
                        </div>
                    }
                }>
                    {move || match resource.get() {
                        None => None,
                        Some(Err(error)) => {
                            Some(
                                view! {
                                    <p class="notice notice-error">
                                        {format!("Contact details could not be loaded: {error}")}
                                    </p>
                                }
                                    .into_any(),
                            )
                        }
                        Some(Ok((_, contact))) => {
                            let rows = profile_urls(&contact)
                                .into_iter()
                                .map(|(key, label, href)| {
                                    view! {
                                        <li>
                                            <a href=href target="_blank" rel="noopener noreferrer me">
                                                <span class="contact-key">{key}</span>
                                                <span class="contact-val">{label}</span>
                                                <span class="contact-arr" aria-hidden="true">
                                                    "↗"
                                                </span>
                                            </a>
                                        </li>
                                    }
                                })
                                .collect::<Vec<_>>();
                            Some(view! { <ul class="contact-list">{rows}</ul> }.into_any())
                        }
                    }}
                </Suspense>
            </div>
        </section>
    }
}

/// Compact identity block, kept for callers outside the landing page.
#[component]
pub fn WhoAmI(personal_information: PersonalInformationDto) -> impl IntoView {
    let PersonalInformationDto {
        name,
        surname,
        image_url,
        ..
    } = personal_information;
    let alt = format!("{name} {surname}");

    view! {
        <div class="flex gap-4 items-center">
            <DuotoneFilter />
            <span class="w-20 portrait shrink-0">
                <img src=image_url alt=alt width="80" height="80" decoding="async" />
            </span>
            <p class="text-lg font-semibold">{format!("{name} {surname}")}</p>
        </div>
    }
}
