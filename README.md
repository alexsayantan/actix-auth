# 🚀 JWT-Based Authentication System with Actix Web

Welcome to the **JWT-Based Authentication System** built with **Actix Web** in Rust! This project leverages the security, scalability and speed of Actix Web to provide a robust and secure authentication system.

## 🌟 Features

- **Role-Based Authentication**: Secure access with role-based permissions.
- **Argon2 Password Hashing**: Enhanced security with Argon2 hashing.
- **User Registration**: Easy user sign-up process.
- **Login with Token Generation**: Secure login with JWT token generation.
- **Token Removal with Logout API**: Seamless logout functionality.
- **Admin Role**: Admins can view the list of users.
- **User Details**: Logged-in users can view their own details.
- **Swagger UI**: Integrated Swagger UI for easy API documentation.
- **Production-Level Structure**: Organized and maintainable project structure.
- **PostgreSQL Connectivity**: Reliable database connectivity with PostgreSQL.
- **Middleware for Route Guarding**: Secure routes with custom middleware.

## 📚 Documentation

Explore the API documentation with Swagger UI:

<img src="https://static1.smartbear.co/swagger/media/assets/images/swagger_logo.svg" alt="Swagger UI" width="250"/>

## 🛠️ Installation

1. **Clone the repository**:
    ```bash
    https://github.com/alexsayantan/actix-auth.git
    cd jwt-auth-actix-web
    ```

2. **Set up PostgreSQL**:
    - Ensure you have PostgreSQL installed and running.
    - Create a database and update the connection string in the `.env` file.

3. **Run the application**:
    ```bash
    cargo run
    ```

## 🧩 Usage

- **Register a new user**:
    ```http
    POST /api/auth/register
    ```

- **Login and get a token**:
    ```http
    POST /api/auth/login
    ```

- **Logout and remove the token**:
    ```http
    POST /api/auth/logout
    ```

- **View user list (Admin only)**:
    ```http
    GET /api/users
    ```

- **View logged-in user details**:
    ```http
    GET /api/users/me
    ```

## 🏗️ Project Structure

```
src/
├── handlers/
│   ├── auth.rs
│   ├── mod.rs
│   └── users.rs
├── routes/
│   ├── auth.rs
│   ├── heath.rs
│   ├── mod.rs
│   └── users.rs
└── utils/
│   ├── mod.rs
│   ├── password.rs
│   └── token.rs
├── auth.rs
├── config.rs
├── db.rs
├── dtos.rs
├── error.rs
├── main.rs
└── models.rs
```

## 🤝 Contributing

Contributions are welcome! Please fork this repository and submit a pull request.
