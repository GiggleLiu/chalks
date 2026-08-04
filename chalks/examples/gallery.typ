#import "@preview/chalks:0.1.0": *
#set page(width: 420pt, height: auto, margin: 16pt)

= chalks gallery

#sketch(380pt, 200pt,
  rect((10, 10), (100, 70), fill: "hachure"),
  ellipse((190, 45), (45, 32), fill: "scribble", color: rgb("#8a4a3a")),
  circle((320, 45), 34, fill: "shade"),
  arrow((10, 110), (110, 110)),
  brace((140, 105), (250, 105), amplitude: 10),
  bracket((280, 100), (370, 100), tick: 8),
  polygon(((30, 140), (100, 150), (80, 190), (20, 180)), fill: "hachure", angle: -30.0),
  path(((140, 190), (180, 140), (230, 185), (280, 145), (330, 180)), roughness: 1.6),
)

= Function curves (y-up)

#sketch(380pt, 160pt, origin: "bottom-left",
  arrow((15, 15), (370, 15)),
  arrow((15, 15), (15, 150)),
  fn-curve(x => 20 + 100 * calc.exp(-calc.pow((x - 190) / 60, 2)), (30, 350), samples: 40),
  fn-curve(x => 20 + (x - 30) * (x - 30) / 900, (30, 350), samples: 30, color: rgb("#a03b2e")),
)

= Raw engine access

#box(width: 380pt, height: 90pt)[
  #raw-stroke(((10, 45), (100, 15), (200, 70), (290, 25), (370, 55)),
    style: (width: 3.0, taper: 0.8, passes: 2))
]
