//! The query tool that replaces the Code, Papers and Tools sections.
//!
//! Not a search box above a page of results — a tool with three states:
//!
//! 1. **Idle.** The prompt, and nothing else. No archive dump, no grid.
//! 2. **Results.** Matches appear as a compact list as you type, each naming the
//!    section it lives in. Still no cards.
//! 3. **In context.** Clicking a result opens that item's section, renders the
//!    matches inside it as cards, highlights the one chosen, and scrolls to it.
//!
//! The result list stays visible in state 3, so trying a different match is one
//! click rather than a re-query. That is the difference between a search box and a
//! tool: the query stays live while you look around.
//!
//! ## Why every card is still in the DOM
//!
//! Hidden, not absent. Three reasons, in the order they were learned:
//!
//! * A node that appears from nothing has no state to transition *from*, so
//!   collapsing with a class is what makes the reveal animatable at all.
//! * The archive stays in the server-rendered HTML, so crawlers and no-JS visitors
//!   still get all 53 items even though a visitor sees none at first. (Caveat:
//!   content behind `display: none` is weighted lower than visible content — a
//!   deliberate trade for the interaction.)
//! * `#code`, `#papers` and `#tools` are what the nav rail links to. CSS `:target`
//!   reveals a group when its anchor is the fragment, so the rail keeps working
//!   with no JavaScript at all — see `.ask-group:target` in the stylesheet.
//!
//! Everything renders from the router index compiled into the binary: no server
//! function, no `Resource`, no `Suspense`, and no request of any kind. Query and
//! route are logged to the browser console only.

use leptos::ev;
use leptos::prelude::*;
use pb_router::{IndexItem, Route, Router};

/// Sections this component owns. `bio` and `contact` stay with `<Masthead/>` and
/// `<Closing/>`, which already own `#top` and `#contact`.
const GROUPS: [&str; 3] = ["code", "papers", "tools"];

/// Below this many characters, routing is not attempted. One or two letters match
/// almost anything through the student's stem fallback, and a list that thrashes on
/// every keystroke reads as broken rather than responsive.
const MIN_QUERY_LEN: usize = 2;

/// One row in the result list.
#[derive(Clone, PartialEq)]
struct ResultRow {
    item_id: String,
    dom_id: String,
    title: String,
    section: String,
    section_name: String,
}

/// What the current query found, and what the visitor has opened.
#[derive(Clone, PartialEq, Default)]
struct State {
    /// Matches, best first. Empty means nothing to show — including at rest.
    results: Vec<ResultRow>,
    /// The item the visitor clicked. `None` keeps every card collapsed.
    opened: Option<String>,
    /// What the router decided, announced politely.
    readout: String,
    /// The router declined; the query is probably not about him.
    abstained: bool,
    /// A query has been entered, as opposed to the initial rest state. Gates the
    /// entry animation so server-rendered markup does not animate on load.
    active: bool,
}

impl State {
    /// Which section, if any, is currently open.
    fn open_section(&self) -> Option<&str> {
        let opened = self.opened.as_deref()?;
        self.results
            .iter()
            .find(|r| r.item_id == opened)
            .map(|r| r.section.as_str())
    }

    /// A card shows only once something is opened, and only within that section.
    fn shows(&self, item_id: &str, section: &str) -> bool {
        self.open_section() == Some(section)
            && self.results.iter().any(|r| r.item_id == item_id)
    }

    fn is_open(&self, item_id: &str) -> bool {
        self.opened.as_deref() == Some(item_id)
    }
}

fn row(router: &'static Router, item_id: &str) -> Option<ResultRow> {
    let item = router.item(item_id)?;
    if !GROUPS.contains(&item.section.as_str()) {
        return None;
    }
    Some(ResultRow {
        item_id: item.id.clone(),
        dom_id: item.dom_id.clone(),
        title: item.title.clone(),
        section: item.section.clone(),
        section_name: section_name(router, &item.section),
    })
}

