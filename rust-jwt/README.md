# Rust JWT Authentication

Simple JWT authentication service built with Axum and jsonwebtoken.

## Features

- `/login` - Generate JWT token with username/password
- `/info` - Protected endpoint requiring valid JWT token
- 1-hour token expiration
- Bearer token authentication

## Running

```bash
cargo run -p rust-jwt
```

Server starts on `http://0.0.0.0:3000`

## Testing

### Using curl

**1. Login and get token**

```bash
curl -X POST http://localhost:3000/login \
  -H "Content-Type: application/json" \
  -d '{"username": "user", "password": "pass"}'
```

Response:
```json
{"token": "eyJ0eXAiOiJKV1QiLCJhbGc..."}
```

**2. Access protected endpoint**

```bash
curl http://localhost:3000/info \
  -H "Authorization: Bearer YOUR_TOKEN_HERE"
```

Response:
```json
"You're valid here is Info "
```

### Using Postman (or any API client)

**1. Login**
- Method: `POST`
- URL: `http://localhost:3000/login`
- Headers: `Content-Type: application/json`
- Body (raw JSON):
  ```json
  {
    "username": "user",
    "password": "pass"
  }
  ```
- Copy the token from response

**2. Get Info**
- Method: `GET`
- URL: `http://localhost:3000/info`
- Headers: `Authorization: Bearer YOUR_TOKEN_HERE`

## Notes

- Current implementation accepts any non-empty username/password
- Secret key is hardcoded as "secret" (demo purposes only)
- Tokens expire after 1 hour
