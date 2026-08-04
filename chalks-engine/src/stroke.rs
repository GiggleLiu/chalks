use crate::geom::{add, catmull_rom, cumlen, dist, mul, normals, sample, Pt};
use crate::rng::Rng;
use crate::schema::{Path, StrokeStyle, Subpath};

const SAMPLE_STEP: f64 = 2.0; // pt between perturbation samples

/// One hand-drawn stroke through `points` -> `passes` filled-outline paths.
pub fn run(points: &[Pt], closed: bool, style: &StrokeStyle, rng: &mut Rng) -> Vec<Path> {
    let segs = catmull_rom(points, closed, style.smoothness);
    let mut base = sample(points[0], &segs, SAMPLE_STEP);
    // For closed curves, sample() appends the wrap-around duplicate; drop it before perturbation
    // to avoid the seam jitter creating a near-duplicate kink.
    if closed && base.len() > 1 {
        base.pop();
    }
    (0..style.passes)
        .map(|pass| {
            let fade = if pass == 0 { 1.0 } else { 0.55 };
            let amp = if pass == 0 { 1.0 } else { 0.8 };
            Path {
                subpaths: outline(&perturb(&base, style, amp, rng), style, closed, rng),
                weight: fade,
            }
        })
        .collect()
}

/// Jitter + low-frequency bowing along the sampled spine.
fn perturb(base: &[Pt], style: &StrokeStyle, amp: f64, rng: &mut Rng) -> Vec<Pt> {
    let ns = normals(base);
    let s = cumlen(base);
    let total = *s.last().unwrap_or(&1.0);
    let bow_amp = amp * style.roughness * (0.35 + total * 0.008).min(2.5);
    let jitter = amp * style.roughness * 0.45;
    let k = rng.range(0.8, 1.6);
    let phase = rng.range(0.0, std::f64::consts::PI);
    base.iter()
        .enumerate()
        .map(|(i, p)| {
            let u = s[i] / total.max(1e-9);
            let bow = bow_amp * (std::f64::consts::PI * u * k + phase).sin();
            let along = jitter * 0.5 * rng.tri();
            let across = bow + jitter * rng.tri();
            let t = [ns[i][1], -ns[i][0]]; // unit tangent
            add(add(*p, mul(ns[i], across)), mul(t, along))
        })
        .collect()
}

/// Variable-width filled outline around the perturbed spine.
/// Open stroke -> single ring (left side out, right side back).
/// Closed stroke -> annulus: outer ring + inner ring (even-odd fill).
fn outline(spine: &[Pt], style: &StrokeStyle, closed: bool, rng: &mut Rng) -> Vec<Subpath> {
    let ns = normals(spine);
    let s = cumlen(spine);
    let total = *s.last().unwrap_or(&1.0);
    let width_at = |i: usize, rng: &mut Rng| -> f64 {
        let u = s[i] / total.max(1e-9);
        let envelope = if closed {
            1.0
        } else {
            (1.0 - style.taper) + style.taper * (std::f64::consts::PI * u).sin()
        };
        let noise = 1.0 + 0.3 * style.roughness * rng.tri();
        (style.width * envelope * noise).max(0.15 * style.width)
    };
    let mut left = Vec::with_capacity(spine.len());
    let mut right = Vec::with_capacity(spine.len());
    for i in 0..spine.len() {
        let hw = width_at(i, rng) / 2.0;
        left.push(add(spine[i], mul(ns[i], hw)));
        right.push(add(spine[i], mul(ns[i], -hw)));
    }
    if closed {
        vec![ring(&left), ring(&right)]
    } else {
        right.reverse();
        left.extend(right);
        vec![ring(&left)]
    }
}

