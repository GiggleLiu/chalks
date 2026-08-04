#import "../lib.typ": arrow, brace, circle, fn-curve, line, rect, sketch
#set page(width: 260pt, height: 320pt, margin: 10pt)

#sketch(240pt, 140pt,
  rect((10, 10), (90, 60), fill: "hachure"),
  circle((160, 45), 30, fill: "scribble", color: rgb("#8a4a3a")),
  arrow((105, 40), (128, 40)),
  brace((10, 80), (100, 80), amplitude: 10),
)

// bottom-left origin: a parabola opening upward must render upward.
#sketch(240pt, 140pt, origin: "bottom-left",
  line((10, 10), (230, 10)),
  line((10, 10), (10, 130)),
  fn-curve(x => 10 + (x - 120) * (x - 120) / 150, (20, 220), samples: 24),
)
Sketch OK.
