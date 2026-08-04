#[cfg(target_arch = "wasm32")]
use wasm_minimal_protocol::*;

#[cfg(target_arch = "wasm32")]
initiate_protocol!();

#[cfg_attr(target_arch = "wasm32", wasm_func)]
pub fn version() -> Vec<u8> {
    format!("chalks-engine {}", env!("CARGO_PKG_VERSION")).into_bytes()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_reports_crate_semver() {
        assert_eq!(version(), b"chalks-engine 0.1.0".to_vec());
    }
}
