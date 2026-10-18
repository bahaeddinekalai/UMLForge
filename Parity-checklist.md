# Parity checklist

Scope: every StarUML-style feature UMLForge will reproduce. Behaviour only, clean-room (see AGENTS.md rule 4).

**Status:** ⬜ not started · 🚧 in progress · 🟡 partial · ✅ done · ⛔ out of scope
**Branch rule:** one feature = one branch `feat/<ID>-<slug>`. Claim a row by setting 🚧 and the branch name in your first commit.
**Source of rows:** StarUML's public documentation and observed behaviour, plus the OMG/ISO specs. Rows marked *verify* must be confirmed against real behaviour before being marked ✅. Priority: **P0** core, **P1** expected, **P2** long tail.

Last revision: 2026-10-18 (initial draft).

---

## 1. Application shell (APP)

| ID | Feature | Pri | Status | Branch | Notes |
|---|---|---|---|---|---|
| APP-001 | New project | P0 | ⬜ | | |
| APP-002 | Open project (native file dialog on Windows, File System Access / upload on web) | P0 | ⬜ | | |
| APP-003 | Save / Save As | P0 | ⬜ | | |
| APP-004 | Recent files list | P1 | ⬜ | | |
| APP-005 | Autosave and crash recovery | P1 | ⬜ | | |
| APP-006 | Undo / Redo with history panel | P0 | ⬜ | | |
| APP-007 | Cut / Copy / Paste (model and views) | P0 | ⬜ | | |
| APP-008 | Menu bar with shortcuts, every item bound to a command id | P0 | ⬜ | | |
| APP-009 | Command palette | P1 | ⬜ | | |
| APP-010 | Preferences dialog (grid, snap, fonts, defaults, theme) | P1 | ⬜ | | |
| APP-011 | Light and dark themes | P1 | ⬜ | | |
| APP-012 | Customisable keyboard shortcuts | P2 | ⬜ | | |
| APP-013 | Localisation framework (English first) | P1 | ⬜ | | |
| APP-014 | Panel layout: toolbox, explorer, editors, properties; resizable/collapsible | P0 | ⬜ | | |
| APP-015 | Multiple diagram tabs | P0 | ⬜ | | |
| APP-016 | Full-screen / zoom UI scale | P2 | ⬜ | | |
| APP-017 | About window and contributor credits | P2 | ⬜ | | |
| APP-018 | Windows installer, file association for project files | P1 | ⬜ | | |
| APP-019 | Web app: installable PWA, offline cache | P1 | ⬜ | | |
| APP-020 | Auto-update check (desktop) | P2 | ⬜ | | |
| APP-021 | Keyboard-only operation of every feature, screen-reader roles and names (WCAG 2.2 AA) | P0 | ⬜ | | see docs/design.md section 11 |
| APP-022 | Text scale 100-150%, high-contrast / forced-colors support, reduced motion | P1 | ⬜ | | |

## 2. Model explorer and project structure (EXP)

| ID | Feature | Pri | Status | Branch | Notes |
|---|---|---|---|---|---|
| EXP-001 | Model tree (project, packages, elements, diagrams) | P0 | ⬜ | | |
| EXP-002 | Rename in place | P0 | ⬜ | | |
| EXP-003 | Drag and drop to move elements between packages | P0 | ⬜ | | |
| EXP-004 | Delete from model vs delete from diagram | P0 | ⬜ | | |
| EXP-005 | Drag element from explorer onto a diagram to create a view | P0 | ⬜ | | |
| EXP-006 | Search/filter in explorer | P1 | ⬜ | | |
| EXP-007 | Diagram explorer (list of diagrams) | P1 | ⬜ | | |
| EXP-008 | Show element usages / "find views" | P1 | ⬜ | | |
| EXP-009 | Sorting and visibility filters (by type) | P2 | ⬜ | | |
| EXP-010 | Model fragments / sub-projects stored in separate files | P2 | ⬜ | | verify |
| EXP-011 | Multi-select operations in tree | P1 | ⬜ | | |

## 3. Diagram canvas and editing (DGM)

