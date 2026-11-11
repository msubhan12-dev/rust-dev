# CORS Configuration - PHASE 3.1

## Overview

Cross-Origin Resource Sharing (CORS) is configured on the backend to allow the Vue.js frontend (running on `localhost:5173`) to make HTTP requests to the API (running on `localhost:3000`).

## Current Configuration

### Development Environment (Default)
```rust
CorsLayer::very_permissive()
```

**Allows:**
- ✅ All origins
- ✅ All HTTP methods (GET, POST, PUT, DELETE, etc.)
- ✅ All headers
- ✅ Credentials (cookies, authentication headers)
- ✅ Any preflight requests

**Frontend URLs:**
- `http://localhost:5173` (Vue.js dev server)
- `http://127.0.0.1:5173` (alternative localhost)
- Any other origin (for testing)

### Production Environment (To Be Configured)
Set `ENVIRONMENT=production` in `.env` to enable strict CORS:

```rust
CorsLayer::very_permissive()
// In production, configure with specific origins:
// .allow_origin(AllowOrigin::predicate(|origin, _request_head| {
//     origin.as_bytes().ends_with(b"yourdomain.com")
// }))
```

## How to Configure for Production

### Step 1: Update CORS Layer

Edit `src/main.rs` and modify the `get_cors_layer()` function:

```rust
fn get_cors_layer() -> CorsLayer {
    let env = env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());
    
    match env.as_str() {
        "production" => {
            // Allow specific production domains
            CorsLayer::permissive()
                .allow_origin("https://yourdomain.com".parse().unwrap())
                .allow_origin("https://www.yourdomain.com".parse().unwrap())
                .allow_origin("https://api.yourdomain.com".parse().unwrap())
        },
        _ => {
            // Development: Allow all
            CorsLayer::very_permissive()
        }
    }
}
```

### Step 2: Set Environment Variable

In `.env` or `.env.production`:
```
ENVIRONMENT=production
```

### Step 3: Rebuild and Deploy

```bash
cargo build --release
```

## CORS Preflight Requests

When the frontend makes a cross-origin request (different origin from HTML page), the browser first sends a **preflight request** (OPTIONS method):

### Example Preflight Request
```http
OPTIONS /api/auth/login HTTP/1.1
Host: localhost:3000
Origin: http://localhost:5173
Access-Control-Request-Method: POST
Access-Control-Request-Headers: content-type, authorization
```

### Expected CORS Response Headers
```http
HTTP/1.1 200 OK
Access-Control-Allow-Origin: http://localhost:5173
Access-Control-Allow-Methods: GET, POST, PUT, DELETE, OPTIONS
Access-Control-Allow-Headers: content-type, authorization
Access-Control-Allow-Credentials: true
Access-Control-Max-Age: 86400
```

If CORS headers are correct, the browser allows the actual request.

## Testing CORS

### Using curl
```bash
# Test CORS from localhost:5173
curl -X OPTIONS http://localhost:3000/api/auth/login \
  -H "Origin: http://localhost:5173" \
  -H "Access-Control-Request-Method: POST" \
  -H "Access-Control-Request-Headers: content-type" \
  -v
```

### Using Browser DevTools
1. Open http://localhost:5173 in browser
2. Open DevTools (F12)
3. Go to Network tab
4. Make a login request
5. Check request headers: Should show `Origin: http://localhost:5173`
6. Check response headers: Should show `Access-Control-Allow-Origin` matching

### Using JavaScript
```javascript
// This will trigger a preflight request
fetch('http://localhost:3000/api/auth/login', {
  method: 'POST',
  headers: {
    'Content-Type': 'application/json',
    'Authorization': 'Bearer token'
  },
  body: JSON.stringify({ email: 'test@example.com', password: 'password' })
})
  .then(response => response.json())
  .then(data => console.log('Success:', data))
  .catch(error => console.error('Error:', error));
```

## Common CORS Issues & Solutions

### Issue: "No 'Access-Control-Allow-Origin' header"

**Symptoms:**
```
Access to XMLHttpRequest at 'http://localhost:3000/api/auth/login' 
from origin 'http://localhost:5173' has been blocked by CORS policy: 
No 'Access-Control-Allow-Origin' header is present on the requested resource.
```

**Causes:**
1. CORS layer not configured
2. Backend not running
3. Wrong port/origin

**Solution:**
- Verify backend is running: `cargo run`
- Check CORS configuration in `src/main.rs`
- Verify frontend URL is in allowed origins

### Issue: "Method not allowed"

**Symptoms:**
```
Access to XMLHttpRequest has been blocked by CORS policy: 
Method POST not allowed by Access-Control-Allow-Methods.
```

**Cause:** Preflight request failed

**Solution:**
- Ensure CORS allows the HTTP method
- Check that preflight OPTIONS requests are allowed

### Issue: "Credentials" error

**Symptoms:**
```
The value of the 'Access-Control-Allow-Credentials' header in the response is '' 
which must be 'true' when the request's credentials mode is 'include'.
```

**Cause:** Frontend sends credentials but backend doesn't allow them

**Solution:**
- Set `credentials: 'include'` in fetch options if needed
- Ensure backend CORS allows credentials

## Architecture

```
Frontend (http://localhost:5173)
         ↓ (XHR/Fetch with Origin header)
Browser (checks CORS)
         ↓ (preflight OPTIONS if cross-origin)
Backend (http://localhost:3000)
         ↓ (returns CORS headers)
Browser (validates CORS)
         ↓ (allows or blocks request)
JavaScript (success or error)
```

## Configuration by Deployment

### Local Development
```bash
ENVIRONMENT=development
# OR just omit (defaults to development)
```
✅ CORS: Very permissive
✅ Allows: All origins, methods, headers
✅ Good for: Development, testing

### Staging Environment
```bash
ENVIRONMENT=staging
```
⚠️ CORS: Should be configured like production
⚠️ Allows: Specific staging domain
⚠️ Good for: Testing production setup

### Production
```bash
ENVIRONMENT=production
CORS_ALLOWED_ORIGINS=https://yourdomain.com,https://www.yourdomain.com
```
🔒 CORS: Strict, only specific domains
🔒 Allows: Only whitelisted origins
🔒 Good for: Public/private APIs

## References

- [MDN: CORS](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS)
- [tower-http: CORS](https://docs.rs/tower-http/0.5/tower_http/cors/index.html)
- [W3C: Fetch Standard](https://fetch.spec.whatwg.org/#http-cors-protocol)

## Summary

CORS is configured using `tower-http`'s `CorsLayer`:
- **Development**: Very permissive (all origins allowed)
- **Production**: Restricted to specific domains (commented code shows pattern)
- **Testing**: Use browser DevTools to verify headers
- **Debugging**: Check for "No 'Access-Control-Allow-Origin'" errors

The frontend can now communicate with the backend without CORS issues during development. For production, update the allowed origins in the `get_cors_layer()` function.
