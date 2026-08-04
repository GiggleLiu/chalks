# sci-sketch

Monorepo for **chalks**, a Typst package for hand-drawn pencil/chalk-style
figures and annotations, and **chalks-engine**, the Rust crate compiled to
WASM that gives it its sketchy line and fill algorithms. Typst calls the
engine over a CBOR-encoded plugin boundary; styling, theming, and layout stay
entirely in Typst (`chalks/`), while `chalks-engine/` owns geometry, jitter,
tapering, and doodle-fill generation.

## Development

```sh
make test      # links the local @preview/chalks package root, runs
               # cargo test -p chalks-engine, then chalks' Typst test suite
               # (including compiling manual.typ) and error-message assertions
make examples  # compiles chalks/examples/*.typ against @preview/chalks:0.1.0
make images    # renders chalks/examples/*.typ to chalks/images/*.png for
               # visual review (also needs the package root, hence run from
               # here rather than `make -C chalks images`)
make plugin    # rebuilds plugin/chalks_engine.wasm with the pinned Rust
               # toolchain (rust-toolchain.toml) and, if available, wasm-opt
```

See `chalks/README.md` for the package's quick start and API. For the design
rationale and implementation plan, see
`docs/superpowers/specs/2026-08-04-chalks-design.md` and
`docs/superpowers/plans/2026-08-04-chalks.md`.
