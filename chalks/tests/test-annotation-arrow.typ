#import "@preview/chalks:0.1.1": annotate, pin
#import "../src/annotate.typ": _arrow-geometry
#import "../src/shapes.typ": arrow

#let a = (x: 0, y: 0, w: 20, h: 10)
#let b = (x: 100, y: 0, w: 20, h: 10)
#let straight = _arrow-geometry(a, b, 3, 0)
#assert.eq(straight.from, (23.0, 5.0))
#assert.eq(straight.to, (97.0, 5.0))
#for bend in (-20, 20) {
  let g = _arrow-geometry(a, b, 3, bend)
  assert.eq(g.mid, (60.0, 5.0 + bend))
  assert.eq(g.normal, (0.0, if bend < 0 { -1.0 } else { 1.0 }))
  assert.eq(g.from.at(0), 23.0)
  assert.eq(g.to.at(0), 97.0)
  let shaft = arrow(g.from, g.to, via: g.mid).first()
  assert.eq(shaft.points, (g.from, g.mid, g.to))
}
// Vertical and diagonal arrows also stop at the padded box boundary.
#let vertical = _arrow-geometry(a, (x: 0, y: 100, w: 20, h: 10), 3, 0)
#assert.eq(vertical.from, (10.0, 13.0))
#assert.eq(vertical.to, (10.0, 97.0))
#let diagonal = _arrow-geometry(a, (x: 100, y: 100, w: 20, h: 10), 3, 0)
#assert.eq(diagonal.from, (18.0, 13.0))
#assert.eq(diagonal.to, (102.0, 97.0))

// The waypoint must stay in the gap when one pinned expression is much wider.
#let unequal = _arrow-geometry((x: 0, y: 0, w: 200, h: 20),
  (x: 220, y: 0, w: 20, h: 20), 3, 5)
#assert.eq(unequal.mid, (210.0, 15.0))
#assert.eq(unequal.from.at(0), 203.0)
#assert.eq(unequal.to.at(0), 217.0)
#assert(unequal.from.at(0) < unequal.mid.at(0) and unequal.mid.at(0) < unequal.to.at(0))

#set page(width: 300pt, height: 220pt, margin: 20pt)
#set text(size: 10pt)
#pin("a", box(width: 20pt, height: 10pt))#h(80pt)#pin("b", box(width: 20pt, height: 10pt))
#annotate(arrow: ("a", "b"), bend: 2em, label: box(width: 20pt, height: 10pt)[#metadata("below") <arrow-label>])
#annotate(arrow: ("a", "b"), bend: -2em, label: [above])
#annotate(arrow: ("a", "b"), label: [straight])
#context {
  let marker = query(<arrow-label>).first()
  let pos = marker.location().position()
  // Arrow midpoint is (80pt,45pt) on the page; label top-left is (70pt,48pt).
  assert(calc.abs(pos.x.pt() - 70) < 0.001)
  assert(pos.y >= 48pt and pos.y <= 58pt)
}
