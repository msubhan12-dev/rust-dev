# 🚀 Enterprise Rust API dengan PostgreSQL & JWT Authentication

Full-stack application dengan authentication JWT, backend Rust + PostgreSQL, dan frontend TypeScript (Vue.js/Angular).

## 📋 Prerequisites

- **Rust**: 1.70+ ([https://rustup.rs/](https://rustup.rs/))
- **PostgreSQL**: 14+ atau Docker
- **Node.js**: 18+ (untuk frontend)
- **Git**: 2.30+

## 🏗️ Project Structure

```
rust-postgres-service/
├── src/
│   ├── main.rs           # Entry point
│   ├── db.rs             # Database initialization
│   ├── handlers/         # HTTP request handlers
│   ├── models/           # Data models & DTOs
│   ├── migrations/       # SQL migrations
│   └── .env              # Environment variables
├── Cargo.toml            # Rust dependencies
├── docker-compose.yml    # PostgreSQL container
└── README.md             # This file
```

## ⚙️ Setup Database

### Option 1: Using Docker (Recommended)

```bash
# Start PostgreSQL container
docker-compose up -d

# Verify connection
docker-compose ps
```

Database akan tersedia di: `postgresql://postgres:postgres@localhost:5432/rust_app`

### Option 2: Local PostgreSQL

```bash
# Create database
createdb rust_app

# Set environment variable
# Update DATABASE_URL in .env file
```

## 🚀 Quick Start

### 1. Clone & Install

```bash
cd rust-postgres-service
cargo build
```

### 2. Setup Environment

```bash
# File .env sudah siap, update jika perlu:
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/rust_app
JWT_SECRET=your-super-secret-key
```

### 3. Run Server

```bash
cargo run
```

Server akan berjalan di: `http://127.0.0.1:3000`

### 4. Test Endpoints

```bash
# Health check
curl http://localhost:3000/health

# Root
curl http://localhost:3000/
```

## 📚 Development Phases

### Phase 1: Backend Setup ✅ (In Progress)
- [x] PostgreSQL connection setup
- [ ] Database migrations
- [ ] JWT authentication
- [ ] API endpoints (Register, Login)
- [ ] Protected endpoints

### Phase 2: Frontend Setup
- [ ] Vue.js/Angular project setup
- [ ] Authentication service
- [ ] Login & Register pages
- [ ] Protected routes

### Phase 3: Integration & Deployment
- [ ] CORS setup
- [ ] End-to-end testing
- [ ] Docker deployment

## 🔐 API Endpoints (Coming Soon)

### Authentication
- `POST /api/auth/register` - Register user
- `POST /api/auth/login` - Login & get JWT token
- `GET /api/users/me` - Get current user (Protected)

## 📖 Useful Commands

```bash
# Build project
cargo build

# Run project
cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Check for issues
cargo clippy

# Create migration (when sqlx-cli is installed)
sqlx migrate add <migration_name>

# See logs
RUST_LOG=debug cargo run
```

## 🐛 Troubleshooting

### Database Connection Error
- Pastikan PostgreSQL running (gunakan `docker-compose ps`)
- Verify DATABASE_URL di .env file
- Check credentials

### Port Already in Use
- Default port: 3000
- Change di .env: `SERVER_PORT=3001`

### Compilation Error
- Update Rust: `rustup update`
- Clean build: `cargo clean && cargo build`

## 📝 Git Workflow

```bash
# Create feature branch
git checkout -b phase/1-backend-setup

# After changes
git add .
git commit -m "feat: setup database connection"
git push -u origin phase/1-backend-setup

# Create Pull Request
```

## 🤝 Contributing

Setiap phase akan di-push sebagai branch terpisah:
1. Phase 1: `phase/1-backend-setup`
2. Phase 2: `phase/2-frontend-setup`
3. Phase 3: `phase/3-integration`

## 📄 License

MIT

## 🎯 Next Steps

1. ✅ Setup database connection (CURRENT)
2. Create database migrations
3. Implement JWT authentication
4. Build API endpoints
5. Create frontend

---

Dokumentasi akan di-update seiring dengan development 🚀
