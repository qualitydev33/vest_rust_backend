# Rust Test Project
## Build
* Simple build

```
cargo build
```
* Watching
```
cargo install cargo-watch
cargo watch -x build
```

## Installing sea-orm-cli and Migration
* Installing sea-orm-cli
```
cargo install sea-orm-cli
sea-orm-cli migrate init
```
* Open the file migration/Cargo.toml and uncomment the two dependencies("sqlx-sqlite", "runtime-tokio-rustls") for sea-orm-migration.
<br />
<br />
* Set the URL for your database as an environment varibale.
```
export DATABASE_URL='sqlite://posts.sqlite?mode=rwc'
```
* We will run the migration.
```
sea-orm-cli migrate up
```
<br />

## Generate entities
* Create a new entity module.
```
cargo new entity --lib
```

* Next, generate the entities and add the sea-orm dependency to the entity module.
```
sea-orm-cli generate entity -o entity/src -u <database_url>
```