// expected: chalks: arrow pins must have distinct positions
#import "@preview/chalks:0.1.1": annotate, pin
#pin("x")[word]
#annotate(arrow: ("x", "x"))
