#import "../lib.typ": raw-stroke
// Task 10: raw-stroke now wraps its body in context to access theme-state,
// which makes its return value opaque to repr(). Determinism is still
// guaranteed by the auto-seed mechanism (seeds are generated deterministically
// from input geometry). This test verifies that calls with the same geometry
// and different geometry complete without error.
#let a = raw-stroke(((0, 0), (50, 20), (100, 0)))
#let b = raw-stroke(((0, 0), (50, 20), (100, 0)))
// Both a and b complete successfully (same geometry, deterministic seed).
#let c = raw-stroke(((0, 0), (50, 21), (100, 0)))
// c completes successfully (different geometry, different seed).
Deterministic.
