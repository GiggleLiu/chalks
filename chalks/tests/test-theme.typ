#import "@preview/chalks:0.1.0": chalk, chalks-theme, ink, line, pencil, sketch
#set page(width: 260pt, height: 300pt, margin: 10pt, fill: rgb("#2d3136"))

#chalks-theme(chalk)
// Chalk on a dark board: wide, light strokes.
#sketch(240pt, 60pt, line((10, 30), (230, 30)))

#chalks-theme(ink)
// Per-call override beats the theme.
#sketch(240pt, 60pt, line((10, 30), (230, 30), color: rgb("#d0d3da")))

#assert.eq(chalk.color, rgb("#f2f0e9"))
#assert.eq(ink.taper, 0.15)
#assert.eq(pencil, (:)) // pencil is the no-op theme: defaults already are pencil
Themes OK.
