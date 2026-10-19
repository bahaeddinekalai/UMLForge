# Testing and quality

How we know UMLForge works, and how we see that at a glance. Everything here is enforced by CI where a tool can enforce it; the rest is part of review.

## 1. The core rule

**A feature is never tested only as a block.** "Create an association" is not covered because one end-to-end test happens to draw one. The functions that build the feature (parsing a multiplicity, finding the anchor point on a border, routing the connector, the command that creates it, the properties panel that edits it) each have their own tests, and so does the UI that exposes it. A feature PR is complete only when all three levels exist:

1. **Parts:** unit and property tests for every non-trivial function the feature added or changed.
2. **Feature:** a command or integration test that exercises the feature through the engine (success, bad input, undo).
3. **UI:** component tests for the screens and controls, plus visual and accessibility checks where the look matters.

## 2. Test layers

| Layer             | What it checks                                                                                                                                | Tooling                                                                                   | Runs                                         |
| ----------------- | --------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- | -------------------------------------------- |
| Unit              | One function or type, in isolation. Fast, deterministic, no I/O.                                                                              | `#[test]` in the crate                                                                    | every PR                                     |
| Property          | Invariants over many generated inputs (round-trips, undo/redo is an inverse, geometry never yields NaN, routing never crosses its own start). | `proptest`                                                                                | every PR                                     |
| Command           | Every engine command: success path, param validation, wrong state, graceful `Err` (never a panic), `enabled` state, undo and redo.            | `crates/engine/tests`                                                                     | every PR                                     |
| Format / golden   | Project files round-trip; migrations from every old version; `.mdj` fixtures; rendered SVG equals the reviewed golden.                        | `insta` snapshots, `testkit` fixtures                                                     | every PR                                     |
| Fuzz              | Importers and parsers never panic on hostile input; `panic_hunt` fuzzes every command.                                                        | `cargo-fuzz`, `panic_hunt`                                                                | nightly (short on PR for `panic_hunt`)       |
| UI component      | A component renders the right structure, roles, labels and states, and reacts to events.                                                      | `dioxus-ssr` plus virtual-DOM event dispatch (check the API of the pinned Dioxus version) | every PR                                     |
| Browser           | Web-only code: storage, file picker fallbacks, clipboard.                                                                                     | `wasm-bindgen-test`                                                                       | every PR (headless browser)                  |
| End-to-end        | A few user journeys in the real app: create a diagram, save, reload, export. 10 to 30 journeys total, not one per feature.                    | WebDriver (`thirtyfour` plus chromedriver) for web; the control channel for desktop       | every PR (web), nightly (Windows desktop)    |
| Visual regression | Key screens look as designed: light and dark, 100% and 150% text scale, focus and error states. Taken from the component gallery.             | screenshot compare with a small tolerance; diffs are reviewed in the PR                   | every PR                                     |
| Accessibility     | Token contrast, roles and names in component tests, keyboard order. Manual pass each release (section 7).                                     | contrast test in `theme.rs`, component assertions                                         | every PR plus release                        |
| Performance       | The benchmark diagram stays within budget.                                                                                                    | `criterion`, `perf/budgets.toml`                                                          | nightly; a regression of more than 10% fails |
| Mutation          | Do the tests actually fail when the code is wrong?                                                                                            | `cargo-mutants` on L0 to L2 crates                                                        | weekly                                       |

Test tooling stays in Rust (see AGENTS.md). If a JavaScript test tool such as an accessibility scanner is ever added, it runs only inside the e2e job and ships nothing.

## 3. What must be tested

- **Every function with logic**, in L0 to L5: geometry, routing, layout, model operations, validation rules, name and multiplicity parsing, formatters, code and DDL generators (the generator functions and the whole output), importers.
- **Every public API** of a crate: at least one test per public function through its public interface.
- **Every command**: success, each param failure, wrong document state, undo, redo, `enabled`.
- **Invariants**: ids are stable across save/load; deleting a view does not delete its model element; undo then redo equals the original; no cycles hang the walker.
- **Every file format path**: round-trip, unknown fields preserved, every migration, truncated and corrupt input returns `Err`.
- **Every bug fix**: a regression test that failed before the fix (AGENTS.md "Prove it").
- **Every UI component in the kit and every screen**: structure, accessible names, keyboard behaviour, each visual state (empty, filled, disabled, error), light and dark.
- **Error paths and edge cases**: empty input, one item, a huge item, duplicates, Unicode names, zero and negative sizes, NaN and infinity where numbers come from outside.
- **Localisation hooks**: no string literals in components; a test fails if a component renders text that is not from the string table.

