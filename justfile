@_list:
	just --list --unsorted


alias r := run

bt := '0'

log := "warn"

export JUST_LOG := log

watch:
	cargo watch -c -- just verify
	

run:
   trunk serve

test:
    cargo test --target wasm32-unknown-unknown

# Perform all verifications (compile, test, lint etc.)
verify: test lint

# Run the static code analysis
lint:
	cargo fmt --check --target wasm32-unknown-unknown
	cargo clippy --all-targets

clean:
	rm -rf target
	rm -f Cargo.lock
	rm -rf node_modules


fmt:
  cargo fmt

# run the release process in dry run mode (requires `npm`, a `GITHUB_TOKEN` and a `CARGO_REGISTRY_TOKEN`)
release *args:
	npm install --no-save conventional-changelog-conventionalcommits @semantic-release/exec
	npx semantic-release {{args}}


