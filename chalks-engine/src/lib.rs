#[cfg(target_arch = "wasm32")]
use wasm_minimal_protocol::*;

#[cfg(target_arch = "wasm32")]
initiate_protocol!();

pub mod fill;
pub mod geom;
pub mod rng;
pub mod schema;
pub mod stroke;

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn version() -> Vec<u8> {
    format!("chalks-engine {}", env!("CARGO_PKG_VERSION")).into_bytes()
}

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn stroke(input: &[u8]) -> Result<Vec<u8>, String> {
    let req: schema::StrokeRequest = ciborium::from_reader(input)
        .map_err(|e| format!("chalks-engine: bad stroke request: {e}"))?;
    req.validate()?;
    let mut rng = crate::rng::Rng::new(req.seed);
    let resp = schema::Response {
        paths: stroke::run(&req.points, req.closed, &req.style, &mut rng),
    };
    let mut buf = Vec::new();
    ciborium::into_writer(&resp, &mut buf)
        .map_err(|e| format!("chalks-engine: encode failed: {e}"))?;
    Ok(buf)
}

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn fill(input: &[u8]) -> Result<Vec<u8>, String> {
    let req: schema::FillRequest = ciborium::from_reader(input)
        .map_err(|e| format!("chalks-engine: bad fill request: {e}"))?;
    req.validate()?;
    let mut rng = crate::rng::Rng::new(req.seed);
    let resp = schema::Response {
        paths: fill::run(&req.boundaries, &req.style, &mut rng)?,
    };
    let mut buf = Vec::new();
    ciborium::into_writer(&resp, &mut buf)
        .map_err(|e| format!("chalks-engine: encode failed: {e}"))?;
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_reports_crate_semver() {
        assert_eq!(version(), b"chalks-engine 0.1.1".to_vec());
    }

    #[test]
    fn stroke_entry_round_trips_cbor() {
        let req = schema::StrokeRequest {
            points: vec![[0.0, 0.0], [50.0, 10.0], [100.0, 0.0]],
            closed: false,
            style: Default::default(),
            seed: 42,
        };
        let mut buf = Vec::new();
        ciborium::into_writer(&req, &mut buf).unwrap();
        let out = stroke(&buf).expect("valid request must succeed");
        let resp: schema::Response = ciborium::from_reader(&out[..]).unwrap();
        assert!(!resp.paths.is_empty());
    }

    #[test]
    fn stroke_entry_rejects_bad_input() {
        let req = schema::StrokeRequest {
            points: vec![[0.0, 0.0]],
            closed: false,
            style: Default::default(),
            seed: 1,
        };
        let mut buf = Vec::new();
        ciborium::into_writer(&req, &mut buf).unwrap();
        let err = stroke(&buf).unwrap_err();
        assert!(
            err.contains("chalks-engine: stroke needs at least 2 points"),
            "{err}"
        );
    }

    #[test]
    fn fill_entry_round_trips_and_rejects_bad_pattern() {
        let mut req = schema::FillRequest {
            boundaries: vec![vec![[0.0, 0.0], [50.0, 0.0], [50.0, 50.0]]],
            style: Default::default(),
            seed: 3,
        };
        let mut buf = Vec::new();
        ciborium::into_writer(&req, &mut buf).unwrap();
        let resp: schema::Response = ciborium::from_reader(&fill(&buf).unwrap()[..]).unwrap();
        assert!(!resp.paths.is_empty());

        req.style.pattern = "polkadots".into();
        buf.clear();
        ciborium::into_writer(&req, &mut buf).unwrap();
        assert!(fill(&buf).unwrap_err().contains("unknown fill pattern"));
    }

    #[test]
    fn fill_entry_rejects_scribble_pattern() {
        let mut req = schema::FillRequest {
            boundaries: vec![vec![[0.0, 0.0], [50.0, 0.0], [50.0, 50.0]]],
            style: Default::default(),
            seed: 3,
        };
        req.style.pattern = "scribble".into();
        let mut buf = Vec::new();
        ciborium::into_writer(&req, &mut buf).unwrap();
        let err = fill(&buf).unwrap_err();
        assert!(
            err.contains("chalks-engine: unknown fill pattern: scribble"),
            "{err}"
        );
    }

    #[test]
    fn requests_reject_invalid_numbers_and_excessive_work() {
        fn encode(req: &impl serde::Serialize) -> Vec<u8> {
            let mut bytes = Vec::new();
            ciborium::into_writer(req, &mut bytes).unwrap();
            bytes
        }
        let mut s = schema::StrokeRequest {
            points: vec![[0.0, 0.0], [50.0, 10.0]],
            closed: false,
            style: Default::default(),
            seed: 42,
        };
        for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            s.points[0][0] = bad;
            assert!(stroke(&encode(&s)).unwrap_err().contains("coordinate"));
            s.points[0][0] = 0.0;
            s.style.width = bad;
            assert!(stroke(&encode(&s)).unwrap_err().contains("width"));
            s.style.width = 1.2;
            s.style.roughness = bad;
            assert!(stroke(&encode(&s)).unwrap_err().contains("roughness"));
            s.style.roughness = 1.0;
        }
        s.style.passes = 33;
        assert!(stroke(&encode(&s)).unwrap_err().contains("passes"));
        s.style.passes = 32;
        s.points = vec![[0.0, 0.0]; 129];
        assert!(stroke(&encode(&s))
            .unwrap_err()
            .contains("points times passes"));
        s.style.passes = 1;
        s.points = vec![[0.0, 0.0]; 4097];
        assert!(stroke(&encode(&s)).unwrap_err().contains("input points"));

        let mut f = schema::FillRequest {
            boundaries: vec![vec![
                [10.0, 10.0],
                [110.0, 10.0],
                [110.0, 110.0],
                [10.0, 110.0],
            ]],
            style: Default::default(),
            seed: 42,
        };
        for bad in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            f.style.spacing = bad;
            assert!(fill(&encode(&f)).unwrap_err().contains("spacing"));
            f.style.spacing = 4.0;
            f.style.angle = bad;
            assert!(fill(&encode(&f)).unwrap_err().contains("angle"));
            f.style.angle = 0.0;
            f.boundaries[0][0][1] = bad;
            assert!(fill(&encode(&f)).unwrap_err().contains("coordinate"));
            f.boundaries[0][0][1] = 10.0;
        }
        for pattern in ["hachure", "shade"] {
            f.style.pattern = pattern.into();
            for spacing in [1e-300, 0.001] {
                f.style.spacing = spacing;
                assert!(fill(&encode(&f)).unwrap_err().contains("fill work limit"));
            }
        }
    }
}
