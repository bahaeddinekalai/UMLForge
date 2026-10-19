# Architecture (phase 0)

One Rust workspace produces a headless CLI and two Dioxus frontend applications (desktop and WebAssembly web).

## Crate layers

| Layer        | Crates                                                     | Responsibility                              |
| ------------ | ---------------------------------------------------------- | ------------------------------------------- |
| L0           | `umlforge-geom`                                            | Reusable geometry                           |
| L1           | `umlforge-model`, `umlforge-diagram`                       | Semantic data and diagram views             |
| L2           | `umlforge-history`, `umlforge-layout`, `umlforge-validate` | Undo/redo, algorithms, rules                |
| L3           | `umlforge-render`, `umlforge-format`                       | SVG oracle and versioned project format     |
| L4           | `umlforge-io`, `umlforge-codegen`, `umlforge-plugins`      | Import/export, code, WebAssembly extensions |
| L5           | `umlforge-engine`                                          | Headless session and commands               |
| L6           | `umlforge-ui`, `umlforge-automation`                       | Dioxus presentation and control channel     |
| Shared tools | `umlforge-testkit`, `xtask`                                | Test helpers and repository tasks           |

Apps under `apps/` depend on higher layers; supporting libraries depend on lower layers. Each lower-level crate must remain usable without Dioxus or platform UI bindings.

## Current status

The walking skeleton renders an initial welcome screen on web. The data, command, rendering, persistence, and platform-service APIs remain to be designed and implemented in build-plan order. Canvas rendering strategy, performance, and browser file-access fallbacks are undecided; evaluate them in phase 3 before implementation.
