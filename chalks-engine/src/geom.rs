pub type Pt = [f64; 2];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CubicSeg {
    pub c1: Pt,
    pub c2: Pt,
    pub to: Pt,
}

pub fn add(a: Pt, b: Pt) -> Pt {
    [a[0] + b[0], a[1] + b[1]]
}
pub fn sub(a: Pt, b: Pt) -> Pt {
    [a[0] - b[0], a[1] - b[1]]
}
pub fn mul(a: Pt, s: f64) -> Pt {
    [a[0] * s, a[1] * s]
}
pub fn norm(a: Pt) -> f64 {
    (a[0] * a[0] + a[1] * a[1]).sqrt()
}
pub fn dist(a: Pt, b: Pt) -> f64 {
    norm(sub(a, b))
}

/// Catmull-Rom through `pts` as cubic beziers; `smoothness` in [0,1] scales the
/// tangent handles (0 => straight polyline, 1 => full flow). Open curves use
/// one-sided tangents at the ends; closed curves wrap (one segment per point).
pub fn catmull_rom(pts: &[Pt], closed: bool, smoothness: f64) -> Vec<CubicSeg> {
    let n = pts.len();
    if n < 2 {
        return Vec::new();
    }
    let tangent = |i: usize| -> Pt {
        if closed {
            let prev = pts[(i + n - 1) % n];
            let next = pts[(i + 1) % n];
            mul(sub(next, prev), smoothness / 6.0)
        } else if i == 0 {
            mul(sub(pts[1], pts[0]), smoothness / 3.0)
        } else if i == n - 1 {
            mul(sub(pts[n - 1], pts[n - 2]), smoothness / 3.0)
        } else {
            mul(sub(pts[i + 1], pts[i - 1]), smoothness / 6.0)
        }
    };
    let segs = if closed { n } else { n - 1 };
    (0..segs)
        .map(|i| {
            let j = (i + 1) % n;
            CubicSeg {
                c1: add(pts[i], tangent(i)),
                c2: sub(pts[j], tangent(j)),
                to: pts[j],
            }
        })
        .collect()
}

fn eval_cubic(p0: Pt, s: &CubicSeg, t: f64) -> Pt {
    let u = 1.0 - t;
    let (a, b, c, d) = (u * u * u, 3.0 * u * u * t, 3.0 * u * t * t, t * t * t);
    [
        a * p0[0] + b * s.c1[0] + c * s.c2[0] + d * s.to[0],
        a * p0[1] + b * s.c1[1] + c * s.c2[1] + d * s.to[1],
    ]
}

/// Flatten segments to a polyline with roughly `step` pt between samples.
/// Includes the start point; each segment contributes its interior + end.
pub fn sample(start: Pt, segs: &[CubicSeg], step: f64) -> Vec<Pt> {
    let mut out = vec![start];
    let mut from = start;
    for seg in segs {
        let chord = dist(from, seg.to) + dist(from, seg.c1) + dist(seg.c2, seg.to);
        let n = ((chord / step).ceil() as usize).clamp(2, 128);
        for k in 1..=n {
            out.push(eval_cubic(from, seg, k as f64 / n as f64));
        }
        from = seg.to;
    }
    out
}

/// Cumulative arc length along a polyline; cumlen[0] == 0.
pub fn cumlen(pts: &[Pt]) -> Vec<f64> {
    let mut out = Vec::with_capacity(pts.len());
    let mut acc = 0.0;
    out.push(0.0);
    for w in pts.windows(2) {
        acc += dist(w[0], w[1]);
        out.push(acc);
    }
    out
}

/// Unit normals from central-difference tangents (one-sided at the ends).
pub fn normals(pts: &[Pt]) -> Vec<Pt> {
    let n = pts.len();
    (0..n)
        .map(|i| {
            let t = sub(pts[(i + 1).min(n - 1)], pts[i.max(1) - 1]);
            let l = norm(t).max(1e-12);
            [-t[1] / l, t[0] / l]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_smoothness_degenerates_to_straight_segments() {
        let pts = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]];
        let segs = catmull_rom(&pts, false, 0.0);
        assert_eq!(segs.len(), 2);
        // Handles collapse onto the endpoints -> straight lines.
        assert_eq!(segs[0].c1, [0.0, 0.0]);
        assert_eq!(segs[0].c2, [10.0, 0.0]);
        assert_eq!(segs[0].to, [10.0, 0.0]);
    }

    #[test]
    fn full_smoothness_bends_the_interior_joint() {
        let pts = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0]];
        let segs = catmull_rom(&pts, false, 1.0);
        // Interior tangent at p1 is (p2 - p0)/6: segment 0's c2 pulls off the corner.
        assert!(segs[0].c2 != [10.0, 0.0]);
    }

    #[test]
    fn closed_curve_wraps_and_has_one_seg_per_point() {
        let pts = [[0.0, 0.0], [10.0, 0.0], [10.0, 10.0], [0.0, 10.0]];
        assert_eq!(catmull_rom(&pts, true, 1.0).len(), 4);
        assert_eq!(catmull_rom(&pts, false, 1.0).len(), 3);
    }

    #[test]
    fn sample_hits_both_ends_and_is_dense_enough() {
        let pts = [[0.0, 0.0], [100.0, 0.0]];
        let segs = catmull_rom(&pts, false, 0.5);
        let s = sample(pts[0], &segs, 2.0);
        assert_eq!(s.first().unwrap(), &[0.0, 0.0]);
        assert!(dist(*s.last().unwrap(), [100.0, 0.0]) < 1e-9);
        assert!(
            s.len() >= 40,
            "100pt chord at step 2 must give >= 40 samples, got {}",
            s.len()
        );
    }

    #[test]
    fn normals_are_unit_and_perpendicular_on_a_straight_line() {
        let pts: Vec<Pt> = (0..10).map(|i| [i as f64, 0.0]).collect();
        for n in normals(&pts) {
            assert!((norm(n) - 1.0).abs() < 1e-9);
            assert!(n[0].abs() < 1e-9); // perpendicular to +x
        }
    }
}
