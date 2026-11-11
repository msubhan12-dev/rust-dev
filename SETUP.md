# 🛠️ Database Setup Guide

## Prerequisites

- Docker & Docker Compose (recommended)
- atau PostgreSQL 14+ local installation
- sqlx-cli (optional, untuk manual migrations)

## 🐳 Quick Start dengan Docker

### 1. Start PostgreSQL Container

```bash
# Start the database
docker-compose up -d

# Verify container is running
docker-compose ps

# View logs
docker-compose logs postgres
```

Database akan siap di: `postgresql://postgres:postgres@localhost:5432/rust_app`

### 2. Run Backend (migrations akan berjalan otomatis)

```bash
cargo run
```

Output yang diharapkan:
```
🔧 Initializing database connection...
✅ Database migrations completed
✅ Database connection verified
🚀 Server running at http://127.0.0.1:3000
```

## 📊 Verify Database Setup

### Option 1: Using `psql` CLI

```bash
# Connect to database
psql postgresql://postgres:postgres@localhost:5432/rust_app

# List tables
\dt

# Describe users table
\d users

# Exit
\q
```

### Option 2: Docker Exec

```bash
# Connect to container
docker exec -it rust_postgres_db psql -U postgres -d rust_app

# View tables
\dt

# View users table structure
\d users

# Exit
\q
```

### Option 3: DBeaver atau pgAdmin

- DBeaver: https://dbeaver.io (desktop app)
- pgAdmin: `docker-compose up pgadmin` (add to docker-compose.yml)

## 🔄 Manual Migration (jika diperlukan)

### Install sqlx-cli

```bash
cargo install sqlx-cli --no-default-features --features postgres
```

### Create Migration

```bash
# Create new migration
sqlx migrate add -r <migration_name>

# Run pending migrations
sqlx migrate run --database-url $DATABASE_URL

# Revert last migration
sqlx migrate revert --database-url $DATABASE_URL
```

## 📝 Migration Files

Migrations disimpan di: `src/migrations/`

Naming convention:
- `001_create_users.sql` (up migration)
- `001_create_users.sql.down` (down migration, jika perlu revert)

## 🧹 Cleanup & Reset

### Reset Database (WARNING: akan menghapus semua data!)

```bash
# Stop container
docker-compose down

# Remove volume (data will be deleted)
docker volume rm rust-postgres-service_postgres_data

# Start fresh
docker-compose up -d

# Migrations akan berjalan lagi
cargo run
```

### Backup Database

```bash
# Backup
docker exec rust_postgres_db pg_dump -U postgres rust_app > backup.sql

# Restore
docker exec -i rust_postgres_db psql -U postgres rust_app < backup.sql
```

## 🚨 Troubleshooting

### Connection Refused
```
Error: connection refused
```
- Pastikan container running: `docker-compose ps`
- Cek DATABASE_URL di .env
- Tunggu PostgreSQL siap: `docker-compose logs postgres`

### Migration Error
```
Error: migration failed
```
- Check logs: `cargo run`
- Verify migration file syntax
- Clear migration state: `docker-compose down -v`

### Port Already in Use
```
Error: address already in use
```
- PostgreSQL: change port in docker-compose.yml (5432:5433)
- Backend: change port in .env (SERVER_PORT=3001)

## 📚 Database Schema

### Users Table

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Indexes
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_created_at ON users(created_at);

-- Auto-update timestamp
CREATE TRIGGER update_users_updated_at BEFORE UPDATE ON users
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
```

## ✅ Verification Checklist

- [ ] Docker container running: `docker-compose ps`
- [ ] Database created: `CREATE DATABASE rust_app`
- [ ] Users table created: `\dt` shows users table
- [ ] Indexes created: `\di` shows indexes
- [ ] Backend connects: `cargo run` shows "✅ Database connection verified"
- [ ] Health endpoint works: `curl http://localhost:3000/health`

---

Next: PHASE 1.3 - JWT Authentication Setup 🔐