| ID | Feature | Pri | Status | Branch | Notes |
|---|---|---|---|---|---|
| DGM-001 | Pan and zoom (wheel, drag, fit, 100%) | P0 | ⬜ | | |
| DGM-002 | Select, multi-select, marquee select | P0 | ⬜ | | |
| DGM-003 | Move and resize views with handles | P0 | ⬜ | | |
| DGM-004 | Grid display and snap-to-grid | P0 | ⬜ | | |
| DGM-005 | Smart alignment guides while dragging | P1 | ⬜ | | |
| DGM-006 | Align / distribute / same size commands | P1 | ⬜ | | |
| DGM-007 | Z-order (front, back) | P1 | ⬜ | | |
| DGM-008 | Toolbox palette per diagram type | P0 | ⬜ | | |
| DGM-009 | Create connectors by drag between views | P0 | ⬜ | | |
| DGM-010 | Connector routing styles: rectilinear, oblique, curve, rounded | P0 | ⬜ | | |
| DGM-011 | Connector waypoints, reconnect ends | P1 | ⬜ | | |
| DGM-012 | Self-connectors (loops) | P1 | ⬜ | | |
| DGM-013 | Containment: drop views inside packages/containers | P1 | ⬜ | | |
| DGM-014 | Auto-resize to fit content | P1 | ⬜ | | |
| DGM-015 | Quick edit (inline text editing, quick-add attributes/operations) | P1 | ⬜ | | |
| DGM-016 | Overview / minimap | P2 | ⬜ | | |
| DGM-017 | Diagram properties: size, background, grid | P2 | ⬜ | | |
| DGM-018 | Text labels, notes, free text, free lines, shapes | P1 | ⬜ | | |
| DGM-019 | Note links (note to element) | P1 | ⬜ | | |
| DGM-020 | Copy/paste views between diagrams | P1 | ⬜ | | |
| DGM-021 | Zoom to selection | P2 | ⬜ | | |
| DGM-022 | Auto-layout (hierarchical, grid, tree) | P2 | ⬜ | | beyond or equal to StarUML, verify |
| DGM-023 | Performance: 2,000 views, 60 fps pan/zoom (benchmark) | P0 | ⬜ | | |

## 4. Properties, formatting, documentation (PRP)

| ID | Feature | Pri | Status | Branch | Notes |
|---|---|---|---|---|---|
| PRP-001 | Properties editor for selected element | P0 | ⬜ | | |
| PRP-002 | Format: fill color, line color, line width, line style | P0 | ⬜ | | |
| PRP-003 | Format: font family, size, bold/italic, text color, alignment | P1 | ⬜ | | |
| PRP-004 | Format painter / copy and paste format | P2 | ⬜ | | |
| PRP-005 | Documentation field (rich or markdown) | P1 | ⬜ | | |
| PRP-006 | Stereotype assignment and display (text/icon) | P1 | ⬜ | | |
| PRP-007 | Tags (tagged values) editor | P1 | ⬜ | | |
| PRP-008 | Constraints editor | P2 | ⬜ | | |
| PRP-009 | Show/hide compartments (attributes, operations, visibility, signature) | P1 | ⬜ | | |
| PRP-010 | Default styles per diagram type | P2 | ⬜ | | |
| PRP-011 | Reset to default format | P2 | ⬜ | | |

## 5. UML structural diagrams

### 5.1 Class diagram (CLS)

| ID | Feature | Pri | Status | Branch | Notes |
|---|---|---|---|---|---|
| CLS-001 | Class, abstract class, active class | P0 | ⬜ | | |
| CLS-002 | Interface, Enumeration (with literals), DataType, Signal | P0 | ⬜ | | |
| CLS-003 | Attributes and operations with visibility, type, multiplicity, static, abstract, derived, read-only | P0 | ⬜ | | |
| CLS-004 | Association: ends, role names, multiplicity, navigability, aggregation, composition | P0 | ⬜ | | |
| CLS-005 | Generalization, Realization, Dependency, Usage | P0 | ⬜ | | |
| CLS-006 | Association class | P1 | ⬜ | | |
| CLS-007 | Template parameters and template binding | P2 | ⬜ | | |
| CLS-008 | Package and nested package views | P0 | ⬜ | | |
| CLS-009 | Inner/nested classifiers, containment connector | P2 | ⬜ | | |
| CLS-010 | Parameters (direction, default value), return type | P1 | ⬜ | | |
| CLS-011 | Qualified association, interface realization lollipop notation | P2 | ⬜ | | |
| CLS-012 | Operation signature display options | P1 | ⬜ | | |

### 5.2 Other structural (STR)

| ID | Feature | Pri | Status | Branch | Notes |
|---|---|---|---|---|---|
| STR-001 | Object diagram (instance specification, slots, links) | P1 | ⬜ | | |
| STR-002 | Package diagram (imports, merges, access) | P1 | ⬜ | | |
| STR-003 | Component diagram (components, ports, provided/required interfaces, assembly) | P1 | ⬜ | | |
| STR-004 | Deployment diagram (nodes, artifacts, deployment, communication path) | P1 | ⬜ | | |
| STR-005 | Composite structure diagram (parts, ports, connectors, collaborations) | P2 | ⬜ | | |
| STR-006 | Profile diagram (profiles, stereotypes, metaclass extension) | P2 | ⬜ | | |

## 6. UML behavioural diagrams

