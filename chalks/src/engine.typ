// WASM bridge: CBOR in/out to chalks-engine. Styling stays in Typst.
#let _engine = plugin("../plugin/chalks_engine.wasm")

/// Plugin version string (smoke check that the binary loads).
#let engine-version() = str(_engine.version())
