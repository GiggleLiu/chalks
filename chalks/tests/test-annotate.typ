#import "../lib.typ": annotate, pin
#set page(width: 320pt, height: 240pt, margin: 16pt)

The energy $E = #pin("mc2")[$m c^2$]$ hides a #pin("deep")[deep idea]:
mass #pin("a")[is] energy #pin("b")[itself].

#annotate(circle: "mc2", color: rgb("#a03b2e"))
#annotate(underline: "deep")
#annotate(box: "a", pad: 4pt)
#annotate(arrow: ("a", "b"))
Annotations OK.