fn rows(router: &'static Router, ids: impl IntoIterator<Item = String>) -> Vec<ResultRow> {
    let mut out: Vec<ResultRow> = Vec::new();
    for id in ids {
        if let Some(r) = row(router, &id) {
            if !out.iter().any(|e| e.item_id == r.item_id) {
                out.push(r);
            }
        }
    }
    out
}

fn from_route(router: &'static Router, route: &Route) -> State {
    let mut state = State {
        active: true,
        ..Default::default()
    };
    match route {
        Route::Item {
            item_id, hits, why, ..
        } => {
            // Winner first, then the other close hits, so a near-miss is one click
            // away instead of needing a rephrase.
            let mut ids = vec![item_id.clone()];
            ids.extend(hits.iter().map(|h| h.item_id.clone()));
            state.results = rows(router, ids);
            state.readout = why.clone();
        }
        Route::Facet {
            item_ids,
            field,
            value,
            ..
        } => {
            state.results = rows(router, item_ids.clone());
            state.readout = if field == "languages" {
                format!("{} projects in {value}", state.results.len())
            } else {
                format!("{} items tagged {value}", state.results.len())
            };
        }
        Route::Multi { hits, why, .. } => {
            state.results = rows(router, hits.iter().map(|h| h.item_id.clone()));
            state.readout = why.clone();
        }
        Route::Section { section, .. } => {
            state.results = rows(
                router,
                router.items_in(section).iter().map(|i| i.id.clone()),
            );
            state.readout = format!(
                "{} in {}",
                state.results.len(),
                section_name(router, section)
            );
        }
        Route::Abstain { why, deferred } => {
            // Two different failures. `deferred` means the lexical layers had no grip
            // and the dense stage could not help either — the answer may well be here
            // and the router missed it. Saying "nothing answers that" would be false.
            state.readout = if *deferred {
                "No match — try a name, a language, or a technique.".into()
            } else {
                format!("Nothing here answers that ({why}).")
            };
            state.abstained = true;
        }
    }
    // A route naming only items outside this component's sections (bio or contact)
    // leaves nothing to list; say where to look rather than showing an empty panel.
    if state.results.is_empty() && !state.abstained {
        state.readout = format!("{} — see the top and bottom of the page", state.readout);
    }
    state
}

fn section_name(router: &'static Router, section: &str) -> String {
    router
        .section_order()
        .into_iter()
        .find(|(id, _, _)| *id == section)
        .map(|(_, name, _)| name.to_owned())
        .unwrap_or_else(|| section.to_owned())
}

/// Logs the query and the route to the browser console.
///
/// Console only — nothing is sent anywhere, which is what keeps the lede's claim
/// about the browser true. `via` is the field worth watching: it says whether the
/// lexical layers answered or the 8MB student had to, which is the entire
/// cost-justification for shipping the student.
#[cfg(feature = "hydrate")]
fn log_route(query: &str, route: &Route) {
    use leptos::wasm_bindgen::JsValue;

    let (target, via) = match route {
        Route::Item { item_id, via, .. } => (item_id.clone(), format!("{via:?}").to_lowercase()),
        Route::Facet { field, value, .. } => (format!("{field}={value}"), "facet".into()),
        Route::Multi { section, .. } => (format!("multi:{section}"), "lexical".into()),
        Route::Section { section, .. } => (section.clone(), "intent".into()),
        Route::Abstain { deferred, .. } => (
            "—".into(),
            if *deferred { "deferred" } else { "abstain" }.to_string(),
        ),
    };
    let hits = if route.hits().is_empty() {
        "—".to_string()
    } else {
        route
            .hits()
            .iter()
            .take(5)
            .map(|h| format!("{} ({:.2}, cov {:.0}%)", h.item_id, h.score, h.coverage * 100.0))
            .collect::<Vec<_>>()
            .join(", ")
    };
    web_sys::console::log_1(&JsValue::from_str(&format!(
        "[router] in: {query:?}\n         out: {} via {via} -> {target}\n         why: {}\n         hits: {hits}",
        route.kind(),
        route.why(),
    )));
}

