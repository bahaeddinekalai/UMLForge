# Design guide: "Paper & Ink"

The look and feel of UMLForge. Read this before touching `crates/ui`. Colours, sizes and radii live in `crates/ui/src/theme.rs` (rule 6 in AGENTS.md); this file says what the values are and why.

## 1. Who this is for

Someone who has never used a modelling tool should be able to draw a first diagram in minutes. Someone who has used one for twenty years should not feel slowed down. A 70-year-old with reading glasses and a 15-year-old on a tablet must both be comfortable. Design for the slowest, most careful user; experts get shortcuts on top, never instead.

## 2. Principles

1. **One obvious next step.** Each screen has at most one primary button. Empty states say what to do ("Add your first class").
2. **Say it in words.** Every action has a visible text label. Icon-only buttons are allowed only for universal symbols (close, undo, redo, zoom in/out) and always carry a tooltip and an accessible name.
3. **Nothing is hidden behind a gesture.** Anything you can do with a right-click, drag or shortcut can also be done from a menu or a visible button.
4. **Forgiving.** Undo for everything. Destructive actions say exactly what will be lost and name the button after the action ("Delete diagram", never "OK"). Deleting shows a toast with Undo instead of a blocking dialog whenever undo is possible.
5. **Calm.** Lots of space, few colours, no decoration that carries no meaning. The diagram is the star; the chrome steps back.
6. **Plain language.** Short, concrete, no jargon in the interface itself. UML terms are fine where they are the thing being named (Class, Lifeline); everywhere else say "Remove", not "Dereference".
7. **Progressive disclosure.** Properties panels show the few things most people need first; "More options" expands the rest. Never remove power, only defer it.

## 3. Personality (so it is not generic)

- **Warm paper, crisp ink.** The canvas is an off-white paper tone, text is near-black ink. It feels like a drafting table, not a dashboard.
- **One accent: deep teal.** Used for selection, the primary button, links and focus. Nothing else is saturated, so the accent always means "this is what you are working on".
- **Atkinson Hyperlegible** for the interface: a typeface designed for low-vision readers whose letter shapes are unusually distinct. It is friendly, a little distinctive, and the most readable choice for a mixed-age audience.
- **Soft geometry, hairline structure.** 10px radii on panels, 1px borders instead of heavy shadows, a faint dot grid on the canvas.
- **Quiet motion.** Short, purposeful transitions that explain a change (a panel sliding in). Nothing bounces.

Avoid: gradients, glassmorphism/blur, neon, 3D or skeuomorphic buttons, emoji used as icons, dense toolbars of tiny unlabeled glyphs, dark patterns, and tooltips as the only place information lives.

## 4. Colour tokens

Names below are the CSS custom properties generated from `theme::Tokens`. Components use tokens only.

| Token | Light | Dark | Use |
|---|---|---|---|
| `--canvas` | `#F7F5F1` | `#121518` | App and diagram background |
| `--surface` | `#FFFFFF` | `#1A1E22` | Panels, dialogs, cards, inputs |
| `--ink` | `#1B1F23` | `#ECEFF2` | Primary text |
| `--ink-muted` | `#58616B` | `#A5AEB8` | Secondary text, hints |
| `--line` | `#E3DFD7` | `#2A3036` | Decorative dividers only (never the sole boundary of a control) |
| `--line-strong` | `#8A9199` | `#7C858F` | Control borders, icon outlines |
| `--accent` | `#0E6B62` | `#5FCDBF` | Primary button, selection, links, focus ring |
| `--on-accent` | `#FFFFFF` | `#0B1412` | Text on accent |
| `--danger` | `#B42318` | `#FF8A7A` | Destructive actions, errors |
| `--success` | `#177245` | `#5FD39A` | Saved, valid |
| `--warning` | `#8F5200` | `#F0B35A` | Needs attention |

Contrast, computed with the WCAG 2.x formula (needs 4.5:1 for text, 3:1 for control borders and icons):

| Pair | Light | Dark |
|---|---|---|
| ink on canvas / surface | 15.2 / 16.6 | 15.9 / 14.5 |
| ink-muted on canvas / surface | 5.8 / 6.3 | 8.2 / 7.5 |
| accent on canvas / surface | 5.9 / 6.4 | 9.6 / 8.8 |
| on-accent on accent | 6.4 | 9.8 |
| danger / success / warning on surface | 6.6 / 6.0 / 6.2 | 7.3 / 9.0 / 9.0 |
| line-strong on surface | 3.2 | 4.5 |

Rules:
- Any new foreground/background pair must be added to the contrast test in `theme.rs` (it iterates every declared pair and fails below 4.5, or 3.0 for non-text). Do not use a pair that is not in the test.
- Colour never carries meaning alone. Errors have an icon and words; connector types differ by line style and arrowhead, not just colour.
- Diagram fill colours offered to users are soft tints that keep ink text above 4.5:1. They are defined as a palette in `theme.rs` and covered by the same test.
- The tints (`accent-tint` for selection backgrounds, hover shades) are derived from the table and checked by the test before use.