## 4. What not to test

- Behaviour of third-party crates and of Dioxus itself. Test how we use them, not whether they work.
- Trivial code: plain getters and setters, `#[derive]`d impls, constants, `Default` for plain data, one-line forwarding functions.
- Private implementation details. Test through the public interface so refactors do not break tests. If a private function is complex enough to need its own test, it probably deserves its own module with a public API.
- Exact pixel output of fonts or anti-aliasing. Compare SVG structure (the oracle) and use tolerant screenshot compare.
- Huge snapshots nobody will read. A golden must be small enough to review; split it.
- The same behaviour at several layers. If a unit test proves the multiplicity parser, the e2e journey does not need to retest every multiplicity.
- Implementation order, timing or log text unless that is the specified behaviour.
- Anything that needs the network or the wall clock. Inject the dependency instead.

## 5. Writing good tests

- **Name:** `unit_condition_expected`, for example `parse_multiplicity_rejects_negative_lower_bound`. One behaviour per test.
- **Shape:** arrange, act, assert. No logic (loops, branches) in a test except in property tests.
- **Deterministic:** no sleeps, no real time, no random seeds you cannot reproduce. Use the `testkit` id generator and clock; property tests print and persist their seeds.
- **Independent:** no test depends on another or on run order; no shared mutable state; temp files via `testkit`.
- **Real over mocked:** prefer real objects and in-memory fakes. Mock only what is slow or external (the file system, clipboard, clock).
- **Messages:** assertions say what was expected in domain words, so a failure explains itself.
- **Fixtures:** small, hand-made or generated, original (never copied from StarUML samples or personal projects). Shared fixtures live in `crates/testkit`.
- **Flaky tests** are bugs. CI retries once and reports a pass-on-retry as flaky. A flaky test is fixed or quarantined with `#[ignore = "flaky: <issue>"]` within 48 hours, and quarantined tests are listed on the dashboard. More than five at once blocks merges to `main`.

## 6. Worked example: CLS-004 association ends

| Level   | Tests                                                                                                                                                                                                                                                                                                       |
| ------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Parts   | `parse_multiplicity` (valid, `*`, ranges, negative, empty, Unicode); `anchor_on_border` for each side and corner; `route_orthogonal` never produces NaN and respects obstacles (property test); `end_label_position` avoids overlapping the line; model: `set_aggregation_kind`, navigability flags.        |
| Feature | Command `uml.add_association`: creates model relationship plus view; rejects missing endpoints; undo removes both; redo restores; deleting the view keeps the relationship; panic_hunt covers it. Save/load round trip keeps both ends and their order.                                                     |
| UI      | Properties panel renders both ends with labels, multiplicity and navigability controls, with accessible names; editing a field dispatches the command; keyboard-only flow works; visual snapshot of the panel and of the canvas in light and dark; one e2e journey may include drawing a class association. |

## 7. Quality gates (code quality, not only features)

Enforced on every PR (see `.github/workflows/ci.yml` and `.github/workflows/Quality.yml`):

- `cargo fmt --check`, `cargo clippy -D warnings` with the panic-free lints from AGENTS.md, `unsafe_code = "forbid"`.
- `cargo xtask layers` (layer rules) and `cargo xtask wasm` (wasm compiles).
- `cargo deny check` for licence and advisory problems. This matters for the clean-room promise: only licences allowed by the project policy (`deny.toml`) may enter the dependency tree.
- `cargo doc` with warnings denied; public items documented in L0 to L5 crates.
- No new `TODO` without an issue number; unused dependencies removed (`cargo machete` or equivalent).
- Review limits as guidance, not machine gates: functions that need scrolling to read, or files over about 600 lines, are split.

