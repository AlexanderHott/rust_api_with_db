# Rust API with Postgres

## Quickstart

Rename `.env.example` to `.env` and edit it to point to a postgres database.

```
rustup default nightly

cargo install diesel_cli --no-default-features --features postgres

diesel migration run

cargo run
```

Routes can be found in `src/routes.rs`

## Diesel Commands

```bash
diesel setup
diesel migration generate create_books
# diesel migration run
# diesel migration redo
diesel print-schema > src/schema.rs
```