Dark theme is designed, not inverted: surfaces are slightly lifted greys rather than pure black, and the accent is lighter to keep contrast. Follow the system setting by default; the user can override it (APP-011). Honour the OS high-contrast / forced-colors mode (APP-022).

## 5. Typography

- **UI:** Atkinson Hyperlegible (variable if available). **Code, ids, DDL:** JetBrains Mono. Both are open-licence fonts: confirm the licence file, store it next to the font, and add a row to `ATTRIBUTION.md` in the PR that adds them. Fallback stack: `system-ui, "Segoe UI", sans-serif`. The app must stay fully usable if the fonts fail to load.
- **Sizes:** 16px default for UI and body; 14px is the minimum anywhere in the chrome; headings 20 / 24 / 32. Diagram text defaults to 14px and is user-adjustable. No text below 14px, ever.
- **Weights:** 400 and 600 only. Bold for emphasis, never for decoration.
- **Line height:** 1.5 for body, 1.3 for headings. Line length in panels and dialogs under about 60 characters.
- **User text scale:** 100%, 125%, 150% setting (APP-022). Layouts must not break or clip at 150% and at browser zoom 200%.
- Sentence case everywhere ("Add attribute", not "Add Attribute"). Do not use all caps.

## 6. Space, shape, elevation

- **Spacing scale (4px base):** 4, 8, 12, 16, 24, 32, 48. Panels use 16px padding; related controls 8px apart; sections 24px apart.
- **Radii:** 6px controls and chips, 10px panels and cards, 14px dialogs, full round for pills.
- **Borders:** 1px `--line-strong` for controls, 1px `--line` for panel dividers.
- **Elevation:** only two levels. Flat (panels, 1px border) and floating (menus, dialogs, toolbox: a soft shadow `0 6px 24px` at about 12% ink plus the border). No other shadows.
- **Touch and pointer targets:** at least 40×40px on desktop, 44×44px when the pointer is coarse (tablets). Spacing between adjacent targets at least 8px. Canvas resize handles are drawn 10px but have a 24px hit area.

## 7. Layout

```
┌────────────────────────────────────────────────────────────────┐
│ ☰ File Edit View Help     Project name ▾     ↶ Undo ↷ Redo  Export ▸ │ top bar, 56px
├─────────┬──────────────────────────────────────┬───────────────┤
│ Explorer│ ┌──────┐     canvas (dot grid)        │ Properties    │
│  280px  │ │toolbox│                              │   320px       │
│ (collaps│ │labelled│    ┌────────┐               │ (collapsible, │
│  ible)  │ └──────┘    │ Class  │               │  contextual)  │
├─────────┴──────────────────────────────────────┴───────────────┤
│ 100% · 12 elements · All changes saved                          │ status, 28px
└────────────────────────────────────────────────────────────────┘
```

- **Top bar:** menu, project name (click to rename), Undo/Redo with text labels, one primary action (Export). Menus are real menus, with shortcuts shown on the right.
- **Explorer (left):** tree of the model. Collapsible; remembers its state. Real tree semantics for keyboard and screen readers.
- **Canvas (centre):** the diagram, on `--canvas` with a faint dot grid (grid size = snap size). A small **floating toolbox** hugs the left edge: a rounded card listing the tools for the current diagram type, each with icon and label ("Class", "Association"). Hovering shows the shortcut.
- **Properties (right):** shows the selected item. Top section "Basics" (name, type, visibility); "Appearance" and "Advanced" are collapsed by default. Nothing selected shows the diagram's own properties and a short hint.
- **Status bar:** zoom, counts, save state in words ("All changes saved", "Saving…", "Couldn't save: disk is full. Try Save As").
- **Narrow windows (< 900px):** explorer and properties become slide-over panels opened from the top bar; the canvas keeps the full width. The web app must be usable on a tablet.
- **Empty project:** a centred card on the canvas: "Start a diagram", with large buttons for the most common types and "Open a project".

## 8. Components

