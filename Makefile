# sci-sketch monorepo — chalks package + chalks-engine crate.
.PHONY: all pkgroot test rust-test examples images plugin install clean

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

# Link this checkout into Typst's user package directory so
# `@preview/chalks:0.1.0` resolves locally in any document.
TYPST_DATA_DIR := $(if $(filter Darwin,$(shell uname -s)),$(HOME)/Library/Application Support,$(if $(XDG_DATA_HOME),$(XDG_DATA_HOME),$(HOME)/.local/share))

install:
	@mkdir -p "$(TYPST_DATA_DIR)/typst/packages/preview/chalks"
	@ln -sfn "$(CURDIR)/chalks" "$(TYPST_DATA_DIR)/typst/packages/preview/chalks/0.1.0"
	@echo "linked @preview/chalks:0.1.0 -> $(CURDIR)/chalks"

clean:
	rm -rf _pkgroot
	@$(MAKE) -C chalks clean
