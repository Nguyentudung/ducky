# Ducky native design system

Implemented in `ui/theme/tokens.slint` and shared components.

## Intent

Calm, compact, premium desktop productivity UI for programmers: restrained blue action color, clear hierarchy, generous-but-not-empty page gutters, and subtle hover/selection feedback.

## Tokens

| Token family | Light | Dark |
|---|---|---|
| Background | `#f5f7fa` | `#171a20` |
| Surface | `#ffffff` | `#20252e` |
| Sidebar | `#edf1f6` | `#1b2028` |
| Text | `#182230` | `#edf2f8` |
| Accent | `#2d70d8` | `#79aef7` |
| Accent soft | `#e5efff` | `#213b61` |

Semantic colors: success `#168354/#69d49a`, warning `#a96500/#f4be5d`, danger `#c9364d/#f28b99`.

## Scale

- Sidebar: 220 px
- Page gutter: 32 px
- Standard controls: 38 px height; filter choices: 30 px
- Corners: 7–16 px, according to component hierarchy
- Typography: 11 px metadata, 13–14 px body/actions, 19–25 px page/dialog headings

## Components

`Button`, `NavItem`, `Field`, `Choice`, `Tag`, `ProblemRow`, page section headings and modal overlays share the tokens. The UI uses vector-rendered Slint primitives rather than browser CSS or emoji assets.