Coverage floors (starting points; tune after phase 1 and only ever raise them):

| Layers                                                     | Line coverage            | Mutation score (weekly) |
| ---------------------------------------------------------- | ------------------------ | ----------------------- |
| L0 to L2 (geom, model, diagram, history, layout, validate) | 90%                      | 70%                     |
| L3 to L5 (render, format, io, codegen, plugins, engine)    | 80%                      | report only             |
| L6 (ui, automation)                                        | 60% from component tests | not measured            |
| New and changed lines in a PR                              | 85% (patch coverage)     |                         |

Coverage says what was run, not what was checked. It is a floor against neglect, not a goal. The mutation score and review are what show whether the tests are any good.

**Release checklist (manual, each release):** keyboard-only run of the main journey; a screen-reader pass (NVDA on Windows, a browser reader on the web build) of the main journey; browser zoom 200% and text scale 150%; light, dark and forced-colors; one pass on a touch device for the web app.

## 8. Monitoring test results

Start simple, add the dashboard once there is data worth watching.

**Stage 1: every PR (built in, `quality.yml`)**

- `cargo nextest` runs the suite with the `ci` profile (`.config/nextest.toml`): JUnit XML output, one retry so flaky tests are detected rather than hidden.
- The test summary action writes a pass/fail/flaky table to the PR's job summary, so you see failures without opening logs.
- `cargo llvm-cov` produces coverage; the report goes to Codecov (free for public repositories), which comments on the PR with total and patch coverage and sets a status check.
- Badges for CI, coverage and the latest release go in `README.md`.

**Stage 2: nightly (`quality.yml` schedule)**

- Full suite including `#[ignore]`d tests, short fuzz runs, benchmarks, Windows desktop e2e.
- Weekly mutation testing on L0 to L2.
- If a nightly run or a push to `main` fails, the workflow opens an issue automatically with a link to the run.

**Stage 3: dashboard (once the suite is large, around phase 5)**

- `cargo xtask test-report` collects the JUnit files, coverage and benchmark output into one `test-status.json`: pass, fail, skip and flaky counts per crate; coverage per layer; slowest tests; quarantine list; benchmark deltas; and the parity count (done rows versus rows with passing tests).
- The nightly job publishes that JSON plus a small static page to GitHub Pages under `/status/` and keeps the history, so trends are visible.

**What to watch, and when to act**

| Signal                    | Healthy                               | Action                                        |
| ------------------------- | ------------------------------------- | --------------------------------------------- |
| Pass rate on `main`       | 100%                                  | Any failure is fixed or reverted the same day |
| Flaky tests               | 0 to 5, each fixed in 48h             | Quarantine, fix, never ignore silently        |
| Coverage per layer        | at or above floor, trending up        | A drop is a review comment                    |
| PR CI time                | under 15 minutes                      | Over that, split or parallelise jobs          |
| Mutation score (L0 to L2) | at or above 70%                       | Add tests for surviving mutants               |
| Benchmark vs baseline     | within 10%                            | Investigate before merging                    |
| Parity traceability       | every `✅` row names tests that exist | `cargo xtask parity` fails otherwise          |

**Traceability:** each checklist row that is marked `✅` lists the command ids and the test names that prove it. `cargo xtask parity` greps the workspace and fails if a named test does not exist, so the checklist cannot drift from the tests.

## 9. Definition of done for a feature PR

- [ ] Unit and property tests for every non-trivial function added or changed.
- [ ] Command or integration tests: success, each bad input, wrong state, undo and redo.
- [ ] UI component tests and, where it looks different, visual snapshots in light and dark.
- [ ] Accessibility: names, keyboard flow, any new colour pair in the contrast test.
- [ ] Golden or round-trip tests updated and reviewed if formats or rendering changed.
- [ ] Regression test for any bug fixed.
- [ ] No coverage drop below the layer floor; patch coverage at or above 85%.
- [ ] Checklist row updated with the command ids and test names.
- [ ] CI green, including `quality.yml`.
