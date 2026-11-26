# UI/UX redesign — 2026-11-26

## Audit

The prior Slint UI was functional but visually read as a prototype: blue-grey dark surfaces, borders on almost every card/control, identical-looking choices, a disconnected sidebar, and a long single-column problem form. The lack of spacing/type hierarchy made the application feel dense despite generous dimensions.

## Direction

The redesign uses a quiet productivity-tool direction: warm-neutral charcoal in dark mode, neutral off-white in light mode, and a restrained indigo action color. Surfaces establish hierarchy; borders now act only as subtle separators, focus rings, and dialog edges. The visual language favours compact rounded rectangles, clear type contrast, and calm empty states rather than decorative cards.

## Palette

| Token | Dark | Light |
| --- | --- | --- |
| Canvas | `#111216` | `#F5F5F7` |
| Sidebar | `#15151A` | `#F0F0F3` |
| Surface | `#1B1B21` | `#FFFFFF` |
| Raised surface | `#222229` | `#FFFFFF` |
| Hover | `#282830` | `#EAEAF0` |
| Accent | `#6C8EFF` | `#5578E8` |
| Main text | `#F1F1F4` | `#202027` |
| Secondary text | `#A7A7B3` | `#666672` |

Semantic success/warning/danger variants include matching low-contrast soft fills. All are centralized in `ui/theme/tokens.slint`.

## Components and pages redesigned

- New semantic token system: surfaces, field, hover, separator, overlay, semantic fills, geometry and control height.
- Buttons now have primary, secondary, ghost and destructive hierarchy; disabled content remains readable.
- Fields use quiet fills and a dedicated focus outline. Choices act as compact segmented controls instead of outlined buttons.
- Sidebar has calmer spacing, selected-nav indicator, restrained hover, and visual separation from content.
- Home uses low-border stat surfaces, refined progress, and branded empty state.
- Problems gains a compact integrated search/filter surface and cleaner list/empty state.
- Rows communicate state through a narrow accent rail and semantic tags, instead of enclosing borders.
- Recent and Settings use the same spacing/surface rhythm.
- Create/edit form has a fixed header/footer, independent scroll body, grouped metadata, balanced two-column rows, and a compact property/language section.
- Detail and confirmation dialogs share overlay, header, footer and raised-surface treatment.

## QA

`CARGO_TARGET_DIR=/tmp/ducky-ui-redesign cargo check` passed after the redesign. The computer-use environment has no available native desktop or browser host, so screenshot-level inspection cannot occur there. A software-renderer launch is performed separately; visual interaction remains a limitation of this environment rather than being recorded as complete visual QA.

## Follow-up visual QA

On a desktop with an available surface, verify 1120×760 and narrower window layouts in each theme, then inspect dialog scroll/content, Vietnamese labels, native picker, and external open operations. No known compile-time clipping errors remain, but this manual visual pass is required before claiming pixel-level QA.
