# Chewback - Desktop Messenger Application

A modern, secure desktop messenger built with Tauri, SolidJS, and Rust backend.

## 🚀 Project Overview

Chewback is a secure desktop messaging application with a modern architecture that combines:

- **Frontend**: SolidJS with TypeScript and Tailwind CSS
- **Desktop Shell**: Tauri for native desktop experience
- **Backend Server**: Rust with Axum framework
- **Database**: PostgreSQL with SQLx

## 📁 Project Structure

```
chewback/
├── src/                    # Frontend (SolidJS + TypeScript)
│   ├── api/               # API clients and types
│   ├── assets/            # Static assets
│   ├── bindings/          # OpenAPI TypeScript bindings
│   ├── context/           # SolidJS context providers
│   ├── hooks/             # Custom React hooks
│   ├── pages/             # Application pages
│   └── components/        # Reusable components
├── src-tauri/             # Tauri desktop application
│   ├── src/               # Rust backend for Tauri
│   ├── capabilities/      # Tauri capabilities
│   └── icons/             # Application icons
├── server/                # Backend server (Rust + Axum)
│   ├── src/               # Server source code
│   │   ├── handlers/      # Request handlers
│   │   ├── models/        # Data models
│   │   ├── routes/        # API routes
│   │   └── services/      # Business logic
│   └── migrations/        # Database migrations
├── docs-project/          # Project documentation
└── public/                # Public assets
```

## 🛠️ Technology Stack

### Frontend (Client)

- **SolidJS** - Reactive UI framework
- **TypeScript** - Type-safe JavaScript
- **Tailwind CSS** - Utility-first CSS framework
- **@solidjs/router** - Client-side routing
- **@tauri-apps/api** - Tauri API bindings

### Desktop Application (Tauri)

- **Rust** - System programming language
- **Tauri 2.0** - Desktop application framework
- **Keyring** - Secure credential storage
- **Reqwest** - HTTP client for API requests

### Backend Server

- **Rust** with **Axum** - Web framework
- **PostgreSQL** - Relational database
- **SQLx** - Async SQL toolkit
- **JWT** - JSON Web Tokens for authentication
- **Argon2** - Password hashing
- **WebSocket** - Real-time messaging (planned)

## 🔧 Prerequisites

- **Rust** (latest stable)
- **Node.js** 18+ or **Bun** 1.0+
- **PostgreSQL** 17+ (or Docker)
- **Tauri CLI**: `cargo install tauri-cli`

## 🚀 Quick Start

### 1. Clone and Install Dependencies

```bash
# Clone the repository
git clone <repository-url>
cd chewback

# Install frontend dependencies
bun install

# Install Rust dependencies (automatically handled by Cargo)
```

### 2. Database Setup

```bash
# Start PostgreSQL with Docker
docker-compose up -d

# Run database migrations
cd server
sqlx migrate run
```

### 3. Development

#### Start the Backend Server

```bash
cd server
cargo watch -x run
```

#### Start the Frontend with Tauri

```bash
# In the project root
bun run tauri dev
```

### 4. Build for Production

```bash
# Build the application
bun run tauri build
```

## 📖 Available Scripts

### Using Just (Task Runner)

```bash
# Build the application
just build

# Run in development mode
just run

# Start the backend server
just server

# Generate TypeScript types from OpenAPI
just typegen

# Open Swagger UI
just swagger

# Reset database
just db-reset
```

### Using Package.json Scripts

```bash
# Start development server
bun run dev

# Build for production
bun run build

# Tauri commands
bun run tauri dev    # Development
bun run tauri build  # Production build
```

## 🔐 Authentication System

The application implements a secure JWT-based authentication system:

1. **Registration/Login** - Users can register and login with credentials
2. **JWT Tokens** - Access tokens (short-lived) and refresh tokens (long-lived)
3. **Secure Storage** - Refresh tokens stored in system keyring
4. **Auto-refresh** - Automatic token refresh before expiration
5. **Session Management** - Server-side session tracking

### Key Features:

- Password hashing with Argon2
- Rate limiting for authentication endpoints
- Secure credential storage using system keyring
- Automatic session restoration on app restart

## 📡 API Architecture

### Client-Server Communication

- **HTTP API** - RESTful endpoints for user management and data
- **Tauri Bridge** - Rust commands for secure system operations
- **Type Safety** - Auto-generated TypeScript bindings from OpenAPI

### API Endpoints

- `POST /api/v1/auth/register` - User registration
- `POST /api/v1/auth/login` - User login
- `POST /api/v1/auth/refresh` - Token refresh
- `POST /api/v1/auth/logout` - User logout
- `GET /api/v1/auth/me` - Get current user info

## 🗄️ Database Schema

### Users Table

```sql
CREATE TABLE users (
    id TEXT PRIMARY KEY NOT NULL,
    login TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    role user_role NOT NULL DEFAULT 'user',
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
```

### Sessions Table

```sql
CREATE TABLE sessions (
    id TEXT PRIMARY KEY NOT NULL,
    user_id TEXT NOT NULL,
    refresh_token_hash TEXT NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
```

## 🧪 Testing

### Backend Tests

```bash
cd server
cargo test
```

### Database Integration Tests

```bash
cd server
DATABASE_URL=postgres://user:password@localhost:5431/chewback cargo test
```

## 📈 Development Roadmap

### Current Status (Phase 1-2)

- ✅ Basic authentication system
- ✅ User registration and login
- ✅ JWT token management
- ✅ Secure credential storage
- ✅ Database migrations
- ✅ API client with TypeScript bindings
- ✅ SolidJS frontend with routing
- ✅ Tauri desktop shell

### Planned Features

- **Phase 3**: Real-time messaging with WebSocket
- **Phase 3**: Message history and user search
- **Phase 4**: File sharing and group chats
- **Phase 5**: End-to-end encryption

## 🔧 Configuration

### Environment Variables (Server)

Create `.env` file in `server/` directory:

```env
DATABASE_URL=postgres://user:password@localhost:5431/chewback
JWT_SECRET=your-secret-key-here
```

### Tauri Configuration

- **App Identifier**: `com.kozar.chewback`
- **Window Size**: 800x600
- **Build Targets**: All platforms

## 🐛 Troubleshooting

### Common Issues

1. **Database Connection Failed**
   - Ensure PostgreSQL is running: `docker-compose up -d`
   - Check connection string in `.env` file

2. **Tauri Build Errors**
   - Update Rust: `rustup update`
   - Clean build: `cargo clean && bun run tauri build`

3. **TypeScript Generation**
   - Ensure server is running: `just server`
   - Generate types: `just typegen`

## 📄 License

MIT License - see LICENSE file for details.

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests
5. Submit a pull request

## 📞 Support

For issues and questions, please check:

- [Tauri Documentation](https://tauri.app/)
- [SolidJS Documentation](https://www.solidjs.com/)
- [Axum Documentation](https://docs.rs/axum/latest/axum/)
