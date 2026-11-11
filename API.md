# 📚 API Documentation

## Base URL

```
http://localhost:3000
```

## Authentication Endpoints

### 1. Register User

**Endpoint**: `POST /api/auth/register`

**Request Body**:
```json
{
  "email": "user@example.com",
  "password": "SecurePassword123",
  "confirm_password": "SecurePassword123"
}
```

**Validation Rules**:
- Email must be valid format (RFC 5322)
- Password minimum 8 characters
- Confirm password must match password

**Success Response** (201 Created):
```json
{
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "created_at": "2024-11-11T10:30:00Z",
    "updated_at": "2024-11-11T10:30:00Z"
  },
  "message": "User registered successfully"
}
```

**Error Responses**:
- `400 Bad Request` - Invalid email or password
- `409 Conflict` - Email already exists
- `500 Internal Server Error` - Server error

**Example with cURL**:
```bash
curl -X POST http://localhost:3000/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "SecurePassword123",
    "confirm_password": "SecurePassword123"
  }'
```

---

### 2. Login User

**Endpoint**: `POST /api/auth/login`

**Request Body**:
```json
{
  "email": "user@example.com",
  "password": "SecurePassword123"
}
```

**Validation Rules**:
- Email must be valid format
- Password is required

**Success Response** (200 OK):
```json
{
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "created_at": "2024-11-11T10:30:00Z",
    "updated_at": "2024-11-11T10:30:00Z"
  },
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "expires_in": 86400
}
```

**Error Responses**:
- `400 Bad Request` - Invalid email format
- `401 Unauthorized` - Invalid credentials (email not found or wrong password)
- `500 Internal Server Error` - Server error

**Example with cURL**:
```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "SecurePassword123"
  }'
```

**Save Token**:
```bash
# Save token for authenticated requests
TOKEN=$(curl -s -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "SecurePassword123"
  }' | jq -r '.token')

echo $TOKEN
```

---

### 3. Logout User

**Endpoint**: `POST /api/auth/logout`

**Request**: No body required

**Success Response** (200 OK):
```json
{
  "message": "Logged out successfully"
}
```

**Note**: Token validation is client-side (remove from localStorage)

**Example with cURL**:
```bash
curl -X POST http://localhost:3000/api/auth/logout
```

---

## Protected Endpoints (Phase 1.6)

### 4. Get Current User

**Endpoint**: `GET /api/users/me`

**Headers**:
```
Authorization: Bearer <token>
```

**Success Response** (200 OK):
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "email": "user@example.com",
  "created_at": "2024-11-11T10:30:00Z",
  "updated_at": "2024-11-11T10:30:00Z"
}
```

**Error Responses**:
- `401 Unauthorized` - Missing or invalid token
- `404 Not Found` - User not found

**Example with cURL**:
```bash
# Using saved token
curl -X GET http://localhost:3000/api/users/me \
  -H "Authorization: Bearer $TOKEN"

# Or inline
curl -X GET http://localhost:3000/api/users/me \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

**Example with Postman**:
1. Get token from login endpoint
2. Set Authorization header: `Bearer {{token}}`
3. Send GET request to `http://localhost:3000/api/users/me`

---

## Health Check

**Endpoint**: `GET /health`

**Response** (200 OK):
```
Server is healthy ✅ | Database: Connected | JWT: Ready
```

---

## Error Response Format

All error responses follow this format:

```json
{
  "error": "ERROR_CODE",
  "message": "Human readable error message"
}
```

**Error Codes**:
- `INVALID_EMAIL` - Email format is invalid
- `INVALID_PASSWORD` - Password format is invalid
- `PASSWORD_TOO_SHORT` - Password must be at least 8 characters
- `EMAIL_EXISTS` - Email already registered
- `INVALID_CREDENTIALS` - Email or password is incorrect
- `TOKEN_EXPIRED` - JWT token has expired
- `INVALID_TOKEN` - JWT token is invalid
- `TOKEN_MISSING` - Authorization header missing
- `USER_NOT_FOUND` - User doesn't exist
- `DATABASE_ERROR` - Database operation failed
- `INTERNAL_ERROR` - Server error occurred

---

## JWT Token Structure

**Header**:
```json
{
  "alg": "HS256",
  "typ": "JWT"
}
```

**Payload**:
```json
{
  "sub": "user-uuid",
  "email": "user@example.com",
  "iat": 1699000000,
  "exp": 1699086400
}
```

**Token Validity**: 24 hours

---

## Testing with Postman

### 1. Register Request
```
POST http://localhost:3000/api/auth/register
Content-Type: application/json

{
  "email": "testuser@example.com",
  "password": "TestPassword123",
  "confirm_password": "TestPassword123"
}
```

### 2. Login Request
```
POST http://localhost:3000/api/auth/login
Content-Type: application/json

{
  "email": "testuser@example.com",
  "password": "TestPassword123"
}
```

**Save response token in Postman variable**:
1. Go to Tests tab
2. Add: `pm.environment.set("token", pm.response.json().token);`
3. Use in other requests: `Authorization: Bearer {{token}}`

---

## Rate Limiting

Currently no rate limiting implemented. Can be added in Phase 2.

---

## Pagination

Currently no pagination for list endpoints (no user list endpoint yet).

---

## CORS

CORS is currently permissive (allows all origins). Will be restricted in production.

---

## Version

**API Version**: v1
**Last Updated**: November 2024

---

**Next Steps**: 
- Phase 1.6: Protected endpoints with JWT middleware
- Phase 1.7: Testing and Git push
- Phase 2: Frontend implementation

