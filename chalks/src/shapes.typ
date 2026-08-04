// Shape builders: pure functions from geometry to op dicts. No engine calls
// here — `sketch` and `annotate` resolve styles and render. Adding a shape
// never touches Rust.
#import "style.typ": validate-style

#let _pt(p) = (float(p.at(0)), float(p.at(1)))
#let _sub(a, b) = (a.at(0) - b.at(0), a.at(1) - b.at(1))
#let _add(a, b) = (a.at(0) + b.at(0), a.at(1) + b.at(1))
#let _mul(a, s) = (a.at(0) * s, a.at(1) * s)
#let _len(a) = calc.sqrt(a.at(0) * a.at(0) + a.at(1) * a.at(1))

#let _stroke-op(points, closed, style) = {
  validate-style(style)
  ((op: "stroke", points: points.map(_pt), closed: closed, style: style),)
}

#let _fill-op(boundaries, style) = {
  validate-style(style)
  ((op: "fill", boundaries: boundaries.map(b => b.map(_pt)), style: style),)
}

#let line(a, b, ..style) = _stroke-op((a, b), false, style.named())

#let path(points, closed: false, ..style) = _stroke-op(points, closed, style.named())

#let polygon(points, fill: none, closed: true, ..style) = {
  let s = style.named()
  let ops = ()
  if fill != none { ops += _fill-op((points,), s + (pattern: fill)) }
  ops + _stroke-op(points, closed, s)
}

/// Sharp corners by default: a hand-drawn rectangle is four straight-ish
/// segments, so smoothness defaults low unless overridden.
#let rect(origin, size, fill: none, ..style) = {
  let (x, y) = _pt(origin)
  let (w, h) = _pt(size)
  let corners = ((x, y), (x + w, y), (x + w, y + h), (x, y + h))
  polygon(corners, fill: fill, ..((smoothness: 0.15) + style.named()))
}

#let ellipse(center, radii, fill: none, n: 12, ..style) = {
  let (cx, cy) = _pt(center)
  let (rx, ry) = _pt(radii)
  let pts = range(n).map(i => {
    let a = 360deg * i / n
    (cx + rx * calc.cos(a), cy + ry * calc.sin(a))
  })
  polygon(pts, fill: fill, ..((smoothness: 1.0) + style.named()))
}

#let circle(center, r, fill: none, ..style) = ellipse(center, (r, r), fill: fill, ..style)

#let arrow(from, to, head: 8, ..style) = {
  let s = style.named()
  let d = _sub(to, from)
  let l = calc.max(_len(d), 1e-6)
  let t = _mul(d, 1 / l)
  let n = (-t.at(1), t.at(0))
  let hl = calc.min(float(head), l * 0.4)
  let back = _sub(to, _mul(t, hl))
  let wing = _mul(n, hl * 0.45)
  _stroke-op((from, to), false, s) + _stroke-op(
    (_add(back, wing), to, _sub(back, wing)),
    false,
    (smoothness: 0.2) + s,
  )
}

/// Direct doodle-fill of explicit boundary rings (even-odd holes).
#let region(boundaries, pattern: "hachure", ..style) = _fill-op(
  boundaries,
  style.named() + (pattern: pattern),
)

/// Curly brace from `from` to `to`, bulging left of the from->to direction.
/// Drawn as two strokes meeting at the cusp — the way a hand draws it —
/// so the cusp stays sharp while each half flows.
#let brace(from, to, amplitude: 8, ..style) = {
  let s = (smoothness: 0.9) + style.named()
  let d = _sub(to, from)
  let l = calc.max(_len(d), 1e-6)
  let t = _mul(d, 1 / l)
  let n = (-t.at(1), t.at(0))
  let a = float(amplitude)
  let mid = _add(from, _mul(t, l / 2))
  let cusp = _add(mid, _mul(n, a))
  let half(p, dir) = (
    p,
    _add(_add(p, _mul(t, dir * 0.10 * l)), _mul(n, 0.55 * a)),
    _add(_add(mid, _mul(t, -dir * 0.08 * l)), _mul(n, 0.60 * a)),
    cusp,
  )
  _stroke-op(half(from, 1), false, s) + _stroke-op(half(to, -1), false, s)
}

/// Square bracket: end ticks point left of the from->to direction.
#let bracket(from, to, tick: 6, ..style) = {
  let s = (smoothness: 0.1) + style.named()
  let d = _sub(to, from)
  let l = calc.max(_len(d), 1e-6)
  let n = (-d.at(1) / l, d.at(0) / l)
  let k = _mul(n, float(tick))
  _stroke-op((_add(from, k), from, to, _add(to, k)), false, s)
}
