set dotenv-load

# Perform all verifications (compile, test, lint, etc.)
verify: test lint doc

# Watch the source files and run `just verify` when source changes
watch:
	cargo watch --delay 0.1 --clear --why -- just verify

# Run the tests
test:
	cargo hack check --feature-powerset
	cargo check --examples
	cargo playdate build --example minimal

# Run the static code analysis
lint:
	cargo fmt -- --check
	cargo hack clippy --each-feature

run-example example="minimal":
	cargo playdate run --example {{example}}

# Build the documentation
doc *args:
	cargo doc --all-features --no-deps {{args}}

# Open the documentation page
doc-open: (doc "--open")

# Clean up compilation output
clean:
	rm -rf target
	rm -f Cargo.lock
	rm -rf node_modules

# Install cargo dev-tools used by the `verify` recipe (requires rustup to be already installed)
install-dev-tools:
	rustup install stable
	rustup override set stable
	cargo install cargo-hack cargo-watch cargo-playdate

# Install a git hook to run tests before every commits
install-git-hooks:
	echo '#!/usr/bin/env sh' > .git/hooks/pre-push
	echo 'just verify' >> .git/hooks/pre-push
	chmod +x .git/hooks/pre-push

release *args: verify
    test $GITHUB_TOKEN
    test $CARGO_REGISTRY_TOKEN
    cargo release {{args}}
