# sci-sketch monorepo — chalks package + chalks-engine crate.
.PHONY: all pkgroot test rust-test examples images plugin clean

PACKAGES := chalks
export TYPST_PACKAGE_PATH := $(CURDIR)/_pkgroot

all: test

pkgroot:
	@rm -rf _pkgroot/preview
	@for pkg in $(PACKAGES); do \
	  mkdir -p _pkgroot/preview/$$pkg; \
	  ln -sfn $(CURDIR)/$$pkg _pkgroot/preview/$$pkg/0.1.0; \
	  echo "linked @preview/$$pkg:0.1.0 -> $$pkg/"; \
	done

rust-test:
	cargo test -p chalks-engine

test: pkgroot rust-test
	@$(MAKE) -C chalks test

examples: pkgroot
	@$(MAKE) -C chalks examples

images: pkgroot
	@$(MAKE) -C chalks images

plugin:
	@$(MAKE) -C chalks plugin

clean:
	rm -rf _pkgroot
	@$(MAKE) -C chalks clean
