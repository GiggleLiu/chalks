#import "@preview/chalks:0.1.1": annotate, pin, sketch, line
#import "../src/annotate.typ": _pin-bbox

#set text(size: 10pt)
#set page(width: 300pt, height: 200pt, margin: (x: 5% + 1em, y: 10% + 2em))
#set par(first-line-indent: 0pt)

#h(20pt)#pin("repeat", box(width: 30pt, height: 12pt))
#place(top + left, context {
  let origin = here().position()
  assert.eq((origin.x, origin.y), (25pt, 40pt))
  let b = _pin-bbox("repeat", origin)
  assert(calc.abs(b.x - 20) < 0.001)
  assert(calc.abs(b.y) < 0.001)
  assert.eq((b.w, b.h), (30.0, 12.0))
})
#annotate(box: "repeat", pad: 0.5em, dx: 0.2em, dy: -0.1em)

#pagebreak()
#h(60pt)#pin("repeat", box(width: 70pt, height: 12pt))
#place(top + left, context {
  let b = _pin-bbox("repeat", here().position())
  assert(calc.abs(b.x - 60) < 0.001)
  assert(calc.abs(b.y) < 0.001)
  assert.eq((b.w, b.h), (70.0, 12.0))
})
#annotate(circle: "repeat", pad: 0.5em)

#context {
  let canvas = sketch(10em, 4em, origin: "bottom-left", line((0, 0), (80, 30)))
  assert.eq(measure(canvas), (width: 100pt, height: 40pt))
  canvas
}

#pagebreak()
#set page(margin: (top: 4.5em, bottom: 3.5em, x: 2.5em))
#pin("em", box(width: 30pt, height: 12pt))
#place(top + left, context {
  let origin = here().position()
  assert.eq((origin.x, origin.y), (25pt, 45pt))
  let b = _pin-bbox("em", origin)
  assert(calc.abs(b.x) < 0.001 and calc.abs(b.y) < 0.001)
})
#annotate(underline: "em", pad: 0.2em, dx: 0.1em, dy: 0.1em)

// Page margins keep their original font size when annotation text changes.
#text(size: 20pt)[
  #place(top + left, context {
    let origin = here().position()
    assert.eq((origin.x, origin.y), (25pt, 45pt))
    let b = _pin-bbox("em", origin)
    assert(calc.abs(b.x) < 0.001 and calc.abs(b.y) < 0.001)
  })
  #annotate(box: "em", pad: 0.2em)
]
