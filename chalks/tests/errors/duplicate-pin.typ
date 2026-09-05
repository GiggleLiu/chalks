// expected: chalks: duplicate pin on this page: same
#import "@preview/chalks:0.1.1": annotate, pin
#pin("same")[first] and #pin("same")[second].
#annotate(circle: "same")
