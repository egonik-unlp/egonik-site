# UI requirements — things the new front end wants from the back end

The redesign is complete and ships against the API exactly as it exists today:
nothing below is required for the site to work. Each item is a place where the
UI is currently compensating for missing data, and what it would render if the
data arrived.

Ordered by how much they'd improve the page per unit of work.

---

## 0. How many projects render — RESOLVED

The archive now renders **100 projects**. Both filters that used to cut it are
gone.

| stage | count |
|---|---|
| repositories on the GitHub account | **103** |
| minus 3 forks (0 archived) | **100** |
| in `portfolio_items` after the re-sync | **100** ✓ |
| entries in the published `projects.json` | **104** |
| **rendered** = database ∩ `projects.json` | **100** |

What it took:

1. **The pagination loop** in `get_public_github_repos` (yours) — pages at
   `per_page=100` instead of taking GitHub's 30-item first page.
2. **An upsert in `create_article`** (added here). `portfolio_items.title` is
   `UNIQUE` and the insert had no `ON CONFLICT`, so the second run aborted on the
   first repository that already existed — which is why the pagination fix never
   reached the database. It now upserts on `title` and *replaces* the item's tags
   rather than appending, so the whole sync is idempotent and safe to schedule.
3. **Curation** stopped biting on its own when `projects.json` grew to 104
   entries covering the account.

Re-run it any time with:

```
cargo run --bin seed_publications --no-default-features --features ssr
```

**`tags` is still 0, and that is correct** — not a fourth bug. **Zero of the 103
repositories have GitHub topics set**, so there is nothing for the tag sync to
insert. Setting topics on the repositories you care about would populate it; the
UI already falls back to `metadata.languages`, so nothing is broken meanwhile.

`spotify-next-track` still has **no repository of that name**, so it can never
match and its live link can never render. Rename the key or drop it.

### The 82 generated entries are thinner than the 22 written-up ones

Field presence differs sharply, and the UI now depends on that difference:

| field | written-up (22) | generated (82) |
|---|---|---|
| `description` | 22/22, ~200 chars | 82/82, ~55 chars |
| `languages` | 22/22 | 73/82 |
| `categories` / `projectTypes` / `domains` | 22/22 | **1/82** |
| `maturity` / `highlights` | 22/15 | 1/82 |

Two consequences, both handled in the UI:

- **`categories` is the discriminator**, and it now does two jobs. Present on all
  22 written-up entries and none of the generated ones, `is_documented` uses it
  to decide (a) **what shows by default** — the 21 written-up projects, with the
  other 79 behind one disclosure control at the end of the list — and (b) **row
  density**, so a written-up row runs ~135px and a generated one ~49px. Nothing
  is hidden: the tail is one click away and the row is short because the record
  is short.

  This is why `domains` on the generated entries matters less than it looks:
  those entries are the tail, not the front page. What would change the front
  page is writing `categories`/`domains`/`highlights` for a repository you want
  promoted out of the tail — that is the whole editorial lever, and it lives in
  `projects.json` rather than in the code.
- **The axis classifier had to stop reading `domains`.** With `domains` on 1 of
  82, `glotaran_converter`, `glotaran_gui`, `glotaran_preprocessing`, `mof_xrd`,
  `QuenchingLFP`, `plotter_eem`, `_eem_converter`, `deconv_fit`,
  `docking_cecilia`, `cinetica-Julia`, `catedra`, `cecilia` and `rust_MD` were
  all landing on the general side. It now matches word stems across name, title,
  description, `domains` and `categories`, which recovers all thirteen.

**If you regenerate `projects.json` again**, the highest-value addition is
`domains` on the generated entries — it is the field the axis would rather read,
and it is derivable from the description you are already writing.

### How the pagination bug was diagnosed, for the record

- `portfolio_items` held **exactly 30 rows** — GitHub's default `per_page` for
  `GET /users/{username}/repos`.
- That endpoint defaults to sorting by `full_name` ascending. The alphabetically
  last row in the database was **`demoLogits`**, and **every missing repository
  sorted after it**. That is page 1 and nothing else.

Note what the cut removed: the missing entries were overwhelmingly **the
laboratory side** — both Glotaran converters, XRD matching, two
molecular-dynamics simulations, the pH calculator, the MOF analysis. The site was
showing the Spotify tools and web apps and hiding the science, which is precisely
why the research/software axis had nothing to stand on.

### Related: your local `projects.json` is out of sync with the published one

The service reads `https://egonik-unlp.github.io/assets/data/projects.json`, not
the file in the repo root. They currently disagree:

