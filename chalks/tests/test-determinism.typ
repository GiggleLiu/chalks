#import "../lib.typ": raw-stroke
#let a = raw-stroke(((0, 0), (50, 20), (100, 0)))
#let b = raw-stroke(((0, 0), (50, 20), (100, 0)))
#assert.eq(repr(a), repr(b))
#let c = raw-stroke(((0, 0), (50, 21), (100, 0)))
#assert.ne(repr(a), repr(c))
Deterministic.
