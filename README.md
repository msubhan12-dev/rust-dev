# 🎨 Frontend - Vue.js 3 + TypeScript

Enterprise-scale frontend application dengan Vue.js 3, TypeScript, dan JWT authentication.

## 📋 Tech Stack

- **Vue.js 3** - Progressive JavaScript framework
- **TypeScript** - Type-safe JavaScript
- **Vite** - Next-generation build tool
- **Pinia** - State management
- **Vue Router** - Client-side routing
- **Axios** - HTTP client
- **TailwindCSS** - Utility-first CSS (optional)

## 🚀 Quick Start

### 1. Install Dependencies

```bash
npm install
```

### 2. Setup Environment

```bash
# Copy environment template
cp .env.example .env

# Update API URL if needed
VITE_API_URL=http://localhost:3000
```

### 3. Start Development Server

```bash
npm run dev
```

Server akan berjalan di: `http://localhost:5173`

### 4. Build for Production

```bash
npm run build
```

Output akan di: `dist/`

## 📁 Project Structure

```
src/
├── api/              # API service layer
│   └── auth.ts       # Authentication API calls
├── components/       # Reusable Vue components
├── stores/           # Pinia stores
│   └── authStore.ts  # Auth state management
├── views/            # Page components
│   ├── LoginView.vue
│   ├── RegisterView.vue
│   └── DashboardView.vue
├── router/           # Vue Router configuration
│   └── index.ts
├── types/            # TypeScript interfaces
│   └── auth.ts
├── App.vue           # Root component
└── main.ts           # Application entry point
```

## 🔐 Authentication Flow

```
User Input (Login/Register)
     ↓
[Validation]
     ↓
[API Call to Backend]
     ↓
[Store JWT Token]
     ↓
[Set Auth State]
     ↓
[Redirect to Dashboard]
```

## 📚 Features

### Phase 2.1: Project Setup ✅
- [x] Vue.js 3 + TypeScript project
- [x] Vite configuration
- [x] Dependencies installed
- [x] Environment variables
- [x] TypeScript types

### Phase 2.2: Auth Service
- [ ] API client (axios)
- [ ] Pinia auth store
- [ ] Token management
- [ ] HTTP interceptors

### Phase 2.3: Auth Pages
- [ ] Login page
- [ ] Register page
- [ ] Form validation
- [ ] Error messages

### Phase 2.4: Routing & Guards
- [ ] Route definitions
- [ ] Protected routes
- [ ] Navigation guards
- [ ] Auto-login

### Phase 2.5: Dashboard
- [ ] User dashboard page
- [ ] User info display
- [ ] Logout functionality
- [ ] UI improvements

### Phase 2.6: Testing & Deployment
- [ ] End-to-end testing
- [ ] Bug fixes
- [ ] Production build
- [ ] Git push

## 🛠️ Available Scripts

```bash
# Development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview

# Type checking
npm run type-check

# Format code
npm run format
```

## 🔗 Backend Integration

Backend API tersedia di: `http://localhost:3000`

### API Endpoints

```
POST   /api/auth/register    - Register user
POST   /api/auth/login       - Login user
POST   /api/auth/logout      - Logout user
GET    /api/users/me         - Get current user (protected)
```

**Header untuk protected routes:**
```
Authorization: Bearer <jwt_token>
```

## 📝 Environment Variables

```env
# API Configuration
VITE_API_URL=http://localhost:3000

# Request timeout
VITE_API_TIMEOUT=10000

# Dev server
VITE_DEV_SERVER_PORT=5173
VITE_DEV_SERVER_HOST=127.0.0.1
```

## 🧪 Testing Authentication

### 1. Register User
```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "Password123",
    "confirm_password": "Password123"
  }'
```

### 2. Login
```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "Password123"
  }'
```

### 3. Copy JWT token dan test di frontend

## 🎯 Learning Path

1. **Components** - Vue.js component basics
2. **Routing** - Single-page application routing
3. **State Management** - Pinia for centralized state
4. **API Integration** - Axios for backend communication
5. **Authentication** - JWT token handling
6. **TypeScript** - Type-safe development

## 🚀 Deployment

### Local Development
```bash
npm run dev
```

### Production Build
```bash
npm run build
npm run preview
```

### Deploy to Cloud (later phases)
- Vercel
- Netlify
- AWS S3 + CloudFront

## 📖 Useful Resources

- [Vue.js 3 Documentation](https://vuejs.org/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- [Vite Documentation](https://vitejs.dev/)
- [Pinia Documentation](https://pinia.vuejs.org/)
- [Vue Router Documentation](https://router.vuejs.org/)
- [Axios Documentation](https://axios-http.com/)

## 🐛 Troubleshooting

### Port already in use
```bash
# Change port in vite.config.ts or .env
VITE_DEV_SERVER_PORT=5174
```

### API connection refused
- Verify backend running: `cargo run`
- Check VITE_API_URL in .env
- Verify backend on http://localhost:3000

### CORS issues
- Backend CORS sudah configured (permissive)
- Check browser console for error details

## 📝 Next Steps

1. ✅ Setup project (PHASE 2.1)
2. → Create auth service (PHASE 2.2)
3. → Build login/register pages (PHASE 2.3)
4. → Setup routing (PHASE 2.4)
5. → Build dashboard (PHASE 2.5)
6. → Test & deploy (PHASE 2.6)

---

**Status**: Phase 2.1 Complete ✅
**Next**: Phase 2.2 - Authentication Service

