// expected: chalks: unknown pin: nope
#import "@preview/chalks:0.1.0": annotate, pin
#pin("yes")[hello]
#annotate(circle: "nope")
