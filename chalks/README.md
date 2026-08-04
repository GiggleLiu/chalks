# chalks

Hand-drawn pencil/chalk-style figures and annotations for Typst. Shapes are
built as point lists in pure Typst, then a Rust → WASM engine (`chalks-engine`)
perturbs them into sketchy, variable-width filled outlines — jitter, bowing,
taper, multi-pass strokes, and three doodle fill patterns.

## Quick start

```typst
#import "@preview/chalks:0.1.0": *

#sketch(200pt, 100pt,
  rect((10, 10), (90, 60), fill: "hachure"),
  circle((150, 40), 30, fill: "scribble"),
  arrow((100, 40), (120, 40)),
)
```

`sketch(width, height, ..elements)` lays out primitives (`line`, `arrow`,
`rect`, `ellipse`, `circle`, `polygon`, `region`, `brace`, `bracket`, `path`,
`fn-curve`) in a plain coordinate space and returns ordinary content —
embeddable anywhere, no CeTZ dependency. Pass `origin: "bottom-left"` for
y-up, math-convention plots.

Annotate content by name — a pin marks a spot, `annotate` draws a mark
anchored to it, called after the pin(s) in flow order on the same page:

```typst
#import "@preview/chalks:0.1.0": annotate, pin

The key #pin("idea")[idea] deserves a ring.
#annotate(circle: "idea")
```

`annotate` also takes `underline:`, `box:`, and `arrow: (from, to)`.

`annotate` places its mark in page coordinates, so call it — like the `pin`s
it references — directly in top-level page flow, not nested inside a
`grid`/`stack`/`table` cell; those containers give `place()` their own local
frame, which throws off `annotate`'s page-relative math (the mark can render
clipped away or offset from its pin).

## Style keys

Every shape/stroke/fill call accepts style overrides as named arguments
(`roughness: 1.5`, `fill: "scribble"`, `seed: 42`, …); unset keys fall back to
the active theme, then to these defaults:

| Key          | Applies to    | Meaning                                                         | Default |
|--------------|---------------|------------------------------------------------------------------|---------|
| `smoothness` | stroke + fill | 0 = sharp polyline corners, 1 = fully flowing curve             | 0.7     |
| `roughness`  | stroke + fill | amplitude of jitter/bowing relative to size                     | 1.0     |
| `width`      | stroke + fill | nominal stroke width (pt)                                       | 1.2     |
| `taper`      | stroke        | pressure variation 0-1 (0 = uniform, 1 = strong taper at ends)   | 0.5     |
| `passes`     | stroke        | number of overlapping strokes (1 = single, 2 = sketchy double)   | 1       |
| `pattern`    | fill          | `hachure` \| `scribble` \| `shade`                               | hachure |
| `angle`      | fill          | hachure/scribble direction (deg)                                 | 45      |
| `spacing`    | fill          | gap between doodle lines (pt)                                    | 4       |
| `color`      | stroke + fill | fill/stroke color                                                 | `#44464a` |
| `opacity`    | stroke + fill | overall opacity                                                   | 100%    |

Seeds are derived deterministically from input geometry by default, so
unchanged figures never re-roll between compiles.

## Themes

Document-wide presets, set with `#chalks-theme(<preset>)`:

- `pencil` — the default look (graphite gray, textured, tapered); the empty
  overlay.
- `ink` — darker, single-pass, crisper.
- `chalk` — light-on-dark, wider, softer; for dark backgrounds/slides.

## Raw engine access

`raw-stroke(points, closed:, style:, seed:)` and
`raw-fill(boundaries, style:, seed:)` call the engine directly on explicit
point lists / boundary rings, bypassing the shape builders.

## Rebuilding the engine

`plugin/chalks_engine.wasm` is a prebuilt artifact. Rebuild it with the
pinned Rust toolchain (see `rust-toolchain.toml` at the repo root) via:

```sh
make plugin
```

run from the repo root (or `make plugin` inside `chalks/`, which delegates
to the same recipe).

## Development

From the repo root (needed so `@preview/chalks:0.1.0` resolves for examples):

```sh
make test      # compile tests + manual, run error-message assertions
make examples  # compile chalks/examples/*.typ via @preview/chalks:0.1.0
```

See `manual.typ` for a rendered walkthrough of the API (every snippet shown
is compiled, not just illustrative), and `examples/` for complete figures:
`gallery.typ` (every primitive + fill), `annotated-equation.typ` (pin/annotate
on math), `chalkboard.typ` (chalk theme on a dark page).
