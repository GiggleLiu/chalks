use crate::geom::{catmull_rom, sample, Pt};
use crate::rng::Rng;
use crate::schema::{FillStyle, Path, StrokeStyle};
use crate::stroke; // the stroke pipeline module (coexists with `fn stroke` in lib.rs)

/// Doodle-fill closed region(s). Boundaries are smoothed per `smoothness`
/// (same rule as stroke), then scanline-hachured in rotated space. Even-odd:
/// a boundary inside another is a hole.
pub fn run(boundaries: &[Vec<Pt>], style: &FillStyle, rng: &mut Rng) -> Vec<Path> {
    let polys: Vec<Vec<Pt>> = boundaries
        .iter()
        .map(|b| {
            let segs = catmull_rom(b, true, style.smoothness);
            sample(b[0], &segs, 2.0)
        })
        .collect();
    match style.pattern.as_str() {
        "scribble" => scribble(&polys, style, rng),
        "shade" => shade(&polys, style, rng),
        _ => hachure(&polys, style, style.angle, style.spacing, 1.0, rng),
    }
}

/// Doodle strokes reuse the stroke pipeline so fills share its texture.
fn doodle_style(style: &FillStyle, smoothness: f64) -> StrokeStyle {
    StrokeStyle {
        smoothness,
        roughness: style.roughness * 0.7, // hachure lines are calmer than outlines
        width: style.width,
        taper: 0.25,
        passes: 1,
    }
}

/// Horizontal scanline segments of `polys` (even-odd), in rotated space.
/// Returns rows of segments, one row per scanline, back-rotated to user space.
fn rows(
    polys: &[Vec<Pt>],
    angle_deg: f64,
    spacing: f64,
    rng: &mut Rng,
    rough: f64,
) -> Vec<Vec<[Pt; 2]>> {
    let a = angle_deg.to_radians();
    let (cs, sn) = (a.cos(), a.sin());
    let rot = |p: Pt| -> Pt { [p[0] * cs + p[1] * sn, -p[0] * sn + p[1] * cs] };
    let unrot = |p: Pt| -> Pt { [p[0] * cs - p[1] * sn, p[0] * sn + p[1] * cs] };
    let rp: Vec<Vec<Pt>> = polys
        .iter()
        .map(|poly| poly.iter().map(|&p| rot(p)).collect())
        .collect();
    let ys: Vec<f64> = rp.iter().flatten().map(|p| p[1]).collect();
    let (ymin, ymax) = ys
        .iter()
        .fold((f64::MAX, f64::MIN), |(lo, hi), &y| (lo.min(y), hi.max(y)));
    let mut out = Vec::new();
    let mut y = ymin + spacing * 0.6;
    while y < ymax {
        let yj = y + rng.tri() * 0.15 * spacing * rough;
        let mut xs: Vec<f64> = Vec::new();
        for poly in &rp {
            let n = poly.len();
            for i in 0..n {
                let (p, q) = (poly[i], poly[(i + 1) % n]);
                if (p[1] <= yj) != (q[1] <= yj) {
                    xs.push(p[0] + (yj - p[1]) / (q[1] - p[1]) * (q[0] - p[0]));
                }
            }
        }
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut row = Vec::new();
        for pair in xs.chunks_exact(2) {
            let inset = rng.range(0.0, 0.4 * spacing * rough);
            let (x0, x1) = (pair[0] + inset, pair[1] - inset);
            if x1 - x0 > spacing * 0.5 {
                row.push([unrot([x0, yj]), unrot([x1, yj])]);
            }
        }
        if !row.is_empty() {
            out.push(row);
        }
        y += spacing * rng.range(0.9, 1.1);
    }
    out
}

fn hachure(
    polys: &[Vec<Pt>],
    style: &FillStyle,
    angle: f64,
    spacing: f64,
    weight: f64,
    rng: &mut Rng,
) -> Vec<Path> {
    let mut paths = Vec::new();
    for (i, row) in rows(polys, angle, spacing, rng, style.roughness)
        .iter()
        .enumerate()
    {
        for seg in row {
            // Alternate direction row by row, like a hand sweeping back and forth.
            let pts = if i % 2 == 0 {
                [seg[0], seg[1]]
            } else {
                [seg[1], seg[0]]
            };
            for mut p in stroke::run(&pts, false, &doodle_style(style, 0.3), rng) {
                p.weight *= weight;
                paths.push(p);
            }
        }
    }
    paths
}

/// Even-odd point-in-polygon test across all boundary rings.
fn contains(polys: &[Vec<Pt>], p: Pt) -> bool {
    let mut inside = false;
    for poly in polys {
        for i in 0..poly.len() {
            let (a, b) = (poly[i], poly[(i + 1) % poly.len()]);
            if (a[1] > p[1]) != (b[1] > p[1])
                && p[0] < (b[0] - a[0]) * (p[1] - a[1]) / (b[1] - a[1]) + a[0]
            {
                inside = !inside;
            }
        }
    }
    inside
}

/// A short connector may join adjacent scan rows only when sampled points stay
/// inside the even-odd region. This prevents a scribble from cutting across a
/// hole or jumping between disconnected components.
fn connector_is_inside(polys: &[Vec<Pt>], from: Pt, to: Pt, spacing: f64) -> bool {
    let len = (to[0] - from[0]).hypot(to[1] - from[1]);
    if len > spacing * 2.5 {
        return false;
    }
    let samples = 8;
    (1..samples).all(|i| {
        let t = i as f64 / samples as f64;
        contains(
            polys,
            [
                from[0] * (1.0 - t) + to[0] * t,
                from[1] * (1.0 - t) + to[1] * t,
            ],
        )
    })
}

