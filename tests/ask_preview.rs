//! Writes a standalone HTML preview of `<Ask/>` for visual review.
//!
//! Ignored by default: it is a development aid, not an assertion. Run with
//! `cargo test --features ssr --test ask_preview -- --ignored --nocapture`.
#![cfg(feature = "ssr")]

use egonik_site::ask::components::Ask;
use leptos::prelude::*;

#[test]
#[ignore]
fn write_preview() {
    let owner = Owner::new();
    owner.set();
    let body = view! { <Ask /> }.to_html();
    drop(owner);

    let css = std::fs::read_to_string("target/site/pkg/egonik-site.css")
        .expect("run `cargo leptos build` first");
    let out = std::env::var("PREVIEW_OUT").unwrap_or_else(|_| "/tmp/ask-preview.html".into());
    let html = format!(
        "<!doctype html><html lang=\"en\"><head><meta charset=\"utf-8\">\
         <meta name=\"viewport\" content=\"width=device-width,initial-scale=1\">\
         <title>Ask preview</title><style>{css}</style></head>\
         <body><main>{body}</main></body></html>"
    );
    std::fs::write(&out, html).expect("write preview");
    println!("wrote {out}");
}
