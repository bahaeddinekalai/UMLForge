# Build plan: from empty repo to parity

The order in which to build UMLForge. `docs/parity-checklist.md` says **what** is in scope; this file says **when**. Every checklist row appears in exactly one phase below (`cargo xtask parity` checks this: no row missing, none listed twice). If you add a row to the checklist, add it here in the same PR.

Effort is relative (S small, M medium, L large, XL very large), not a date. Inside a phase, rows are independent branches (one feature, one branch) and can run in parallel once the phase's **steps** are done.

## How the order was chosen

1. **Dependencies first.** Ids, the model/view split, commands, undo/redo and the file format are touched by everything. Getting them wrong costs a rewrite, so they come before any screen.
2. **Prove the pipeline early.** CI, the web deploy and the Windows installer work in phase 0 with an empty app, so every later phase ships continuously.
3. **Headless before graphical.** The engine, SVG renderer and CLI (phase 2) let agents and tests drive and verify everything before the UI exists. The UI is then a thin layer (AGENTS.md rule 8).
4. **Riskiest things early.** Canvas performance with 2,000 views, undo/redo across model and view, and the file format are the three ways this project can fail. Each has an early spike or a dedicated phase.
5. **Vertical slices.** Each phase ends with something a person can use and a tagged release, not a pile of unconnected parts.
6. **Most-used first.** Class diagrams, then the other core diagram types, then the long tail.
7. **Least-used and most-dependent last.** Plugins need a stable model API; reverse engineering and XMI are large and rarely used; vendor-style shape libraries have licensing questions. They come last.

**Scope of 1.0:** all P0 and P1 rows (phases 0 to 8), plus Windows signing (phase 9). P2 rows are the long tail (phase 10) and may be pulled forward when cheap, but never at the cost of a 1.0 row.

## Summary

| Phase | Goal | Effort | Release |
|---|---|---|---|
| 0 | Foundations: workspace, CI/CD, tokens, test tooling | S | none (pipeline proven) |
| 1 | Core data: geometry, model, diagram, history, native format | M | none |
| 2 | Engine, SVG renderer, CLI (headless vertical slice) | M | none |
| 3 | Walking skeleton UI on web and Windows | L | v0.1 alpha |
| 4a | Class diagram, complete | XL | v0.2 |
| 4b | Formatting, export, save conveniences, performance gate | L | v0.3 beta |
| 5 | Editing power and quality tools | L | v0.4 |
| 6 | Core diagram types | XL | v0.5 |
| 7 | More diagrams and `.mdj` compatibility | L | v0.6 |
| 8 | Code generation, DDL, automation, install polish | L | v0.9 release candidate |
| 9 | Hardening: performance, accessibility audit, signing | M | v1.0 |
| 10 | Long tail (P2) in four ordered waves | XL | v1.x |

---

## Phase 0: Foundations (S)

**Goal:** an empty app that builds, tests, ships and looks right, so nothing later is blocked by plumbing.

**Steps**
1. Decisions, written into `docs/decisions.md`: project licence (drives `deny.toml`), final name and a trademark sanity check (UMLForge is a working name), pinned Dioxus and `dx` versions.
2. Cargo workspace skeleton with the crate layout from AGENTS.md section 2, shared lints (`unsafe_code = "forbid"`, clippy panic-free denies), `rust-toolchain.toml`.
3. `xtask`: `layers`, `wasm`, `parity` (including the build-plan check), `test-report` stub.
4. CI green: `ci.yml`, `quality.yml`, `release.yml` from this repo; `deny.toml`; `.config/nextest.toml`; Codecov token.
5. `ui/src/theme.rs` with the tokens from `docs/design.md`, light and dark, plus the contrast test; i18n string-table skeleton; font files with licences and `ATTRIBUTION.md`.
6. `crates/testkit`: injectable id generator and clock, temp-dir helpers, snapshot helpers.
7. Hello-world Dioxus app for web and desktop showing the theme; tag `v0.0.1` to prove the release pipeline, Pages deploy and Windows installer end to end.

**Rows:** OPS-001, OPS-002, OPS-003, OPS-004, OPS-005, OPS-006, OPS-008, OPS-011, OPS-012, OPS-015, APP-011, APP-013

**Exit:** a PR runs every gate green; a tag produces a Windows installer, a web bundle and a live Pages site.

## Phase 1: Core data (M)

**Goal:** the data model and its rules, fully tested, with no UI.

**Steps**
1. `geom`: points, rects, transforms, hit tests, segment intersection, border anchors. Property tests.
2. `model`: ids (stable, injectable generator), project, packages, classifier kinds needed for classes first, relationships, tags and stereotypes (fields only). Walkers bounded against cycles.
3. `diagram`: diagrams, views that reference model ids, connectors, z-order, formatting fields (stored, not yet editable).
4. `history`: undo/redo as an inverse of every change; property test "apply then undo equals original".
5. `format`: native versioned project format, `#[serde(default)]`, unknown fields preserved, round-trip and corrupt-input tests; the compile-time exhaustiveness guard from AGENTS.md section 7.
6. Benchmark scene generator (2,000 views, 3,000 connectors) in `testkit`; first `criterion` benchmarks for open and save.