/// Serpentine doodles. A simple connected region is one continuous stroke;
/// holes and disconnected components split it into safe sub-strokes.
fn scribble(polys: &[Vec<Pt>], style: &FillStyle, rng: &mut Rng) -> Vec<Path> {
    let mut spines: Vec<Vec<Pt>> = vec![Vec::new()];
    for (i, row) in rows(polys, style.angle, style.spacing, rng, style.roughness)
        .iter()
        .enumerate()
    {
        let mut segs: Vec<[Pt; 2]> = row.clone();
        if i % 2 == 1 {
            segs.reverse();
            for s in &mut segs {
                s.swap(0, 1);
            }
        }
        for s in segs {
            // Densify segment with interior points to pin CR to the scanline and prevent overshoot
            let step = 2.0 * style.spacing;
            let [p0, p1] = s;
            let len = (p1[0] - p0[0]).hypot(p1[1] - p0[1]);
            let n_interior = ((len / step).floor() as usize).max(1);

            let spine = spines.last_mut().unwrap();
            if let Some(&last) = spine.last() {
                if !connector_is_inside(polys, last, p0, style.spacing) {
                    spines.push(Vec::new());
                }
            }
            let spine = spines.last_mut().unwrap();
            spine.push(p0);
            for k in 1..n_interior {
                let t = k as f64 / n_interior as f64;
                spine.push([p0[0] * (1.0 - t) + p1[0] * t, p0[1] * (1.0 - t) + p1[1] * t]);
            }
            spine.push(p1);
        }
    }
    let mut out = Vec::new();
    for spine in spines.into_iter().filter(|spine| spine.len() >= 2) {
        out.extend(stroke::run(&spine, false, &doodle_style(style, 0.85), rng));
    }
    out
}

/// Layered soft shading: three lighter hachure passes at drifting angles.
fn shade(polys: &[Vec<Pt>], style: &FillStyle, rng: &mut Rng) -> Vec<Path> {
    let layers = [(0.0, 0.55), (-8.0, 0.35), (6.0, 0.25)];
    let mut out = Vec::new();
    for (da, w) in layers {
        let angle = style.angle + da + rng.tri() * 2.0;
        out.extend(hachure(polys, style, angle, style.spacing * 1.3, w, rng));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::Rng;
    use crate::schema::FillStyle;

    fn square() -> Vec<Vec<[f64; 2]>> {
        vec![vec![[0.0, 0.0], [100.0, 0.0], [100.0, 100.0], [0.0, 100.0]]]
    }

    fn style(pattern: &str) -> FillStyle {
        FillStyle {
            smoothness: 0.0,
            roughness: 1.0,
            width: 1.2,
            pattern: pattern.into(),
            angle: 45.0,
            spacing: 6.0,
        }
    }

    #[test]
    fn hachure_covers_the_square_with_many_lines() {
        let paths = run(&square(), &style("hachure"), &mut Rng::new(5));
        // 100pt tall square at 45deg/6pt spacing -> on the order of 20 lines.
        assert!(paths.len() >= 12, "got only {} hachure paths", paths.len());
    }

    #[test]
    fn all_fill_output_stays_inside_an_inflated_bbox() {
        for seed in 0..32 {
            for pat in ["hachure", "scribble", "shade"] {
                let paths = run(&square(), &style(pat), &mut Rng::new(seed));
                assert!(!paths.is_empty(), "{pat} seed {seed} produced nothing");
                for p in &paths {
                    for sp in &p.subpaths {
                        for c in &sp.cubics {
                            for q in c {
                                assert!(
                                    q[0] > -8.0 && q[0] < 108.0 && q[1] > -8.0 && q[1] < 108.0,
                                    "{pat} seed {seed} point {q:?} escapes the region"
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn scribble_is_one_continuous_doodle() {
        let paths = run(&square(), &style("scribble"), &mut Rng::new(5));
        assert_eq!(paths.len(), 1, "scribble is a single stroke pass");
    }

    #[test]
    fn shade_layers_carry_reduced_weights() {
        let paths = run(&square(), &style("shade"), &mut Rng::new(5));
        assert!(paths.iter().all(|p| p.weight < 1.0));
        assert!(paths.len() > 12, "shade must layer multiple hachures");
    }

    #[test]
    fn every_pattern_leaves_holes_unfilled() {
        for pattern in ["hachure", "scribble", "shade"] {
            let mut b = square();
            b.push(vec![[40.0, 40.0], [60.0, 40.0], [60.0, 60.0], [40.0, 60.0]]); // hole (even-odd)
            let paths = run(&b, &style(pattern), &mut Rng::new(5));
            for p in &paths {
                for sp in &p.subpaths {
                    for c in &sp.cubics {
                        for q in c {
                            let inside_hole =
                                q[0] > 44.0 && q[0] < 56.0 && q[1] > 44.0 && q[1] < 56.0;
                            assert!(!inside_hole, "{pattern} doodle {q:?} crosses the hole");
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn deterministic_per_seed() {
        let a = run(&square(), &style("hachure"), &mut Rng::new(9));
        let b = run(&square(), &style("hachure"), &mut Rng::new(9));
        assert_eq!(a, b);
    }
}
