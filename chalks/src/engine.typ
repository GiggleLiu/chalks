// WASM bridge: CBOR in/out to chalks-engine. Styling stays in Typst.
#import "style.typ": auto-seed, engine-fill-style, engine-stroke-style, resolve-style

#let _engine = plugin("../plugin/chalks_engine.wasm")

/// Plugin version string (smoke check that the binary loads).
#let engine-version() = str(_engine.version())

// Round to 1e-6 pt: calc.sin/cos/exp go through the platform math library,
// whose last-ULP results vary across OSes; unrounded they would re-roll
// auto-seed and make committed example images irreproducible in CI.
#let _pt(p) = (
  calc.round(float(p.at(0)), digits: 6),
  calc.round(float(p.at(1)), digits: 6),
)

/// Engine paths -> filled curve elements, placed at (0,0) of the caller's
/// frame. weight scales opacity (shade layering).
#let render-paths(paths, color, opacity) = {
  for p in paths {
    let elems = ()
    for sp in p.subpaths {
      elems.push(curve.move((sp.start.at(0) * 1pt, sp.start.at(1) * 1pt)))
      for c in sp.cubics {
        elems.push(curve.cubic(
          (c.at(0).at(0) * 1pt, c.at(0).at(1) * 1pt),
          (c.at(1).at(0) * 1pt, c.at(1).at(1) * 1pt),
          (c.at(2).at(0) * 1pt, c.at(2).at(1) * 1pt),
        ))
      }
      elems.push(curve.close())
    }
    let alpha = calc.max(0%, 100% - opacity * p.weight)
    place(top + left, curve(
      fill: color.transparentize(alpha),
      fill-rule: "even-odd",
      ..elems,
    ))
  }
}

/// Low-level hand-drawn stroke through `points` ((x, y) floats, pt, y-down).
#let raw-stroke(points, closed: false, style: (:), seed: auto) = context {
  let s = resolve-style(style)
  let pts = points.map(_pt)
  let seed = if seed == auto { auto-seed(("stroke", pts, closed)) } else { seed }
  let req = cbor.encode((
    points: pts,
    closed: closed,
    style: engine-stroke-style(s),
    seed: seed,
  ))
  render-paths(cbor(_engine.stroke(req)).paths, s.color, s.opacity)
}

/// Low-level doodle fill of closed boundary rings (even-odd: nested = hole).
#let raw-fill(boundaries, style: (:), seed: auto) = context {
  let s = resolve-style(style)
  let bs = boundaries.map(b => b.map(_pt))
  let seed = if seed == auto { auto-seed(("fill", bs)) } else { seed }
  let req = cbor.encode((
    boundaries: bs,
    style: engine-fill-style(s),
    seed: seed,
  ))
  render-paths(cbor(_engine.fill(req)).paths, s.color, s.opacity)
}
