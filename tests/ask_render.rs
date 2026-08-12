//! Renders `<Ask/>` to HTML without a database.
//!
//! The component this replaced needed a server function, a connection pool, and a
//! suspense boundary; this one reads the compiled-in router index, so it renders
//! synchronously and can be asserted on directly. Two things that matter and are
//! easy to lose silently:
//!
//! * the archive is in the **server-rendered** HTML, so crawlers and no-JS
//!   visitors still get the content;
//! * the `#code`, `#papers` and `#tools` anchors the nav rail links to survived
//!   the replacement.

#![cfg(feature = "ssr")]

use egonik_site::ask::components::Ask;
use leptos::prelude::*;

fn render() -> String {
    let runtime = Owner::new();
    runtime.set();
    let html = view! { <Ask /> }.to_html();
    drop(runtime);
    html
}

#[test]
fn renders_every_group_anchor() {
    let html = render();
    // The rail links to these three (src/ui/components/nav.rs). Replacing the
    // sections must not break them.
    for anchor in ["id=\"code\"", "id=\"papers\"", "id=\"tools\""] {
        assert!(html.contains(anchor), "missing group anchor {anchor}");
    }
}

#[test]
fn archive_is_server_rendered() {
    let html = render();
    // A project, a paper, and a toolkit entry — content, not just scaffolding.
    assert!(html.contains("Convert Invert"), "project title missing");
    assert!(html.contains("Zr(IV)"), "paper title missing");
    assert!(html.contains("Qdrant"), "toolkit entry missing");
}

#[test]
fn items_carry_scroll_targets() {
    let html = render();
    // The per-item DOM ids that let an `item` route scroll to one card. These did
    // not exist before: the site had section anchors only.
    assert!(html.contains("id=\"r-convert-invert\""), "project dom_id missing");
    assert!(
        html.contains("id=\"r-doi-10-1039-d5ra09148a\""),
        "paper dom_id missing"
    );
}

#[test]
fn prompt_is_labelled() {
    let html = render();
    assert!(html.contains("id=\"ask-input\""), "input missing");
    assert!(
        html.contains("for=\"ask-input\""),
        "input has no associated label"
    );
    assert!(html.contains("aria-live"), "outcome is not announced");
}

#[test]
fn router_agrees_with_the_rendered_ids() {
    // Guards the seam between the index's `dom_id` and the markup: if the slug
    // rule in export_index.py changes, this fails instead of the scroll silently
    // doing nothing in the browser.
    let router = pb_router::shared().expect("index parses");
    let html = render();
    for item in router.items() {
        if matches!(item.section.as_str(), "code" | "papers" | "tools") {
            assert!(
                html.contains(&format!("id=\"{}\"", item.dom_id)),
                "item {} rendered no element with id {}",
                item.id,
                item.dom_id
            );
        }
    }
}
