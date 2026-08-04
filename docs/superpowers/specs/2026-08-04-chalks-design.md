# chalks — pencil/chalk-style drawing for Typst

**Date:** 2026-08-04
**Status:** Draft for review
**Repo:** `sci-sketch` (monorepo, scenery-style layout)

## Purpose

A Typst package for hand-drawn, pencil/chalk-look figures in scientific
presentations. Primary target: Touying slides. Two jobs it must do well:

1. **Sketchy primitives** — lines, arrows, rects, ellipses, polygons, freehand
   and function-sampled curves, braces — for conceptual "idea figures".
2. **Sketchy annotations anchored to typeset content** — circle a term inside
   an equation, draw an arrow between two marked spots, underline a result —
   like a lecturer marking up a slide. Equations stay normally typeset; the
   library only draws on top of them.

## Naming

Package name: **`chalks`**. `pencil` was rejected as too close to Typst
Universe's "no obvious or canonical names" rule (their example: `slides` is
forbidden, `sliding` is fine). `chalks` is evocative rather than canonical,
free on Universe (checked 2026-08-04, as are `chalk`, `pencil`, `graphite`),
and matches the lecture/blackboard use case. Rust crate: `chalks-engine`.

## Architecture

A minimal shape-agnostic Rust engine plus a Typst layer that owns all shape
logic. The engine can stay stable while the shape library grows, and users can
call `stroke`/`fill` directly for custom figures.

```
sci-sketch/
├── Makefile                  # pkgroot linking, test/examples/plugin fan-out
├── chalks/                   # the Typst package
│   ├── typst.toml            # entrypoint lib.typ
│   ├── lib.typ
│   ├── plugin/chalks_engine.wasm
│   ├── src/
│   │   ├── engine.typ        # WASM plugin bindings, CBOR encode/decode
│   │   ├── style.typ         # style dict defaults, validation, themes
│   │   ├── shapes.typ        # rect, ellipse, circle, polygon, arrow, brace
│   │   ├── curve.typ         # path(..points), fn-curve(f, domain)
│   │   ├── canvas.typ        # sketch(width, height, ..elements)
│   │   ├── pin.typ           # pin(<label>)[content]
│   │   └── annotate.typ      # page-overlay annotations referencing pins
│   ├── tests/
│   ├── examples/
│   └── manual.typ
└── chalks-engine/            # Rust crate → wasm32-unknown-unknown
    ├── Cargo.toml
    ├── src/
    └── tests/
```

Local development follows scenery: root Makefile symlinks packages into
`_pkgroot/preview/<name>/<version>` and exports `TYPST_PACKAGE_PATH`.

## Engine API (the entire WASM surface)

Two pure functions. Input and output are CBOR. Same `(input, seed)` always
produces the same output — documents build reproducibly.

### `stroke(points, style, seed) → paths`

Renders one hand-drawn stroke through a point sequence.

- **Input:** list of 2D points (open or closed), style dict, integer seed.
- **Pipeline:** interpolate a curve through the points (degree of smoothing
  controlled by `smoothness`), perturb it chalk-style (jitter + bowing,
  optionally multiple offset passes), simulate pressure/width variation along
  the path, and emit the stroke as **closed filled-outline paths** — never
  stroked polylines. Variable width is what makes the result read as
  pencil/chalk rather than a wobbly line (the perfect-freehand insight,
  combined with rough.js-style perturbation).
- **Output:** list of closed paths (cubic-bezier outlines) to be filled.

### `fill(boundary, style, seed) → paths`

Fills a closed region with doodles.

- **Input:** one or more closed boundary polygons (holes allowed via even-odd
  nesting), style dict, integer seed.
- **Pipeline:** interpolate the boundary through its input points per
  `smoothness` (same rule as `stroke`), generate doodle paths inside the
  resulting region according to the fill pattern, then feed each doodle
  through the same stroke pipeline — fills and outlines share one texture
  language.
- **Patterns:** `hachure` (parallel sketchy lines at an angle), `scribble`
  (continuous back-and-forth doodle), `shade` (layered low-opacity passes for
  soft graphite shading).
- **Output:** list of closed filled-outline paths, plus a relative weight
  (0–1) per path for `shade`; the Typst layer maps weights to actual
  opacity/color.

