# Rust API with Postgres

## Quickstart

Edit the `.env` file to point to a postgres database.

```
cargo install diesel_cli --no-default-features --features postgres

diesel migration run
```