**Rows:** IO-001, IO-003, OPS-010

**Exit:** L0 to L2 at or above the coverage floors; generated projects of every size round-trip; benchmark baseline recorded.

## Phase 2: Engine, renderer, CLI (M)

**Goal:** everything that can be done headless, so agents and tests can verify features without a screen.

**Steps**
1. `engine`: `Session`, command registry, command spec (id, label, params doc, `enabled`, `run`), undo integration. First commands: new project, add package, add class, move view, delete (from diagram, from model).
2. `render`: scene graph and SVG output for class diagrams. This is the oracle: golden SVG tests with `insta`.
3. `umlforge-cli`: `new`, `info`, `commands`, `run <command> --params`, `export --svg`.
4. `automation`: JSON control channel skeleton (read state, dispatch command), used later by e2e and agents.
5. `panic_hunt`: fuzz every registered command with adversarial params.

**Rows:** IO-004, EXT-006, EXT-008, OPS-009

**Exit:** a class diagram can be built and exported to SVG from the command line with undo working; `panic_hunt` green.

## Phase 3: Walking skeleton UI (L)

**Goal:** the thinnest end-to-end product: create a class, move it, save, reopen, on web and Windows.

**Steps**
1. **Spike: how to draw the canvas.** Render the 2,000-view benchmark scene as SVG in the DOM versus an HTML canvas, in Dioxus web and desktop. Measure pan, zoom and drag. Pick one, record the numbers and the decision in `docs/architecture.md`. Do this before building the canvas; it is the largest technical risk.
2. **Spike: file access on the web.** The File System Access API is not available in every browser; decide the fallback (upload and download) and record it.
3. UI kit primitives (Button, Field, Menu, Dialog, Tree, Toast, Tooltip) and the `/gallery` page, light and dark, with component tests.
4. App shell from `docs/design.md`: top bar, explorer, canvas, properties, status bar, tabs; real menus bound to command ids; keyboard shortcuts; platform-services trait (open, save, clipboard) with desktop and web implementations.
5. Canvas basics: pan, zoom, select, marquee, move, resize, grid and snap, toolbox, create a class and a connector.
6. Explorer tree and basic properties panel (name).
7. WebDriver e2e journey 1: new project, add class, rename, move, save, reload.

**Rows:** APP-001, APP-002, APP-003, APP-006, APP-008, APP-014, APP-015, APP-021, EXP-001, EXP-002, EXP-005, DGM-001, DGM-002, DGM-003, DGM-004, DGM-008, DGM-009, PRP-001, OPS-017

**Exit:** `v0.1` alpha. Keyboard-only operation works for the above; the canvas holds the benchmark scene at an acceptable frame rate or the spike's follow-up is scheduled.

## Phase 4a: Class diagram, complete (XL)

**Goal:** the most-used diagram done well, including everything it needs from the canvas.

**Steps**
1. Class, interface, enumeration, data type, signal with attributes and operations.
2. Relationships: association (ends, multiplicity, navigability, aggregation, composition), generalization, realization, dependency, usage, association class.
3. Connector routing styles, waypoints, reconnecting ends.
4. Quick edit on canvas, notes, free text and shapes, note links, copy and paste of views, delete from model versus from diagram, drag in the explorer.
5. Properties for documentation, stereotypes, tags, compartments.
6. Visual regression snapshots from the gallery.

**Rows:** CLS-001, CLS-002, CLS-003, CLS-004, CLS-005, CLS-006, CLS-008, CLS-010, CLS-012, EXP-003, EXP-004, DGM-007, DGM-010, DGM-011, DGM-015, DGM-018, DGM-019, DGM-020, PRP-005, PRP-006, PRP-007, PRP-009, OPS-016

**Exit:** `v0.2`. A real class diagram can be drawn without touching the CLI.

## Phase 4b: Look, export and performance (L)

**Goal:** make class diagrams presentable, shareable and fast.

**Steps**
1. Formatting: fill, line, font; reset to default.
2. Export to PNG, JPEG, clipboard image, JSON.
3. Recent files, autosave and crash recovery, preferences, model-and-view copy and paste.
4. Diagram explorer, multi-select in the tree.
5. **Performance gate:** the benchmark scene meets the budget in `perf/budgets.toml` (60 fps pan and zoom target). Fix hot spots now, before more diagram types multiply them.

**Rows:** PRP-002, PRP-003, PRP-011, IO-005, IO-006, IO-010, IO-012, EXP-007, EXP-011, APP-004, APP-005, APP-007, APP-010, DGM-023

**Exit:** `v0.3` beta, usable for real class-diagram work.

## Phase 5: Editing power and quality tools (L)

**Goal:** the features that make daily use pleasant.

