# `ui/` — subject-agnostic presentation

Chrome that belongs to the site as a whole: navigation, layout, the route table.

**Nothing here knows about a content type.** Components that render publications,
portfolio items, experience or profile data live in their own slice
(`publications/components.rs`, `portfolio/components.rs`, …). If a component names a
DTO, it belongs in the slice, not here.

## Files

| File | Holds |
|---|---|
| `layout.rs` | `NavBar`, `Footer`, the `<main>` container |
| `router.rs` | `AppRouter`: the `<Router>` + `<Routes>` tree |
| `paths.rs`  | URL string constants for `<A href>` and the sitemap |

`paths.rs`, not `routes.rs` — it sits next to `router.rs` and one-letter-apart filenames
get opened by mistake. It holds URL strings, not route definitions.

## Rules

**This module is ungated.** `lib.rs` declares `pub mod ui;` with no `#[cfg]`, and every
file here compiles into the wasm bundle as well as the server.

**Routing does not go in `entrypoint/`.** That directory is the *server* process edge and
is `ssr`-only. Leptos page routing runs in the browser. Putting a route or the navbar
there produces `error[E0433]: … could not find … in the crate root`, with the note
`configured out … gated behind the 'ssr' feature` — and the tempting fix (un-gating
`entrypoint`) drags Actix toward the client. `entrypoint/routes/` is for machine-facing
HTTP endpoints only: `/healthz`, `sitemap.xml`, webhooks.

**`<Router>` must wrap the navbar.** `<A>` needs router context for its `href`
resolution and active state, so the nesting order is fixed:

```
AppRouter
 └ <Router>
     ├ <NavBar/>
     └ <main> └ <Routes>
```

**Name the exported component `AppRouter`, not `Router`** — `leptos_router::components::Router`
is already in scope.

## `<A>` gotchas

Checked against `leptos_router 0.8.15`. Its only props are `href`, `target`, `exact`,
`strict_trailing_slash`, `scroll`, `children`.

- **No `class` prop.** Use the macro's attribute spreading: `attr:class="…"`.
- **No `active_class` prop.** `<A>` sets `aria-current="page"` on itself when active.
  Style it with Tailwind's arbitrary variant: `aria-[current=page]:font-semibold`.
- **`exact=true` is required on the home link.** Without it, a link is active when the
  current route *starts with* its href — so `href="/"` would be active on every page.

## Styling

Reusable widget styles (`.btn`, `.card`, `.data-table`, `.muted`, `.notice`) are defined
under `@layer components` in `style/tailwind.css`, not as long class strings in `view!`
macros. Colours come from the `@theme` tokens there (`--color-ink`, `--color-muted`,
`--color-line`, `--color-paper`, `--color-surface`, `--color-accent`) — no raw hex in
components.

Tailwind detects classes by scanning `.rs` files, so class names must appear as literal
text. `format!("text-{colour}-600")` produces no CSS.