- **published, but not in the local file:** `convert-ffi`, `egonik-site`,
  `spotify-next-track` (22 entries)
- **local, but not published:** `convert-invert`, `rust_MD` (21 entries)

So even after a correct re-sync, `convert-invert` and `rust_MD` will still be
dropped by the metadata join until the published file is regenerated. Worth
reconciling in the same pass.

---

## 1. A bio / headline field on `personal_informations`

**Today.** The masthead headline, the lede and the four `readout` facts
("Based in", "Affiliation", "Mostly writing", "Field") are **hardcoded string
literals** in `src/personal_information/components.rs`. They are accurate, but
changing them means recompiling.

Meanwhile `PersonalInformationDto` exposes `birth_date` and `id`, neither of
which the design has any use for — a date of birth is not portfolio content.

**Wanted.** On `personal_informations`, nullable text columns:

| column | example | used by |
|---|---|---|
| `headline` | `Physical chemistry got me writing code. Now I do both.` | `.masthead-title` |
| `bio` | the two-sentence lede | `.masthead-lede` |
| `location` | `La Plata, Argentina` | readout |
| `affiliation` | `INIFTA · UNLP · CONICET` | readout |
| `field` | `Nanomaterials & photochemistry` | readout |

Surfaced through the existing `get_full_personal_info`. The components already
guard every optional field, so adding them is additive.

**Value.** The one thing on the page that cannot currently be edited without a
deploy becomes editable.

---

## 2. Expose the author block from `works.json`

**Today.** `PublicationMetadataTableDto` parses `author` — name, affiliations,
`googleScholarProfile`, `openAlexId` — and then
`PublicationService::get_publications_with_metadata` **throws it away**, keeping
only `works`.

The consequence: `SCHOLAR_URL` is a `const` in
`src/personal_information/components.rs`, hardcoded to
`https://scholar.google.com/citations?user=0CAay5kAAAAJ`. It is correct, and it
is a literal in a view file.

**Wanted.** Return the author block alongside the works — either a second
server function (`get_publication_author`) or a struct wrapping both. Then the
Scholar row in the contact list, and the affiliation in the masthead readout,
both come from data.

---

## 3. One bootstrap call instead of two identical ones

**Today.** `Masthead` and `Closing` each create their own `Resource` over
`get_full_personal_info()`. That is **two round trips for the same single-row
query** on every page load, because the two components sit at opposite ends of
the page and cannot share one `<Suspense>` boundary.

**Wanted.** Either

- a cheap server-side cache on `get_full_personal_info` (the row changes about
  never), or
- a `get_site_bootstrap()` returning personal info + contact + the publication
  author block in one call, provided into context by `App` and read by both.

**Value.** Removes a redundant database round trip from every request.

---

## 4. `job_experience` has no server function

**Today.** `src/job_experience/` has a model, a DTO and a repository. It has
**no `server.rs`, an empty `service.rs`, and an empty `components.rs`**. Nothing
reaches the UI, so the site has no CV / experience section at all.

Two blockers if you want one:

- `JobExperienceItem` and `JobInstitution` have **private fields and no
  accessors**, so even server-side code outside the module cannot read them.
- `accomplishments` and `responsabilities` are `VARCHAR(255)` — about two
  sentences. Also note the spelling (`responsabilities`); renaming it is a
  migration, and now is the cheap moment.

**Wanted, if a CV section is desired.** `get_all_job_experiences()` returning
items joined to their `job_institutions`, ordered by `date_from DESC`, with
`date_to: None` meaning current. The design has a natural slot: it would use the
same `.year-group` treatment as the publications catalogue.

---

## 5. Stable identifiers for list keys

**Today.** `PortfolioItemDto` drops `id` in the `From` conversion, so `title` is
the only unique key. It happens to be `UNIQUE` in Postgres, so it is safe — but
it means a project rename is indistinguishable from a delete plus an insert, and
the filter transition re-creates DOM nodes it could have kept.

**Wanted.** Keep `id` on `PortfolioItemDto` and `PublicationItemDto`.

---

## 6. `stargazers_count` and `language` are fetched and discarded

**Today.** `sync_from_github` deserialises `stargazers_count` and `language`
from the GitHub API and persists neither. `cited_by_count` is likewise
deserialised and dropped in the OpenAlex sync (citation counts come from the
separate metadata JSON instead).

**Wanted.** Persist `stargazers_count`. The `Built` section already has a data
margin pattern established in `Published` — a star count is the obvious thing to
put in it, and it is free at sync time.

