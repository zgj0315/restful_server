# restful_server
This is a RESTful Server Demo build with axum and sea-orm

## Todo List
### axum
- [x] middleware
- [x] graceful shutdown

### sea-orm
- [x] migration
- [ ] entity

```shell
# Install sea-orm-cli whit cargo
cargo install sea-orm-cli

# Setup the migration directory in `./migration`
sea-orm-cli migrate init

# Add migration to members, modify Cargo.toml
members = ["restful_service", "migration"]

# Add table kl_role and kl_user
sea-orm-cli migrate generate create_kl_role
sea-orm-cli migrate generate create_kl_user

# Remove default m20220101_000001_create_table.rs
rm migration/src/m20220101_000001_create_table.rs

# Modify kl_role and kl_user rs like demo

```