#[cfg(not(feature = "hydrate"))]
fn log_route(_query: &str, _route: &Route) {}

/// Moves the viewport and keyboard focus to an element.
///
/// Scrolling alone leaves focus in the input, so a keyboard or screen-reader user is
/// told nothing happened. Targets carry `tabindex="-1"` so they can take focus
/// without becoming tab stops.
#[cfg(feature = "hydrate")]
fn reveal(target_id: &str) {
    use leptos::wasm_bindgen::JsCast;

    let Some(document) = leptos::prelude::document()
        .dyn_ref::<web_sys::Document>()
        .cloned()
    else {
        return;
    };
    if let Some(element) = document.get_element_by_id(target_id) {
        let options = web_sys::ScrollIntoViewOptions::new();
        options.set_behavior(web_sys::ScrollBehavior::Smooth);
        // Centre rather than top-align: the sticky rail would cover an element it had
        // just scrolled to.
        options.set_block(web_sys::ScrollLogicalPosition::Center);
        element.scroll_into_view_with_scroll_into_view_options(&options);
        if let Some(html) = element.dyn_ref::<web_sys::HtmlElement>() {
            let _ = html.focus();
        }
    }
}

#[cfg(not(feature = "hydrate"))]
fn reveal(_target_id: &str) {}

/// Where the distilled table is served from. Synced to the site root from
/// `assets/` by cargo-leptos.
const STUDENT_URL: &str = "/student.bin";

/// Whether to fetch the distilled dense stage at all. **Off, on measurement.**
///
/// Operated at a confidence floor strict enough to add no out-of-scope leak of its
/// own, the table gains 2 points overall (48% -> 50%) for 12MB — it falls through 5
/// times in 250 queries. At the looser floor where it gained 9 points it let 50% of
/// out-of-scope queries through, undoing the lexical layers' refusals. Its wrong
/// answers score nearly as high as its right ones, and that overlap is what makes it
/// unusable, not its accuracy. See project-black/README.md.
///
/// A runtime const rather than a cargo feature so the whole path stays compiled and
/// type-checked; re-enabling is one line once a better-separated stage exists. Note
/// that `assets/student.bin` is gitignored, so a deployment from git has no table to
/// fetch even if this is flipped — regenerate and copy it first.
const FETCH_STUDENT: bool = false;

/// Fetches `student.bin` and installs it, then re-runs the query that needed it.
///
/// Lazy on purpose. The table is ~8MB gzipped, it is consulted on roughly half of
/// queries and on none at page load, so embedding it would make every visitor pay
/// for it before first paint. This way first paint stays at ~1.9MB and the download
/// happens only when someone asks something the lexical layers cannot answer.
///
/// Failure is silent by design: the router keeps working without the student, just
/// less well on paraphrase. A visitor who cannot reach the asset should still get
/// `Zr` and `Rust` answered.
#[cfg(feature = "hydrate")]
fn load_student(loading: RwSignal<bool>, ready: RwSignal<bool>) {
    use leptos::wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;

    if pb_router::static_dense::is_installed() {
        return;
    }
    loading.set(true);
    leptos::task::spawn_local(async move {
        // Every early return clears the flag: a failed fetch must not leave the
        // panel saying "loading" forever.
        let done = move || loading.set(false);

        let Some(window) = web_sys::window() else {
            done();
            return;
        };
        let response: leptos::wasm_bindgen::JsValue =
            match JsFuture::from(window.fetch_with_str(STUDENT_URL)).await {
                Ok(value) => value,
                Err(_) => {
                    web_sys::console::warn_1(
                        &format!("[router] could not fetch {STUDENT_URL}").into(),
                    );
                    done();
                    return;
                }
            };
        let Ok(response) = response.dyn_into::<web_sys::Response>() else {
            done();
            return;
        };
        if !response.ok() {
            web_sys::console::warn_1(
                &format!("[router] {STUDENT_URL}: HTTP {}", response.status()).into(),
            );
            done();
            return;
        }
        let Ok(promise) = response.array_buffer() else {
            done();
            return;
        };
        let Ok(buffer) = JsFuture::from(promise).await else {
            done();
            return;
        };
        let bytes = js_sys::Uint8Array::new(&buffer).to_vec();
        let len = bytes.len();
        done();
        if pb_router::static_dense::install(bytes) {
            web_sys::console::log_1(
                &format!("[router] student installed ({len} bytes)").into(),
            );
            // Flipping this re-runs the query that triggered the load, via an effect
            // in the component. Signalling rather than calling back avoids a
            // self-referential closure.
            ready.set(true);
        } else {
            web_sys::console::warn_1(&"[router] student.bin rejected".into());
        }
    });
}

