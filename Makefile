.PHONY: install format lint test build release sign

install:
	cargo fetch

format:
	cargo fmt

lint:
	cargo clippy --all-targets -- -D warnings

test:
	cargo test

# A stable self-signed identity gives the binary a constant code identity across
# rebuilds, so firewall and security rules survive recompilation. Without it the
# linker's ad-hoc signature changes every build.
SIGN_IDENTITY ?= InstruktAI Dev
SIGN_ID ?= ai.instrukt.erpnext

build:
	cargo build
	@$(MAKE) --no-print-directory sign BIN=target/debug/erpnext

release:
	cargo build --release
	@$(MAKE) --no-print-directory sign BIN=target/release/erpnext

sign:
	@if security find-identity -v -p codesigning 2>/dev/null | grep -q "$(SIGN_IDENTITY)"; then \
		codesign --force --options runtime --identifier "$(SIGN_ID)" --sign "$(SIGN_IDENTITY)" "$(BIN)" && \
		echo "signed $(BIN) as '$(SIGN_ID)' with '$(SIGN_IDENTITY)'"; \
	else \
		echo "no signing identity '$(SIGN_IDENTITY)' in the keychain; $(BIN) keeps its ad-hoc signature"; \
	fi
