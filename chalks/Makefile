.PHONY: all test examples images plugin clean manual

TYPST = typst compile --root .
TESTS := $(wildcard tests/test-*.typ)
ERROR_TESTS := $(wildcard tests/errors/*.typ)
EXAMPLES := $(wildcard examples/*.typ)

all: test

manual:
	$(TYPST) manual.typ manual.pdf

test: plugin manual
test:
	@for t in $(TESTS); do \
	  echo "== $$t"; \
	  $(TYPST) $$t $${t%.typ}.pdf || exit 1; \
	done
	@for t in $(ERROR_TESTS); do \
	  echo "== $$t (expected error)"; \
	  expected=$$(sed -n 's|^// expected: ||p' $$t); \
	  if [ -z "$$expected" ]; then echo "missing // expected: marker in $$t"; exit 1; fi; \
	  out=$$($(TYPST) $$t /tmp/chalks-expected-error.pdf 2>&1); status=$$?; \
	  if [ $$status -eq 0 ]; then echo "expected compilation to fail"; exit 1; fi; \
	  echo "$$out" | grep -F -- "$$expected" >/dev/null || { echo "$$out"; exit 1; }; \
	done
	@echo "All tests passed!"

examples:
	@for f in $(EXAMPLES); do \
	  echo "== $$f"; \
	  $(TYPST) $$f $${f%.typ}.pdf || exit 1; \
	done

images:
	@mkdir -p images
	@for f in $(EXAMPLES); do \
	  echo "== $$f"; \
	  $(TYPST) --ignore-system-fonts --format png --ppi 144 $$f images/$$(basename $${f%.typ}).png || exit 1; \
	done

WASM_OUT = plugin/chalks_engine.wasm

# Remap host-specific paths so the WASM is byte-identical across machines.
# Std sources are canonically /rustc/<commit>/library/... (toolchains without
# rust-src embed exactly that); with rust-src installed rustc resolves them to
# the local checkout instead, so map that checkout back to the canonical form.
# NOTE: builds are byte-stable per platform, but rustc emits functions in a
# host-dependent order, so the committed artifact must come from CI's platform
# (x86_64 Linux) — use `make plugin-linux` to regenerate it anywhere.
RUST_SYSROOT = $(shell rustc --print sysroot)
RUST_COMMIT = $(shell rustc -vV | sed -n 's/^commit-hash: //p')
CARGO_HOME_DIR = $(shell echo "$${CARGO_HOME:-$$HOME/.cargo}")
WORKSPACE_ROOT = $(abspath ..)
REPRO_RUSTFLAGS = --remap-path-prefix=$(CARGO_HOME_DIR)=/cargo --remap-path-prefix=$(WORKSPACE_ROOT)=/build --remap-path-prefix=$(RUST_SYSROOT)/lib/rustlib/src/rust=/rustc/$(RUST_COMMIT)

plugin:
	RUSTFLAGS="$(REPRO_RUSTFLAGS)" cargo build --manifest-path ../Cargo.toml -p chalks-engine --release --target wasm32-unknown-unknown
	@mkdir -p plugin
	cp ../target/wasm32-unknown-unknown/release/chalks_engine.wasm $(WASM_OUT)
	@ls -l $(WASM_OUT)

# Rebuild the committed artifact on CI's platform from any host (needs Docker).
plugin-linux:
	docker run --rm --platform linux/amd64 -v "$(abspath ..)":/src:ro -v "$(CURDIR)/plugin":/out rust:1.93.1 \
	  bash -c 'cp -r /src /tmp/repo && cd /tmp/repo && rm -rf .git target && \
	    rustup target add wasm32-unknown-unknown && make -C chalks plugin && \
	    cp chalks/plugin/chalks_engine.wasm /out/'
	@ls -l $(WASM_OUT)

clean:
	rm -f tests/*.pdf tests/errors/*.pdf examples/*.pdf