#[cfg(not(feature = "hydrate"))]
fn load_student(_loading: RwSignal<bool>, _ready: RwSignal<bool>) {}

#[component]
pub fn Ask() -> impl IntoView {
    // A parse failure means the compiled-in index is malformed — a build problem, but
    // it must not take the page down, so the section simply does not render.
    let Ok(router) = pb_router::shared() else {
        return view! { <></> }.into_any();
    };

    let query = RwSignal::new(String::new());
    let state = RwSignal::new(State::default());

    // Routed on input rather than on submit: scoring 55 items against a lookup table
    // is microseconds, and results that follow the typing are what makes this feel
    // like a tool instead of a form.
    // Set while the table is downloading, so the panel can say so rather than showing
    // a bare "no match" that is about to become a match.
    let loading = RwSignal::new(false);
    // Flipped once the table is installed; the effect below re-runs the query.
    let student_ready = RwSignal::new(false);
    let last_query = RwSignal::new(String::new());

    let run = move |text: String| {
        last_query.set(text.clone());
        if text.trim().chars().count() < MIN_QUERY_LEN {
            state.set(State::default());
            return;
        }
        // Full cascade: lexical first, the student only where lexical has no grip.
        // Before the fetch lands this is simply the lexical router.
        let route = pb_router::static_dense::route(router, &text);
        log_route(&text, &route);

        // A deferred abstain is exactly the case the student exists for, so this is
        // where the download is worth triggering — and nowhere else. Every other
        // query is answered without it, which is the point of loading lazily.
        if FETCH_STUDENT
            && matches!(route, Route::Abstain { deferred: true, .. })
            && !pb_router::static_dense::is_installed()
        {
            load_student(loading, student_ready);
        }
        state.set(from_route(router, &route));
    };

    // Re-runs the query that triggered the download, so the visitor gets the better
    // answer without retyping. Guarded by `is_installed`, so this cannot loop.
    Effect::new(move |_| {
        if student_ready.get() {
            let q = last_query.get_untracked();
            if !q.trim().is_empty() {
                run(q);
            }
        }
    });

    let clear = move |_| {
        query.set(String::new());
        state.set(State::default());
    };

    view! {
        <section id="ask" class="section ask" class:is-active=move || state.get().active>
            <div class="container">
                <div class="section-head">
                    <h2 class="section-title">"What do you want to know about me?"</h2>
                </div>
                <p class="section-lede">
                    "Ask in English or Spanish — a name, a technique, an element symbol. "
                    <span class="u-data">"Zr"</span>", "<span class="u-data">"Rust"</span>", "
                    <span class="u-data">"nanopartículas de circonio"</span>
                    ". Everything runs in your browser; nothing is sent anywhere."
                </p>

                <form
                    class="ask-form"
                    role="search"
                    on:submit=move |ev: ev::SubmitEvent| {
                        ev.prevent_default();
                        run(query.get());
                    }
                >
                    <label class="ask-label" for="ask-input">"Your question"</label>
                    <input
                        id="ask-input"
                        class="ask-input"
                        type="search"
                        autocomplete="off"
                        aria-controls="ask-results"
                        placeholder="Zr, Rust, ¿qué publicó?…"
                        prop:value=move || query.get()
                        on:input=move |ev| {
                            let text = event_target_value(&ev);
                            query.set(text.clone());
                            run(text);
                        }
                    />
                    <button
                        class="ask-clear"
                        type="button"
                        on:click=clear
                        class:is-shown=move || !query.get().is_empty()
                    >
                        "Clear"
                    </button>
                </form>

                // State 2: the result list. Empty and silent at rest.
                <div
                    id="ask-results"
                    class="ask-panel"
                    aria-live="polite"
                    class:is-shown=move || state.get().active
                >
                    <p
                        class="ask-readout"
                        class:is-abstain=move || state.get().abstained
                        class:is-loading=move || loading.get()
                    >
                        {move || {
                            if loading.get() {
                                "Loading the language model…".to_string()
                            } else {
                                state.get().readout
                            }
                        }}
                    </p>
                    <ul class="ask-list">
                        {move || {
                            state
                                .get()
                                .results
                                .into_iter()
                                .enumerate()
                                .map(|(i, r)| {
                                    let target = r.clone();
                                    let opened = r.item_id.clone();
                                    view! {
                                        <li class="ask-list-item" style=("--i", i.to_string())>
                                            <button
                                                class="ask-hit"
                                                type="button"
                                                aria-controls=r.dom_id.clone()
                                                class:is-open=move || state.get().is_open(&opened)
                                                on:click=move |_| {
                                                    let t = target.clone();
                                                    state.update(|s| s.opened = Some(t.item_id.clone()));
                                                    reveal(&t.dom_id);
                                                }
                                            >
                                                <span class="ask-hit-title">{r.title}</span>
                                                <span class="ask-hit-where">{r.section_name}</span>
                                            </button>
                                        </li>
                                    }
                                })
                                .collect_view()
                        }}
                    </ul>
                </div>

                // State 3: the item in context. Hidden until a result is clicked — and
                // revealed by CSS `:target` alone when the nav rail links here, which is
                // how the anchors survive with no JavaScript.
                {GROUPS
                    .iter()
                    .map(|section| {
                        let section = *section;
                        let items = router.items_in(section);
                        let name = section_name(router, section);
                        view! {
                            <div
                                class="ask-group"
                                id=section
                                tabindex="-1"
                                class:is-open=move || {
                                    state.get().open_section() == Some(section)
                                }
                            >
                                <h3 class="ask-group-name">{name}</h3>
                                <ul class="ask-grid">
                                    {items
                                        .into_iter()
                                        .enumerate()
                                        .map(|(i, item)| card(item, section, i, state))
                                        .collect_view()}
                                </ul>
                            </div>
                        }
                    })
                    .collect_view()}
            </div>
        </section>
    }
        .into_any()
}

