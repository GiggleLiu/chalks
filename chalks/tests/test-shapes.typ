#import "../lib.typ": arrow, brace, bracket, circle, ellipse, fn-curve, line, path, polygon, rect, region

#let ops = line((0, 0), (10, 10), roughness: 2.0)
#assert.eq(ops.len(), 1)
#assert.eq(ops.first().op, "stroke")
#assert.eq(ops.first().style.roughness, 2.0)
#assert(not ops.first().closed)

#assert.eq(arrow((0, 0), (50, 0)).len(), 2) // shaft + head
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
#assert.eq(bracket((0, 0), (0, 40)).first().points.len(), 4)
#assert.eq(region((((0, 0), (10, 0), (10, 10)),), pattern: "shade").first().style.pattern, "shade")

#let fc = fn-curve(x => x * x / 100, (0, 100), samples: 20)
#assert.eq(fc.first().points.len(), 21)
#assert.eq(fc.first().points.at(10).at(1), 25.0)

#let pg = polygon(((0, 0), (10, 0), (5, 8)))
#assert(pg.first().closed)
#assert.eq(path(((0, 0), (10, 0), (5, 8))).first().closed, false)
Shape builders OK.
