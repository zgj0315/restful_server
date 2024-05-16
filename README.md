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

# Start a postgres server
docker run \
    --rm \
    --name "restful_server_postgres" \
    --env POSTGRES_DB="restful_server_db" \
    --env POSTGRES_USER="restful_server_user" \
    --env POSTGRES_PASSWORD="restful_server_password" \
    -d -p 5432:5432 postgres:12

# refresh migration to pg
cargo run \
    --manifest-path migration/Cargo.toml \
    -- refresh \
    -u postgres://restful_server_user:restful_server_password@localhost:5432/restful_server_db

# init entity lib
cargo new --lib entity

# add sea-orm and serde to entity Cargo.toml
sea-orm = "0.12"
serde = {version = "1.0", features = ["derive"]}

# generate entity
sea-orm-cli generate entity --with-serde both \
    -u postgres://restful_server_user:restful_server_password@localhost:5432/restful_server_db \
    -o entity/src \
    -l
```