| ID | Feature | Pri | Status | Branch | Notes |
|---|---|---|---|---|---|
| BEH-001 | Use case diagram (actors, use cases, subject, include/extend, generalization) | P0 | ⬜ | | |
| BEH-002 | Sequence diagram: lifelines, messages (sync, async, reply, create, destroy, found, lost) | P0 | ⬜ | | |
| BEH-003 | Sequence: combined fragments (alt, opt, loop, par, ref...), activations, gates | P1 | ⬜ | | |
| BEH-004 | Sequence: messages auto-keep vertical order, drag reorder | P1 | ⬜ | | |
| BEH-005 | Communication diagram | P2 | ⬜ | | |
| BEH-006 | State machine diagram (states, regions, pseudostates, transitions, entry/exit/do) | P0 | ⬜ | | |
| BEH-007 | Activity diagram (actions, control nodes, object nodes, swimlanes/partitions, pins) | P0 | ⬜ | | |
| BEH-008 | Timing diagram | P2 | ⬜ | | |
| BEH-009 | Interaction overview diagram | P2 | ⬜ | | |
| BEH-010 | Interaction/state/activity nested diagram references | P2 | ⬜ | | |

## 7. Other diagram families

| ID | Feature | Pri | Status | Branch | Notes |
|---|---|---|---|---|---|
| ERD-001 | Entity-relationship diagram: entities, columns (PK, FK, unique, nullable), types | P0 | ⬜ | | |
| ERD-002 | Identifying / non-identifying relationships, cardinality notation | P0 | ⬜ | | |
| ERD-003 | ERD to DDL generation and DDL to ERD import (see IO-020, IO-021) | P1 | ⬜ | | |
| FLW-001 | Flowchart (process, decision, terminator, data, document, connectors) | P1 | ⬜ | | |
| DFD-001 | Data flow diagram (process, external entity, data store, data flow) | P1 | ⬜ | | |
| SYS-001 | SysML requirement diagram | P2 | ⬜ | | |
| SYS-002 | SysML block definition and internal block diagrams | P2 | ⬜ | | |
| SYS-003 | SysML parametric diagram | P2 | ⬜ | | |
| C4-001 | C4 model (person, system, container, component, relationship) | P1 | ⬜ | | |
| BPM-001 | BPMN diagram (events, tasks, gateways, pools/lanes, flows) | P2 | ⬜ | | |
| WFR-001 | Wireframe diagram (UI widgets) | P2 | ⬜ | | |
| MND-001 | Mind map diagram | P2 | ⬜ | | |
| CLD-001 | Cloud/infrastructure shape sets (AWS, Azure, GCP, Kubernetes); needs permissively licensed originals, not vendor icons | P2 | ⬜ | | check icon licences |
| GEN-001 | General shapes and annotations (rect, ellipse, text, image) | P1 | ⬜ | | |

## 8. Import, export, documents (IO)

| ID | Feature | Pri | Status | Branch | Notes |
|---|---|---|---|---|---|
| IO-001 | Native project format with versioning and migrations | P0 | ⬜ | | |
| IO-002 | Open/save `.mdj` JSON projects | P0 | ⬜ | | derive schema from own and permitted fixtures |
| IO-003 | Preserve unknown fields on round-trip | P0 | ⬜ | | |
| IO-004 | Export diagram as SVG | P0 | ⬜ | | |
| IO-005 | Export diagram as PNG | P0 | ⬜ | | |
| IO-006 | Export diagram as JPEG | P1 | ⬜ | | |
| IO-007 | Export diagram as PDF | P1 | ⬜ | | |
| IO-008 | Export all diagrams in a batch | P1 | ⬜ | | |
| IO-009 | Print / print to PDF | P2 | ⬜ | | |
| IO-010 | Copy diagram to clipboard as image | P1 | ⬜ | | |
| IO-011 | HTML documentation generation | P2 | ⬜ | | |
| IO-012 | Export model to JSON | P1 | ⬜ | | |
| IO-013 | XMI import/export (UML 2.x) | P2 | ⬜ | | |
| IO-014 | Import from other tools (e.g. XMI from Eclipse/Papyrus) | P2 | ⬜ | | |
| IO-015 | Import/export fragments (package as file) | P2 | ⬜ | | |
| IO-020 | DDL generation from ERD (SQL dialects) | P1 | ⬜ | | |
| IO-021 | Reverse engineer DDL to ERD | P2 | ⬜ | | |

## 9. Code generation and reverse engineering (COD)

| ID | Feature | Pri | Status | Branch | Notes |
|---|---|---|---|---|---|
| COD-001 | Code generator framework (template-driven, configurable per project) | P1 | ⬜ | | |
| COD-002 | Java generation | P1 | ⬜ | | |
| COD-003 | C# generation | P2 | ⬜ | | |
| COD-004 | C++ generation | P2 | ⬜ | | |
| COD-005 | Python generation | P1 | ⬜ | | |
| COD-006 | TypeScript generation | P1 | ⬜ | | |
| COD-007 | Kotlin / Swift / PHP / others | P2 | ⬜ | | |
| COD-008 | Reverse engineering framework (parse source into model) | P2 | ⬜ | | |
| COD-009 | Reverse engineer Java | P2 | ⬜ | | |
| COD-010 | Reverse engineer Python, C#, TypeScript | P2 | ⬜ | | |
| COD-011 | Web build: code gen outputs a zip download | P1 | ⬜ | | |

