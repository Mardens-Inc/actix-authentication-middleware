# Actix Authentication Middleware

A Rust library that provides authentication middleware for Actix Web applications. This middleware validates user authentication tokens and integrates with a remote authentication service.

## Features

- Authentication middleware for Actix Web applications
- User authentication via username/password or token
- Token validation from request headers or cookies
- User registration and management
- Integration with a remote authentication API

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
actix-authentication-middleware = "0.1.0"
```

## Usage

### Basic Example

```rust
use actix_authentication_middleware::{Authentication, User};
use actix_web::{web, App, HttpServer, Responder, HttpResponse};

async fn protected_route() -> impl Responder {
    HttpResponse::Ok().body("This route is protected by authentication")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Authentication::new())  // Add the authentication middleware
            .route("/protected", web::get().to(protected_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### User Authentication

```rust
use actix_authentication_middleware::User;

async fn login(username: String, password: String) -> Result<String, String> {
    match User::authenticate_user(&username, &password).await {
        Ok(Some(token)) => Ok(token),
        Ok(None) => Err("Authentication failed".to_string()),
        Err(e) => Err(format!("Error: {}", e)),
    }
}
```

### User Registration

```rust
use actix_authentication_middleware::User;

async fn register(username: String, password: String) -> Result<(), String> {
    match User::register_user(&username, &password).await {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Registration error: {}", e)),
    }
}
```

## API Reference

### Authentication Middleware

The `Authentication` struct provides middleware for Actix Web applications:

- `Authentication::new()` - Creates a new instance of the authentication middleware

The middleware checks for authentication tokens in:
1. The `X-Authentication` header
2. A `token` cookie

### User Management

The `User` struct provides methods for user authentication and management:

- `User::authenticate_user(username, password)` - Authenticates a user with username and password
- `User::authenticate_user_with_token(token)` - Validates an authentication token
- `User::register_user(username, password)` - Registers a new user
- `User::get_users()` - Retrieves all users
- `User::query_user_by_name(username)` - Searches for users by username

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.