/// One item, rendered in context.
fn card(
    item: &IndexItem,
    section: &'static str,
    position: usize,
    state: RwSignal<State>,
) -> impl IntoView {
    let id = item.id.clone();
    let open_id = item.id.clone();
    let title = item.title.clone();
    let blurb = item.blurb.clone();
    let badges = item.badges.clone();
    let has_badges = !badges.is_empty();
    let has_blurb = !blurb.is_empty();
    let external = item.external.clone();

    view! {
        <li
            class="ask-card"
            id=item.dom_id.clone()
            tabindex="-1"
            style=("--i", position.to_string())
            class:is-out=move || !state.get().shows(&id, section)
            class:is-hit=move || state.get().is_open(&open_id)
        >
            <h4 class="ask-card-title">{title}</h4>
            <Show when=move || has_blurb>
                <p class="ask-card-blurb">{blurb.clone()}</p>
            </Show>
            <Show when=move || has_badges>
                <ul class="ask-badges">
                    {badges
                        .iter()
                        .map(|b| view! { <li class="chip">{b.clone()}</li> })
                        .collect_view()}
                </ul>
            </Show>
            {external
                .map(|url| {
                    view! {
                        <a
                            class="work-link ask-card-out"
                            href=url
                            target="_blank"
                            rel="noopener noreferrer"
                        >
                            "Open source →"
                        </a>
                    }
                })}
        </li>
    }
}