### Style dict (shared vocabulary)

| Key          | Applies to    | Meaning                                                        | Default |
|--------------|---------------|----------------------------------------------------------------|---------|
| `smoothness` | stroke + fill | 0 = sharp polyline corners, 1 = fully flowing curve, controlling interpolation through the input points — the stroke path in `stroke`, the region **boundary** in `fill`. Identical semantics in both, so `stroke(points) + fill(points)` with the same style stay congruent (the fill never pokes out of its own outline). Doodle character inside a fill is governed by `roughness` and `pattern`, not by this key. | 0.7 |
| `roughness`  | stroke + fill | amplitude of jitter/bowing relative to size                    | 1.0 |
| `width`      | stroke + fill | nominal stroke width (pt)                                      | 1.2 |
| `taper`      | stroke        | pressure variation 0–1 (0 = uniform, 1 = strong taper at ends) | 0.5 |
| `passes`     | stroke        | number of overlapping strokes (1 = single, 2 = sketchy double) | 1 |
| `pattern`    | fill          | `hachure` \| `scribble` \| `shade`                             | hachure |
| `angle`      | fill          | hachure/scribble direction (deg)                               | 45 |
| `spacing`    | fill          | gap between doodle lines (pt)                                  | 4 |

Colors and opacity are **not** engine concerns — the Typst layer applies them
when converting paths to `curve` elements, so theming stays in Typst.

## Typst layer

### Rendering

`engine.typ` decodes CBOR paths and emits native Typst `curve` elements with
`fill` (no CeTZ dependency). Colors, opacity, and layering are applied here
from the active theme/per-call style.

### Shapes and curves (all Typst-side point generators)

- `line`, `arrow`, `rect`, `ellipse`, `circle`, `polygon`, `brace`, `bracket`
  — generate point lists / boundary regions and call `stroke`/`fill`. Adding
  a shape never touches Rust. Sharp-cornered shapes (e.g. `rect`) pass a low
  per-call `smoothness` for their edges by default.
- `path(..points)` — freehand curve through explicit points.
- `fn-curve(f, domain, samples: 32)` — samples a Typst closure, then `stroke`.
- Every function accepts style overrides (`roughness: 1.5`, `fill: "scribble"`,
  `seed: 42`, …). Default seed is derived deterministically from the input
  geometry, so unchanged figures never re-roll between builds.

### Canvas

`sketch(width, height, ..elements)` — a simple coordinate space (user units →
canvas size) that places primitives and returns ordinary content, embeddable
anywhere. Not a CeTZ canvas.

### Pins and annotations

- `#pin(<l>)[$x^2$]` wraps content transparently and records its position/size
  via `context` + `locate`.
- `#annotate(circle: <l>)`, `#annotate(underline: <l>)`,
  `#annotate(box: <l>, label: [...])`, `#annotate(arrow: (<a>, <b>))` — drawn
  on a page overlay (`place` at page level) so they float above slide content.
  Annotations are ordinary content, so Touying's `#uncover`/fragment reveals
  apply to them directly.
- Unknown pin labels fail at compile time with a message naming the label.

### Themes

Style presets: `pencil` (default — graphite gray, textured, taper),
`ink` (darker, single-pass, crisper), `chalk` (light-on-dark, wider, softer —
for dark Touying themes). Document-wide default via `#chalks-theme(chalk)`
(state-based); every call can override any key.

## Error handling

- Engine returns structured CBOR errors (bad spec, too few points, degenerate
  boundary); `engine.typ` surfaces them as `panic` with readable messages.
- Style validation (unknown keys, out-of-range values) happens Typst-side
  before the plugin call, so users get errors in Typst terms.

## Testing

- **Rust:** unit tests for geometry invariants (outlines closed, determinism,
  bounds within tolerance of input) and snapshot tests of generated paths.
- **Typst:** compile tests plus reference-image regression for examples,
  scenery-style; `make test` at the root fans out both.

## Out of scope (v1)

Plot layer with axes/ticks, handwriting/math fonts, 3D, animation, image
tracing, CeTZ integration, raster chalk texture (grain is approximated with
multi-pass opacity, not bitmaps).