---

## 7. Metadata joins silently drop unmatched rows

**Today.** Both `get_all_portfolio_items_with_metadata` and
`get_publications_with_metadata` use `filter_map`, so any database row without a
match in the curated JSON **vanishes with no signal**.

This is what turns the pagination bug in §0 into a silent one: a repository that
is missing from the database is indistinguishable, at this layer, from one that
was deliberately left out of the curation file. Nothing logs, nothing warns.

Right now the live database yields **8 projects and 6 publications** against a
published `projects.json` of 22 and a `works.json` of 7.

**Wanted.** Either log the unmatched titles server-side, or return them with
`ProjectMetadataDto::default()` so they still render (the portfolio component
already has exactly this fallback path for the whole-list failure case, and
`Entry::build` handles a fully-empty metadata record).

---

## 8. Panics in server functions

Not a design issue, but adjacent and cheap: `get_all_publications` and
`get_personal_info` call `.unwrap()` on the repository result. A database blip
takes down the worker instead of producing the `notice notice-error` state the
UI already renders for `Err`.

---

## Not requested

For the record, so nobody adds them speculatively:

- **Project screenshots / thumbnails.** The design is deliberately typographic
  and does not have a slot for them. Adding an image field would mean redesigning
  `Code`.
- **A `research: true` flag in `projects.json`.** The two tracks are derived in
  the UI from whether a project's `domains` contain a laboratory subject
  (`photochemistry`, `x-ray-diffraction`, `acid-base-equilibria`, …), matched on
  word stems in `RESEARCH_STEMS`. Deriving it means a new repository lands on the
  right side of the axis the moment it is tagged, with no second field to keep in
  sync. Note it is matched against `domains` and deliberately **not**
  `categories`: `scientific-programming` is applied broadly enough that Pathfinder,
  a Spotify route finder, carries it.
- **A tag/topic filter.** Topics are 1-to-many with a 40-value tail; they render
  as labels, not controls, on purpose. The axis is the only filter because it is
  the only split that partitions cleanly.

---

## 9. `links.demo` is parsed out of existence

**Today.** The published `projects.json` carries deployed URLs:

```json
"convert-ffi":        { "links": { "demo": "https://convert-ffi.onrender.com" } },
"spotify-next-track": { "links": { "demo": "https://infinite-playlist.eduardo-gonik.workers.dev/" } }
```

`ProjectMetadataDto` has no `links` field, and serde ignores unknown keys by
default, so the object is discarded silently before it reaches the UI. GitHub's
own `homepage` field is populated on only two repositories (`gv_xml_ui`,
`training-git`), neither of which is curated, so it is not a usable substitute.

The UI renders live links from a `LIVE_APPS` constant in
`src/portfolio/components.rs` as a stopgap. It has to be edited by hand whenever
`links.demo` changes, which is exactly the kind of duplication that goes stale.

**Wanted.** One additive field, no service or repository changes:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectLinksDto {
    pub demo: Option<String>,
    pub docs: Option<String>,
    pub site: Option<String>,
}

// in ProjectMetadataDto
pub links: Option<ProjectLinksDto>,
```

Then `Entry::build` reads `metadata.links.and_then(|l| l.demo)` and `LIVE_APPS`
is deleted. The row markup and styling stay exactly as they are.

**Two data corrections needed in `projects.json` itself**, both found by
requesting the URLs rather than trusting them:

1. **`convert-ffi`'s `links.demo` is wrong.** The published value
   `https://convert-ffi.onrender.com` returns **404**. The live deployment is
   `https://convert-ffi-latest.onrender.com/` (200, serves "convert-songs").
   `LIVE_APPS` uses the working URL; the JSON still has the broken one.
2. **`spotify-next-track` matches no repository.** The account has no repo of
   that name, so the entry is dropped by the metadata join and its row — and
   therefore its live link — can never render, even after the re-sync in §0. The
   app itself is up (`infinite-playlist.eduardo-gonik.workers.dev`, 200, "Infinite
   Playlist — a mood journey"), so this is worth fixing: rename the key to the
   real repository.

**Also worth doing:** only 2 of 22 curated entries carry a `links.demo`, yet
several others are plainly deployed web apps — Pathfinder, the LLM Sampling
Visualizer, the Calorimetry Curve Demo, Convert Invert Site. None of their READMEs
records a public URL (only `localhost`), so the addresses exist only in your head
or in a dashboard. Adding them to `projects.json` is the highest-value content
edit available: each one turns a row from something to read about into something
to open.
