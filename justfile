watch +args='test':
  cargo watch --clear --exec '{{args}}'

run +args='balance': 
  RUST_LOG=info cargo run -- --chain signet --data-dir . '{{args}}'

fmt:
  cargo fmt --all

clippy:
  cargo clippy --all --all-targets -- --deny warnings

ci: clippy
  cargo fmt -- --check
  cargo test --all
  cargo test --all -- --ignored

doc:
  cargo doc --all --open

outdated:
  cargo outdated -R --workspace

unused:
  cargo +nightly udeps
