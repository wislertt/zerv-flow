
lint:
	npx prettier --write "**/*.{ts,tsx,css,json,yaml,yml,md}"
	cargo +nightly check --tests
	cargo +nightly fmt -- --check || (cargo +nightly fmt && exit 1)
	cargo +nightly clippy --all-targets --all-features -- -D warnings

update:
	rustup update
	cargo update

run:
	cargo run

test:
	RUST_LOG=cargo_tarpaulin=off cargo tarpaulin \
		--out Xml --out Html --out Lcov \
		--output-dir coverage \
		--exclude-files 'src/main.rs' \
		-- --quiet

open_coverage:
	open coverage/tarpaulin-report.html