/// Smooth a point ring into one closed cubic subpath.
fn ring(pts: &[Pt]) -> Subpath {
    let segs = catmull_rom(pts, true, 0.9);
    Subpath {
        start: pts[0],
        cubics: segs.iter().map(|c| [c.c1, c.c2, c.to]).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;
    use crate::schema::StrokeStyle;

    fn style() -> StrokeStyle {
        StrokeStyle { smoothness: 0.7, roughness: 1.0, width: 1.2, taper: 0.5, passes: 1 }
    }

    #[test]
    fn open_stroke_yields_one_closed_outline_per_pass() {
        let pts = [[0.0, 0.0], [60.0, 10.0], [120.0, 0.0]];
        let paths = run(&pts, false, &style(), &mut Rng::new(7));
        assert_eq!(paths.len(), 1);
        assert_eq!(paths[0].subpaths.len(), 1);
        assert!(paths[0].subpaths[0].cubics.len() >= 8, "outline must be a real curve");
        assert_eq!(paths[0].weight, 1.0);

        let mut s2 = style();
        s2.passes = 2;
        let paths = run(&pts, false, &s2, &mut Rng::new(7));
        assert_eq!(paths.len(), 2);
        assert!(paths[1].weight < 1.0, "extra passes are lighter");
    }

    #[test]
    fn closed_stroke_yields_outer_and_inner_ring() {
        let pts = [[0.0, 0.0], [80.0, 0.0], [80.0, 60.0], [0.0, 60.0]];
        let paths = run(&pts, true, &style(), &mut Rng::new(7));
        assert_eq!(paths[0].subpaths.len(), 2, "closed stroke outline is an annulus");
    }

    #[test]
    fn deterministic_and_seed_sensitive() {
        let pts = [[0.0, 0.0], [100.0, 20.0]];
        let a = run(&pts, false, &style(), &mut Rng::new(1));
        let b = run(&pts, false, &style(), &mut Rng::new(1));
        let c = run(&pts, false, &style(), &mut Rng::new(2));
        assert_eq!(a, b);
        assert_ne!(a, c);
    }

    #[test]
    fn outline_stays_near_the_input_within_width_plus_jitter() {
        let pts = [[0.0, 0.0], [100.0, 0.0]];
        let s = style();
        let paths = run(&pts, false, &s, &mut Rng::new(3));
        let slack = s.width / 2.0 + 3.0 * s.roughness + 1.0;
        for sp in &paths[0].subpaths {
            for c in &sp.cubics {
                for p in c {
                    assert!(p[1].abs() <= slack, "point {:?} strays past slack {}", p, slack);
                    assert!(p[0] >= -slack && p[0] <= 100.0 + slack);
                }
            }
        }
    }

    #[test]
    fn closed_stroke_seam_has_no_wrap_duplicate_vertex() {
        // Regression test: after dropping the wrap-duplicate before perturbation,
        // a closed stroke ring should have no artificially-short segment at the seam.
        // For closed curves, the number of cubics should be N (one per point),
        // and the wrap segment (last cubic, from last to first point) should have
        // similar magnitude to other segments (not a tiny near-duplicate seam).
        let pts = [[0.0, 0.0], [80.0, 0.0], [80.0, 60.0], [0.0, 60.0]];
        let paths = run(&pts, true, &style(), &mut Rng::new(7));
        assert_eq!(paths[0].subpaths.len(), 2);

        for sp in &paths[0].subpaths {
            // For a closed curve, catmull_rom produces one cubic per point.
            // So cubics.len() should equal the number of ring vertices.
            let n_cubics = sp.cubics.len();
            assert!(n_cubics > 0);

            // Extract the control points to verify no seam duplicate.
            // Each cubic is [c1, c2, to]; the path closes from the last 'to' back to 'start'.
            let mut vertices = vec![sp.start];
            for c in &sp.cubics {
                vertices.push(c[2]); // Each cubic's endpoint
            }

            // For a proper closed ring of N points, catmull_rom creates N cubics,
            // so vertices should have first == last (the wrap-around).
            if vertices.len() > 2 {
                let last = *vertices.last().unwrap();
                let first = vertices[0];
                assert!(
                    dist(last, first) < 2.0, // Last cubic ends near the first point (expected wrap).
                    "closed ring wrap segment distance {} should be smooth, not near zero (which signals a duplicate seam)",
                    dist(last, first)
                );

                // Check that the last cubic (seam segment) is not anomalously short.
                // Get the last two distinct vertices before the wrap.
                if vertices.len() > 2 {
                    let penultimate = vertices[vertices.len() - 2];
                    let last_seg_len = dist(penultimate, last);
                    let second_last_seg_len = dist(vertices[vertices.len() - 3], penultimate);
                    let ratio = last_seg_len / (second_last_seg_len + 1e-9);

                    // The last segment should be at least 30% the size of the previous (not a tiny duplicate).
                    assert!(
                        ratio >= 0.3,
                        "last segment length {} is only {:.1}% of previous {} (suggests seam duplicate)",
                        last_seg_len, ratio * 100.0, second_last_seg_len
                    );
                }
            }
        }
    }
}
