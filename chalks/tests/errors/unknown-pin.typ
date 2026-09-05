// expected: chalks: unknown pin: nope
#import "@preview/chalks:0.1.1": annotate, pin
#pin("yes")[hello]
#annotate(circle: "nope")
