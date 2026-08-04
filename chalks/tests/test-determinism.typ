// Determinism: same request bytes -> identical engine output bytes;
// different geometry -> different output. Bypasses context/theme by
// calling the plugin directly.
#import "../src/engine.typ": _engine
#import "../src/style.typ": auto-seed, default-style, engine-stroke-style

#let req(points) = cbor.encode((
  points: points.map(p => (float(p.at(0)), float(p.at(1)))),
  closed: false,
  style: engine-stroke-style(default-style),
  seed: auto-seed(("stroke", points, false)),
))
#let p1 = ((0.0, 0.0), (50.0, 20.0), (100.0, 0.0))
#let p2 = ((0.0, 0.0), (50.0, 21.0), (100.0, 0.0))
#let a = _engine.stroke(req(p1))
#let b = _engine.stroke(req(p1))
#let c = _engine.stroke(req(p2))
#assert.eq(a, b)
#assert(a != c)
Deterministic.
