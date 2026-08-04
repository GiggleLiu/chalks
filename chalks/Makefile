.PHONY: all test examples images plugin clean

TYPST = typst compile --root .
TESTS := $(wildcard tests/test-*.typ)
ERROR_TESTS := $(wildcard tests/errors/*.typ)
EXAMPLES := $(wildcard examples/*.typ)

all: test

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
	  $(TYPST) --format png --ppi 144 $$f images/$$(basename $${f%.typ}).png || exit 1; \
	done

WASM_OUT = plugin/chalks_engine.wasm

plugin:
	cargo build --manifest-path ../Cargo.toml -p chalks-engine --release --target wasm32-unknown-unknown
	@mkdir -p plugin
	cp ../target/wasm32-unknown-unknown/release/chalks_engine.wasm $(WASM_OUT)
	@if command -v wasm-opt >/dev/null 2>&1; then \
	  echo "wasm-opt -Oz"; wasm-opt -Oz $(WASM_OUT) -o $(WASM_OUT); \
	else echo "wasm-opt not found, skipping (optional)"; fi
	@ls -l $(WASM_OUT)

clean:
	rm -f tests/*.pdf tests/errors/*.pdf examples/*.pdf manual.pdf
