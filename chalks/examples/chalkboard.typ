#import "@preview/chalks:0.1.0": *
#set page(width: 360pt, height: 240pt, margin: 20pt, fill: rgb("#2d3136"))
#set text(fill: rgb("#e8e6df"))
#chalks-theme(chalk)

= A chalkboard proof sketch

Area of the #pin("circ")[circle] grows as $r^2$:

#sketch(320pt, 120pt,
  circle((70, 60), 45, fill: "hachure", spacing: 7.0),
  arrow((70, 60), (112, 45)),
  fn-curve(x => 100 - (x - 160) * (x - 160) / 260, (160, 310), samples: 24),
)

#annotate(circle: "circ")
