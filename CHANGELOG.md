# Changelog

## 0.1.1

- Add curved pin-to-pin arrows with `annotate(arrow: ..., bend: ...)` and
  optional content labels. Arrow endpoints now meet the padded pin boundaries.
- Fix annotation margins, padding, and offsets expressed in `em` on Typst 0.15.
- Support contextual sketch dimensions and use actual layout coordinates for
  annotation origins, including percentage margins and local font-size changes.
- Resolve pins on the annotation's page and reject duplicate names on that page.
- Include curved arrows with `via:` waypoints, previously available only in the checkout.
- Reject non-finite geometry and style values, excessive stroke passes, and fills
  that exceed work limits instead of hanging or exhausting memory.
- Add annotation placement assertions and test on Typst 0.14.2 and 0.15.1.
- Remove unused rendering offsets and update package imports to 0.1.1.

## 0.1.0

Initial Typst Universe release with sketch canvases, shape and curve builders,
pin-based annotations, pencil/ink/chalk themes, and the bundled WASM engine.