| Component | Rules |
|---|---|
| Primary button | Filled `--accent`, `--on-accent` text, 40px high, 6px radius, verb label. One per view. |
| Secondary button | 1px `--line-strong` outline, ink text. |
| Quiet button | Text only, accent colour, for tertiary actions. |
| Danger button | Filled `--danger`, only inside a confirmation. |
| Text input | 40px high, 1px `--line-strong`, visible label above (not placeholder-as-label), error text below with icon. |
| Menu / dropdown | Floating, 10px radius, 40px rows, shortcut right-aligned in muted ink, disabled items stay visible but muted. |
| Dialog | Floating, 14px radius, title as a sentence ("Delete this diagram?"), body says what will happen, buttons: cancel on the left, action on the right, named for the action. Esc cancels, Enter confirms the primary. |
| Toast | Bottom centre, 4s, plain sentence plus an Undo button where possible. Never the only place an error appears. |
| Tooltip | Name plus shortcut, appears on hover and on keyboard focus, 400ms delay. Never holds essential information. |
| Tree | Rows 36px, chevron + icon + name, selected row uses accent tint with a 3px accent bar on the left. |
| Tabs (diagrams) | Text tabs with close button, current tab underlined with accent. Overflow scrolls with arrows. |
| Icons | 24px grid, 1.75px stroke, rounded caps and joins, drawn in-house or from a permissive set (record in `ATTRIBUTION.md`). Always paired with a label unless listed in principle 2. |

## 9. The diagram canvas

- **Default element:** white fill (`--surface`), 1.25px `--ink` border, 4px corner radius, 14px text, 8px inner padding.
- **Selection:** 2px `--accent` outline, accent-tint fill wash only for marquee. Eight white resize handles with accent border (10px visible, 24px hit area). Multiple selection shows one combined box.
- **Connectors:** 1.5px, ink colour by default; arrowheads and dash styles follow UML notation exactly, because they carry meaning. Hovering a connector thickens it to 2.5px and shows its end handles.
- **Hover** on any element: a faint accent outline so people learn what is clickable.
- **Dot grid:** 1.5px dots at 12% ink, hidden below 50% zoom so it never becomes visual noise.
- **Snap and alignment guides:** thin accent lines that appear only while dragging.
- **Zoom:** wheel with Ctrl, pinch on touch, buttons in the status bar; "Fit" and "100%" are always one click away.
- **Text on canvas** is never smaller than 14px at 100% zoom; at lower zoom it scales naturally.

## 10. Motion

- Durations 120–180ms, ease-out. Used for panel open/close, menu appear, toast in/out, selection outline.
- No motion is required to understand anything. All animation is disabled when the OS asks for reduced motion.
- The canvas itself never animates content position except in response to a direct drag or a deliberate "Fit to window".

## 11. Accessibility (non-negotiable)

Target: WCAG 2.2 level AA for the whole app (APP-021, APP-022).

- Everything is reachable and operable with the keyboard alone, in a sensible order. A visible focus ring (2px `--accent`, 2px offset) on every focusable element; never remove outlines.
- Explorer is a real tree (`role="tree"`), menus are real menus, dialogs trap focus and return it on close, toasts use a polite live region.
- Every control has an accessible name; icons that are decorative are hidden from assistive tech.
- Diagram elements are reachable by Tab / arrow keys from the canvas, announce their name and type, and can be moved with the keyboard (arrows, Shift for larger steps).
- Text scale 100–150% and browser zoom to 200% without loss of content; reflow, no horizontal scroll in dialogs.
- Forced-colors (high contrast) and reduced-motion modes are honoured.
- Touch: pinch zoom, two-finger pan, long-press for context menu, targets as in section 6.
- Language: every string comes from the i18n layer so it can be translated; layouts tolerate text 40% longer than English; avoid idioms and humour in the interface.

## 12. Voice

Short, kind, concrete.
- Do: "Couldn't save the project. The disk is full. Free some space or choose Save As."
- Don't: "Error 0x80070070: write failed."
- Do: "Delete 3 classes? They will also be removed from 2 diagrams. You can undo this."
- Don't: "Are you sure?"
- Buttons are verbs ("Save", "Add attribute", "Delete diagram"). Titles are plain sentences. No exclamation marks, no jokes in errors.

## 13. How to implement

- `crates/ui/src/theme.rs` holds `Tokens` (light, dark, high-contrast) and generates one CSS file of custom properties. Components reference `var(--token)` only; hard-coded colours or sizes fail review.
- Shared primitives (Button, Field, Menu, Dialog, Tree, Toast, Tooltip) live in `crates/ui/src/kit/`. Feature code composes them and does not restyle them.
- A component gallery page (`/gallery` route, dev builds only) shows every primitive in light, dark, 150% text scale, focus and disabled states. It is also what the visual-regression tests screenshot (see `docs/testing.md`).

## 14. Design review checklist (copy into the PR for any UI change)

- [ ] Uses only tokens and kit components; no hard-coded colours, sizes or strings.
- [ ] Looks right in light, dark and at 150% text scale (screenshots attached).
- [ ] Every action is reachable by keyboard and by a visible control; focus order makes sense.
- [ ] Every control has a visible label or accessible name; no colour-only meaning.
- [ ] Copy is plain, sentence case, verbs on buttons; destructive actions are named and undoable.
- [ ] Targets are at least 40px (44px on touch); spacing follows the scale.
- [ ] Empty, loading and error states exist and tell the person what to do next.
- [ ] A new foreground/background pair was added to the contrast test.