**Steps**
1. Alignment guides, align and distribute, self-connectors, containment, auto-resize.
2. Explorer search and "find views".
3. Validation with a problems panel; find in project; jump from problem to element.
4. PDF and batch export.
5. Command palette; text scale, high contrast and reduced motion settings.
6. Stage 3 test dashboard (`test-report`, `/status/` page).

**Rows:** DGM-005, DGM-006, DGM-012, DGM-013, DGM-014, EXP-006, EXP-008, APP-009, APP-022, QLT-001, QLT-002, QLT-004, IO-007, IO-008, OPS-013

**Exit:** `v0.4`.

## Phase 6: Core diagram types (XL)

**Goal:** the diagrams most people reach for after classes. Build in this order, easiest to hardest, so each one reuses the last.

**Steps**
1. General shapes and annotations (they are used by every other diagram).
2. Use case diagram.
3. ER diagram (entities, columns, relationships).
4. Package diagram.
5. State machine diagram.
6. Activity diagram (swimlanes, pins).
7. Sequence diagram last: it has its own layout rules (vertical time, ordered messages), so do it once the canvas and routing are mature.

**Rows:** GEN-001, BEH-001, ERD-001, ERD-002, STR-002, BEH-006, BEH-007, BEH-002

**Exit:** `v0.5`; e2e journeys for each family.

## Phase 7: More diagrams and `.mdj` compatibility (L)

**Goal:** breadth, and the ability to bring existing projects in.

**Steps**
1. Open and save `.mdj` projects for every diagram type that exists. Derive the schema from files you create and from fixtures the maintainers supply with permission; preserve unknown fields (AGENTS.md rule 5). Start with class and use case, widen as types land.
2. Object, component and deployment diagrams.
3. Flowchart, data flow diagram, C4.
4. Sequence combined fragments and drag reorder.

**Rows:** IO-002, STR-001, STR-003, STR-004, FLW-001, DFD-001, C4-001, BEH-003, BEH-004

**Exit:** `v0.6`.

## Phase 8: Code generation, DDL, automation, install polish (L)

**Goal:** the productivity features and the release-candidate polish.

**Steps**
1. Code generation framework (template-driven), then Java, Python, TypeScript; zip download on the web build.
2. DDL generation from ER diagrams.
3. MCP server exposing every command.
4. Windows file association; installable web app with offline cache.

**Rows:** COD-001, COD-002, COD-005, COD-006, COD-011, IO-020, ERD-003, EXT-007, APP-018, APP-019

**Exit:** `v0.9` release candidate; all P0 and P1 rows done or consciously deferred with a note in `docs/gaps.md`.

## Phase 9: Hardening and 1.0 (M)

**Goal:** trustworthy enough to recommend.

**Steps**
1. Performance pass against budgets on both targets; memory check with the largest benchmark scene.
2. Accessibility audit: the manual release checklist in `docs/testing.md` section 7, fixes, and a second keyboard-only pass of every feature.
3. Windows code signing so the installer does not trigger SmartScreen warnings (needs a certificate and the two secrets in `release.yml`).
4. Fuzz time extended on importers; review the unsupported-feature errors for clear wording.
5. Documentation: README, getting-started, the gaps list.

**Rows:** OPS-007

**Exit:** `v1.0`.

## Phase 10: Long tail (XL), in four waves

Do these in order; each wave depends on the stability of the one before.

**Wave 10a: polish and small P2 items.** Cheap, valuable, low risk.
**Rows:** DGM-016, DGM-017, DGM-021, PRP-004, PRP-008, PRP-010, EXP-009, APP-012, APP-016, APP-017, QLT-003, QLT-005, IO-009, IO-011, CLS-007, CLS-009, CLS-011, OPS-014

**Wave 10b: more UML and interchange.** Extends existing models without new infrastructure.
**Rows:** COD-003, COD-004, COD-007, IO-013, IO-014, IO-015, IO-021, STR-005, STR-006, BEH-005, BEH-008, BEH-009, BEH-010, EXT-001, EXT-002

**Wave 10c: new diagram families.** Each is a self-contained new model plus notation. Cloud shape sets last because they need original or permissively licensed artwork.
**Rows:** SYS-001, SYS-002, SYS-003, BPM-001, WFR-001, MND-001, CLD-001

**Wave 10d: extensibility, reverse engineering, auto-update, auto-layout.** These need the most stable foundations and serve the fewest people, so they are last.
**Rows:** EXT-003, EXT-004, EXT-005, COD-008, COD-009, COD-010, EXP-010, APP-020, DGM-022

---

## Rules for working the plan

- Pick work from the lowest-numbered phase that still has open rows. Starting a row from a later phase needs a maintainer's agreement, recorded in the PR.
- A phase's exit criteria must be met before the next phase's first row merges, except rows explicitly marked as independent in the PR description.
- Every phase ends with a tagged release and an entry in `log/devlog.md`; if the release pipeline breaks, fixing it outranks new features.
- Revise this plan when reality disagrees (a spike changes the canvas approach, a phase takes twice as long). Change the plan and the checklist in the same PR, with the reason in the revision history below.

## Revision history

- 2026-10-18: initial plan.