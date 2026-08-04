// Function-sampled curves.
#import "shapes.typ": path

/// Sample `f` over `domain = (lo, hi)` into a hand-drawn curve.
/// `f(x)` may return a float (plotted as (x, f(x))) or an (x, y) pair
/// (parametric). Note page coordinates are y-down; put fn-curve inside
/// `sketch(origin: "bottom-left")` for math-convention plots.
#let fn-curve(f, domain, samples: 32, ..style) = {
  let (lo, hi) = (float(domain.at(0)), float(domain.at(1)))
  let pts = range(samples + 1).map(i => {
    let x = lo + (hi - lo) * i / samples
    let y = f(x)
    if type(y) == array { y } else { (x, float(y)) }
  })
  path(pts, ..style)
}
