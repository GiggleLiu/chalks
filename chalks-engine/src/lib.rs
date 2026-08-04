#[cfg(target_arch = "wasm32")]
use wasm_minimal_protocol::*;

#[cfg(target_arch = "wasm32")]
initiate_protocol!();

pub mod geom;
pub mod rng;
pub mod schema;
pub mod stroke;
pub mod fill;

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
        paths: fill::run(&req.boundaries, &req.style, &mut rng),
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
        assert_eq!(version(), b"chalks-engine 0.1.0".to_vec());
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
        assert!(err.contains("chalks-engine: stroke needs at least 2 points"), "{err}");
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
        let resp: schema::Response =
            ciborium::from_reader(&fill(&buf).unwrap()[..]).unwrap();
        assert!(!resp.paths.is_empty());

        req.style.pattern = "polkadots".into();
        buf.clear();
        ciborium::into_writer(&req, &mut buf).unwrap();
        assert!(fill(&buf).unwrap_err().contains("unknown fill pattern"));
    }
}
