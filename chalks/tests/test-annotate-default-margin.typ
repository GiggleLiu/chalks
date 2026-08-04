// Regression: page.margin is `auto` unless a document sets margins
// explicitly, which is the common case. annotate() must not panic on it.
#import "../lib.typ": annotate, pin
#set page(width: 320pt, height: 240pt)

No explicit margin: #pin("x")[this word] should still get ringed.

#annotate(circle: "x")
Default-margin annotation OK.
