#import "@preview/chalks:0.1.0": arrow, brace, bracket, circle, ellipse, fn-curve, line, path, polygon, rect, region

#let ops = line((0, 0), (10, 10), roughness: 2.0)
#assert.eq(ops.len(), 1)
#assert.eq(ops.first().op, "stroke")
#assert.eq(ops.first().style.roughness, 2.0)
#assert(not ops.first().closed)

#assert.eq(arrow((0, 0), (50, 0)).len(), 2) // shaft + head

#let curved = arrow((0, 0), (100, 0), via: ((30, -20), (70, -20)))
#assert.eq(curved.len(), 2) // shaft + head
#assert.eq(curved.first().points.len(), 4) // from + 2 waypoints + to
#assert.eq(arrow((0, 0), (100, 0), via: (50, -20)).first().points.len(), 3) // bare (x, y) waypoint
#let wings = curved.last().points
#assert.eq(wings.at(1), (100.0, 0.0)) // head tip stays at `to`
// Head follows the direction of arrival (last leg rises from (70, -20)),
// so its base midpoint sits off the from->to chord.
#let back-mid-y = (wings.first().at(1) + wings.last().at(1)) / 2
#assert(back-mid-y < -1.0)
#let r = rect((0, 0), (40, 30), fill: "hachure")
#assert.eq(r.len(), 2)
#assert.eq(r.first().op, "fill")
#assert.eq(r.first().style.pattern, "hachure")
#assert.eq(r.last().op, "stroke")
#assert(r.last().closed)
#assert(r.last().style.smoothness < 0.3) // rects keep sharp corners by default

#let e = ellipse((0, 0), (20, 10))
#assert.eq(e.first().points.len(), 12)
#assert(e.first().closed)
#assert.eq(circle((0, 0), 10).first().points.len(), 12)

#assert.eq(brace((0, 0), (60, 0)).len(), 2) // two strokes meeting at the cusp
#let horizontal-bracket = bracket((0, 0), (40, 0), tick: 6).first().points
#assert.eq(horizontal-bracket, ((0.0, -6.0), (0.0, 0.0), (40.0, 0.0), (40.0, -6.0)))
#let vertical-bracket = bracket((0, 0), (0, 40), tick: 6).first().points
#assert.eq(vertical-bracket, ((6.0, 0.0), (0.0, 0.0), (0.0, 40.0), (6.0, 40.0)))
#assert.eq(region((((0, 0), (10, 0), (10, 10)),), pattern: "shade").first().style.pattern, "shade")

#let fc = fn-curve(x => x * x / 100, (0, 100), samples: 20)
#assert.eq(fc.first().points.len(), 21)
#assert.eq(fc.first().points.at(10).at(1), 25.0)

#let pg = polygon(((0, 0), (10, 0), (5, 8)))
#assert(pg.first().closed)
#assert.eq(path(((0, 0), (10, 0), (5, 8))).first().closed, false)
Shape builders OK.
