<div align="center">

# Sky Genesis Enterprise Registry Service

[![Crates.io](https://img.shields.io/crates/v/registry-service)](https://crates.io/crates/registry-service)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Build Status](https://img.shields.io/github/workflow/status/skygenesisenterprise/registry-service/CI)](https://github.com/skygenesisenterprise/registry-service/actions)
[![Coverage](https://img.shields.io/codecov/c/github/skygenesisenterprise/registry-service)](https://codecov.io/gh/skygenesisenterprise/registry-service)
[![Docker](https://img.shields.io/badge/docker-ready-blue.svg)](https://hub.docker.com/r/skygenesisenterprise/registry-service)

A comprehensive enterprise-grade package registry service inspired by Debian's package management system, designed for the Sky Genesis Enterprise ecosystem. This project provides both a RESTful API server and a command-line interface (CLI) tool called `cpkgs` for seamless package management.

</div>

## 🚀 Quick Start

```bash
# Clone the repository
git clone https://github.com/skygenesisenterprise/registry-service.git
cd registry-service

# Setup development environment
make setup

# Start all services
make dev
```

## 📋 Table of Contents

- [Features](#-features)
- [Architecture](#-architecture)
- [Installation](#-installation)
- [Usage](#-usage)
- [API Documentation](#-api-documentation)
- [Configuration](#-configuration)
- [Development](#-development)
- [Deployment](#-deployment)
- [Contributing](#-contributing)
- [License](#-license)

## ✨ Features

### Core Functionality
- **Package Management**: Complete CRUD operations for packages
- **Version Control**: Semantic versioning with dependency resolution
- **Authentication & Authorization**: JWT-based secure user management with role-based access
- **Search & Discovery**: Advanced package search with filtering capabilities
- **Dependency Management**: Handle complex package dependencies and conflicts
- **Multi-Architecture Support**: Package distribution across different architectures

### API Features
- **RESTful API**: Comprehensive endpoints with proper HTTP semantics
- **Database**: PostgreSQL with Prisma ORM for type-safe database operations
- **Security**: JWT authentication, CORS support, and security middleware
- **Observability**: Structured logging with tracing for monitoring and debugging
- **Performance**: Connection pooling and optimized query execution

### CLI Features (`cpkgs`)
- **Interactive Interface**: User-friendly package installation and removal
- **Advanced Search**: Package search with detailed information and filtering
- **Session Management**: Secure authentication and token handling
- **Administrative Tools**: Complete admin operations for package management
- **Performance**: Local caching and parallel operations
- **User Experience**: Colored output, progress indicators, and intuitive commands

## 🏗️ Architecture

### System Overview

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   CLI Client    │    │   Web Client    │    │  Mobile Client  │
│     (cpkgs)     │    │   Interface     │    │   Application   │
└─────────┬───────┘    └─────────┬───────┘    └─────────┬───────┘
          │                      │                      │
          └──────────────────────┼──────────────────────┘
                                 │
                    ┌─────────────┴─────────────┐
                    │      API Gateway          │
                    │   (Axum + Tower HTTP)     │
                    └─────────────┬─────────────┘
                                 │
                    ┌─────────────┴─────────────┐
                    │   Business Logic Layer    │
                    │     (Services)           │
                    └─────────────┬─────────────┘
                                 │
                    ┌─────────────┴─────────────┐
                    │    Data Access Layer      │
                    │  (Prisma + PostgreSQL)    │
                    └───────────────────────────┘
```

### Project Structure

```
registry-service/
├── api/                           # REST API server
│   ├── src/
│   │   ├── config/               # Application configuration
│   │   ├── controllers/          # Request handlers
│   │   ├── db/                   # Database connection
│   │   ├── middlewares/          # HTTP middleware
│   │   ├── models/               # Data transfer objects
│   │   ├── queries/              # Database queries
│   │   ├── routes/               # API route definitions
│   │   ├── services/             # Business logic
│   │   └── main.rs               # Application entry point
│   ├── prisma/
│   │   └── schema.prisma         # Database schema
│   └── Cargo.toml
├── cli/                          # Command-line interface
│   ├── src/
│   │   ├── commands/             # CLI command implementations
│   │   ├── config/               # Configuration management
│   │   └── main.rs               # CLI entry point
│   └── Cargo.toml
├── .github/                      # GitHub workflows
├── infrastructure/               # Docker and deployment configs
└── README.md
```

## 🛠️ Installation

### Prerequisites

- **Rust** 1.70+ (for building from source)
- **PostgreSQL** 12+ (for the database)
- **Node.js** 16+ (for Prisma client generation)
- **Docker** (optional, for containerized deployment)

### From Source

1. **Clone the repository**
   ```bash
   git clone https://github.com/skygenesisenterprise/registry-service.git
   cd registry-service
   ```

2. **Setup development environment**
   ```bash
   make setup
   ```

3. **Configure environment variables**
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

4. **Start the services**
   ```bash
   make dev
   ```

### Using Docker

```bash
# Build and start all services
make docker-up

# Build release images
make docker-build-release

# Push to registry
make docker-push-release
```

## 📖 Usage

### API Server

Start the API server:
```bash
cd api
cargo run
```

The API will be available at `http://localhost:3000`

### CLI Tool (`cpkgs`)

#### Authentication
```bash
# Register a new user
cpkgs auth register

# Login
cpkgs auth login

# Check authentication status
cpkgs auth status

# Logout
cpkgs auth logout
```

#### Package Management
```bash
# Search for packages
cpkgs search "web server"

# Install a package
cpkgs install nginx

# Install a specific version
cpkgs install nginx --version 1.21.0

# List installed packages
cpkgs list --installed

# List available packages
cpkgs list

# Get package information
cpkgs info nginx

# Remove a package
cpkgs remove nginx

# Update package index
cpkgs update

# Upgrade all packages
cpkgs upgrade --all
```

#### Administrative Operations
```bash
# Upload a package (admin only)
cpkgs admin upload ./package.deb

# Remove a package from registry (admin only)
cpkgs admin remove nginx 1.21.0

# List all users (admin only)
cpkgs admin list-users

# Create a new user (admin only)
cpkgs admin create-user john.doe@company.com
```

## 📚 API Documentation

### Base URL
```
http://localhost:3000/api
```

### Authentication
All protected endpoints require a Bearer token:
```
Authorization: Bearer <your-jwt-token>
```

### Core Endpoints

#### Packages
- `GET /packages` - List all packages
- `GET /packages/:id` - Get package details
- `POST /packages` - Create a new package
- `PUT /packages/:id` - Update package
- `DELETE /packages/:id` - Delete package
- `GET /packages/:id/download` - Download package file
- `GET /packages/search/:query` - Search packages

#### Users
- `GET /users` - List all users
- `GET /users/:id` - Get user details
- `POST /users` - Create user
- `PUT /users/:id` - Update user
- `DELETE /users/:id` - Delete user
- `GET /users/:id/packages` - Get user's packages

#### Authentication
- `POST /auth/login` - User login
- `POST /auth/register` - User registration
- `POST /auth/logout` - User logout

### Response Format

All API responses follow a consistent JSON format:

```json
{
  "id": "package_id",
  "name": "nginx",
  "version": "1.21.0",
  "description": "High performance web server",
  "maintainer": "admin@company.com",
  "architecture": "amd64",
  "size": 1048576,
  "checksum": "sha256_hash",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z",
  "author": "admin",
  "dependencies": [
    {
      "name": "libc6",
      "version": ">=2.28",
      "dependency_type": "REQUIRES"
    }
  ],
  "tags": [
    {
      "name": "web",
      "color": "#007bff"
    }
  ]
}
```

## ⚙️ Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | - |
| `RUST_LOG` | Logging level | `info` |
| `API_HOST` | API server host | `0.0.0.0` |
| `API_PORT` | API server port | `3000` |
| `JWT_SECRET` | JWT signing secret | - |
| `CORS_ORIGINS` | Allowed CORS origins | `*` |

### CLI Configuration

The CLI stores configuration in `~/.cpkgs/config.toml`:

```toml
registry_url = "http://localhost:3000"
auth_token = "your-jwt-token"
cache_dir = "/home/user/.cpkgs/cache"
install_dir = "/home/user/.cpkgs/packages"
```

## 🧪 Development

### Running Tests

```bash
# Run all tests
make test

# Run API tests
make api-test

# Run CLI tests
cd cli && cargo test

# Run tests with coverage
make test-coverage
```

### Code Quality

```bash
# Format code
make fmt

# Run linter
make lint

# Run clippy
make clippy

# Security audit
make audit
```

### Development Workflow

```bash
# Start development environment
make dev

# Build all components
make build

# Run health checks
make health

# Clean build artifacts
make clean
```

## 🚀 Deployment

### Production Deployment

#### Docker Compose

```bash
# Deploy with Docker Compose
make docker-up

# View logs
make docker-logs

# Scale services
docker-compose up -d --scale api=3
```

#### Kubernetes

```bash
# Apply Kubernetes manifests
kubectl apply -f infrastructure/k8s/

# Check deployment status
kubectl get pods -n registry-service
```

### Environment Configuration

#### Development
- Local PostgreSQL instance
- Debug logging enabled
- Hot reload for rapid development

#### Staging
- Managed PostgreSQL database
- Structured logging with tracing
- Performance monitoring enabled

#### Production
- High-availability PostgreSQL cluster
- Comprehensive monitoring and alerting
- Rate limiting and security hardening
- Automated backups and disaster recovery

## 🤝 Contributing

We welcome contributions! Please follow our [Contributing Guidelines](CONTRIBUTING.md).

### Development Process

1. **Fork** the repository
2. **Create** a feature branch (`git checkout -b feature/amazing-feature`)
3. **Commit** your changes (`git commit -m 'Add amazing feature'`)
4. **Push** to the branch (`git push origin feature/amazing-feature`)
5. **Open** a Pull Request

### Code Standards

- Follow [Rust best practices](https://rust-lang.github.io/api-guidelines/)
- Write comprehensive tests for new features
- Update documentation for API changes
- Use [conventional commit messages](https://www.conventionalcommits.org/)
- Ensure all CI checks pass before submitting

### Reporting Issues

- Use [GitHub Issues](https://github.com/skygenesisenterprise/registry-service/issues) for bug reports
- Use [GitHub Discussions](https://github.com/skygenesisenterprise/registry-service/discussions) for questions
- Check existing issues before creating new ones

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Documentation**: [Project Wiki](https://github.com/skygenesisenterprise/registry-service/wiki)
- **Issues**: [GitHub Issues](https://github.com/skygenesisenterprise/registry-service/issues)
- **Discussions**: [GitHub Discussions](https://github.com/skygenesisenterprise/registry-service/discussions)
- **Email**: support@skygenesisenterprise.com

## 🗺️ Roadmap

### Upcoming Features

- [ ] **Package Signing**: GPG-based package verification
- [ ] **Private Repositories**: Organization-specific package repositories
- [ ] **Web Interface**: React-based management dashboard
- [ ] **Analytics**: Package usage metrics and insights
- [ ] **CI/CD Integration**: GitHub Actions, GitLab CI plugins
- [ ] **Mobile CLI**: iOS and Android applications
- [ ] **GraphQL API**: Alternative API interface
- [ ] **Version Rollback**: Safe package version management
- [ ] **Security Scanning**: Automated vulnerability detection
- [ ] **Multi-Region**: Global package distribution

### Performance Improvements

- [ ] **CDN Integration**: Faster package downloads
- [ ] **Caching Layer**: Redis-based response caching
- [ ] **Database Optimization**: Query performance improvements
- [ ] **Horizontal Scaling**: Load balancing and clustering

---

<div align="center">

**Sky Genesis Enterprise Package Registry** - Modern package management for enterprise ecosystems.

Built with ❤️ by the Sky Genesis Enterprise team

</div>