// WASM bridge: CBOR in/out to chalks-engine. Styling stays in Typst.
#import "style.typ": auto-seed, engine-fill-style, engine-stroke-style, resolve-style

#let _engine = plugin("../plugin/chalks_engine.wasm")

/// Plugin version string (smoke check that the binary loads).
#let engine-version() = str(_engine.version())

#let _pt(p) = (float(p.at(0)), float(p.at(1)))

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
#let raw-stroke(points, closed: false, style: (:), seed: auto) = {
  let s = resolve-style(style)
  let seed = if seed == auto { auto-seed(("stroke", points, closed)) } else { seed }
  let req = cbor.encode((
    points: points.map(_pt),
    closed: closed,
    style: engine-stroke-style(s),
    seed: seed,
  ))
  render-paths(cbor(_engine.stroke(req)).paths, s.color, s.opacity)
}

/// Low-level doodle fill of closed boundary rings (even-odd: nested = hole).
#let raw-fill(boundaries, style: (:), seed: auto) = {
  let s = resolve-style(style)
  let seed = if seed == auto { auto-seed(("fill", boundaries)) } else { seed }
  let req = cbor.encode((
    boundaries: boundaries.map(b => b.map(_pt)),
    style: engine-fill-style(s),
    seed: seed,
  ))
  render-paths(cbor(_engine.fill(req)).paths, s.color, s.opacity)
}
