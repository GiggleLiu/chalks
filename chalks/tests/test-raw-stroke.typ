#import "@preview/chalks:0.1.0": raw-stroke, raw-fill
#set page(width: 200pt, height: 240pt, margin: 10pt)

// A wavy open stroke, a closed square stroke, and all three fill patterns.
#box(width: 180pt, height: 60pt)[
  #raw-stroke(((0, 30), (60, 5), (120, 45), (180, 20)), style: (width: 2.0))
]
#box(width: 180pt, height: 70pt)[
  #raw-stroke(((10, 10), (120, 10), (120, 60), (10, 60)), closed: true,
    style: (smoothness: 0.15))
  #raw-fill((((10, 10), (120, 10), (120, 60), (10, 60)),),
    style: (pattern: "hachure", smoothness: 0.15, color: rgb("#7a7d85")))
]
#box(width: 180pt, height: 70pt)[
  #raw-fill((((10, 10), (80, 10), (80, 60), (10, 60)),), style: (pattern: "scribble"))
  #raw-fill((((95, 10), (165, 10), (165, 60), (95, 60)),), style: (pattern: "shade"))
]
Raw bridge OK.
