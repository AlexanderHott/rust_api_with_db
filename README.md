# Rust API with Postgres

## Quickstart

Edit the `.env` file to point to a postgres database.

```
rustup default nightly

cargo install diesel_cli --no-default-features --features postgres

diesel migration run
```

## Diesel Commands

```bash
diesel setup
diesel migration generate create_books
# diesel migration run
# diesel migration redo
diesel print-schema > src/schema.rs
```