## 10. Validation, search, quality (QLT)

| ID | Feature | Pri | Status | Branch | Notes |
|---|---|---|---|---|---|
| QLT-001 | Model validation with a problems panel (e.g. missing names, duplicate names, invalid relations) | P1 | ⬜ | | |
| QLT-002 | Find in project (elements, text) | P1 | ⬜ | | |
| QLT-003 | Replace names across project | P2 | ⬜ | | |
| QLT-004 | Navigate from problem to element/view | P1 | ⬜ | | |
| QLT-005 | Project statistics | P2 | ⬜ | | |

## 11. Profiles, stereotypes, extensibility (EXT)

| ID | Feature | Pri | Status | Branch | Notes |
|---|---|---|---|---|---|
| EXT-001 | UML profile support (apply profile, stereotypes with tag definitions) | P2 | ⬜ | | |
| EXT-002 | Built-in profiles (e.g. Java, Python, C#, EJB-like) | P2 | ⬜ | | write our own |
| EXT-003 | Extension system (sandboxed WebAssembly plugins) | P2 | ⬜ | | |
| EXT-004 | Extension API: commands, menus, model access | P2 | ⬜ | | |
| EXT-005 | Extension manager UI (install/enable/disable) | P2 | ⬜ | | |
| EXT-006 | Scripting / automation via JSON control channel | P1 | ⬜ | | |
| EXT-007 | MCP server exposing every command | P1 | ⬜ | | |
| EXT-008 | CLI: convert, export, validate, run commands headless | P1 | ⬜ | | |

## 12. Build, quality and distribution (OPS)

| ID | Feature | Pri | Status | Branch | Notes |
|---|---|---|---|---|---|
| OPS-001 | Cargo workspace, layering check (`cargo xtask layers`) | P0 | ⬜ | | |
| OPS-002 | CI: fmt, clippy, tests, wasm check | P0 | ⬜ | | workflow provided |
| OPS-003 | CI: web build artifact | P0 | ⬜ | | workflow provided |
| OPS-004 | CI: Windows build artifact | P0 | ⬜ | | workflow provided |
| OPS-005 | Release pipeline: tag to Windows installer + web bundle + GitHub Release | P0 | ⬜ | | workflow provided |
| OPS-006 | Deploy web build to GitHub Pages | P1 | ⬜ | | |
| OPS-007 | Windows code signing | P2 | ⬜ | | needs certificate secrets |
| OPS-008 | `cargo xtask parity` (parse checklist, counts, floor) | P1 | ⬜ | | |
| OPS-009 | `panic_hunt` fuzz test over all commands | P1 | ⬜ | | |
| OPS-010 | Benchmark suite and perf budgets | P1 | ⬜ | | |
| OPS-011 | Telemetry-free by design (no tracking) | P1 | ⬜ | | |
| OPS-012 | Test tooling: nextest with JUnit output, coverage (llvm-cov), PR summary, coverage floors | P0 | ⬜ | | quality.yml provided; see docs/testing.md |
| OPS-013 | Test status dashboard (`cargo xtask test-report`, published JSON and page) | P1 | ⬜ | | |
| OPS-014 | Mutation testing on L0-L2 crates (weekly) | P2 | ⬜ | | workflow step provided |
| OPS-015 | Dependency and licence policy (`cargo deny`, `deny.toml`) | P0 | ⬜ | | protects the clean-room promise |
| OPS-016 | Visual regression screenshots from the component gallery (light, dark, 150% text) | P1 | ⬜ | | |
| OPS-017 | End-to-end journeys via WebDriver (web) and control channel (desktop) | P1 | ⬜ | | |

---

### Out of scope (decisions)

| ID | Item | Reason |
|---|---|---|
| X-001 | Licence keys, activation, trial limits | We are open source; no licensing layer |
| X-002 | StarUML's own extension registry and extensions | Proprietary/third-party code; we build our own plugin API |
| X-003 | Vendor logos/icons (cloud providers) | Trademark and licence; use original or permissive shapes (see CLD-001) |

### Revision history

- 2026-10-18: initial draft from public knowledge of StarUML's feature set. Rows marked *verify* need confirmation against real behaviour.
- 2026-10-18: added APP-021, APP-022 (accessibility) and OPS-012 to OPS-017 (testing and quality tooling). Every row is assigned a phase in `docs/build-plan.md`.