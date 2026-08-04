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
            for pat in ["hachure", "shade"] {
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
    fn shade_layers_carry_reduced_weights() {
        let paths = run(&square(), &style("shade"), &mut Rng::new(5));
        assert!(paths.iter().all(|p| p.weight < 1.0));
        assert!(paths.len() > 12, "shade must layer multiple hachures");
    }

    #[test]
    fn every_pattern_leaves_holes_unfilled() {
        for pattern in ["hachure", "shade"] {
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
