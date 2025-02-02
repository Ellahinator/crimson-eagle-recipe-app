# Crimson Eagle - Recipe Site

## Diesel Implementation

**Set Dependencies:**

Cargo.toml

```rust
[dependencies]
diesel = { version = "2.1.0", features = ["postgres"] }
```

Setting postgres features streamlines the crate for postgres usage.

**Install Diesel CLI**

```bash
cargo install diesel_cli --no-default-features --features postgres
```

Again streamlined for postgres

It needs PostgreSQL installed on your OS. Add the environmental variable PQ_LIB_DIR, for example on windows:

```bash
$ENV:PQ_LIB_DIR="c:\Program Files\PostgreSQL\15\lib"
```

**Database Url**

Set up the database url in `Rocket.toml`

```toml
[global.databases.postgres_logs]
url = "postgres://username:password@localhost/table-name"
timeout = 45
```

Set this to the database url with the login credentials

**Diesel Setup Command**

```rust
diesel setup
```

This command will create a migration file and a diesel.toml file with the necessary configurations.

**Generate Diesel Migration Files**

```rust
diesel migration generate TABLE_NAME
```

This will create two files in /migrations called up.sql and down.sql.

Repeat this step for all your tables

**Write your SQL Query in up.sql**

```sql
CREATE TABLE example (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  servings TEXT NOT NULL,
  created_at TIMESTAMP,
  created_by TIMESTAMP
)
```

This will create your migration file.

**Write your roll-back function in down.sql**

```sql
DROP TABLE example
```

Important: This will allow you to run a command that updates your table if you need to change your structure

**Run your migration**

```bash
diesel migration run
```

This will generate schema.rs in your src folder and write your schema to the database. Use postgres cli to check it has written correctly.
