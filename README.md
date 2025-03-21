# Rust Crates Management System

A Rust application for managing crates and rustaceans, built with Rocket and Diesel ORM.

## Project Overview

- Manage rustaceans (Rust developers)
- Track crates (Rust packages)
- Handle user authentication and authorization

## Technologies Used

- **Rust** - Primary programming language
- **Rocket** - Web framework for routing and HTTP handling
- **Diesel** - ORM for database interactions with PostgreSQL
- **Docker** - Containerization for development and deployment
- **Redis** - Caching layer
- **Clap** - Command-line argument parsing
- **Tokio** - Asynchronous runtime

## Getting Started

### Prerequisites

- Docker and Docker Compose

### Setup

1. Clone the repository
2. Start the application with Docker:

```bash
docker-compose up -d --build
```

3. Run migrations to set up the database:

```bash
docker-compose exec app diesel migration run
```

## Usage

### Web API

The application exposes a REST API on port 8000. Some example endpoints:

- `GET /rustaceans` - List all rustaceans
- `POST /rustaceans` - Create a new rustacean
- `GET /rustaceans/{id}` - Get a specific rustacean
- `PUT /rustaceans/{id}` - Update a rustacean
- `DELETE /rustaceans/{id}` - Delete a rustacean

### CLI

The application includes a command-line interface for user management:

```bash
# Show user creation arguments
docker-compose exec app cargo run --bin cli users create

# Create a user
docker-compose exec app cargo run --bin cli users create username password role

# List all users
docker-compose exec app cargo run --bin cli users list

# Delete a user by ID
docker-compose exec app cargo run --bin cli users delete 1
```

## Development

### Project Structure

- `src/bin/` - Executable entry points (server and CLI)
- `src/models.rs` - Database models
- `src/repositories.rs` - Data access layer
- `src/rocket_routes/` - Web API routes
- `src/schema.rs` - Database schema definitions
- `migrations/` - Database migrations
- `tests/` - Integration tests

### Database Migrations

```bash
# Create a new migration
docker-compose exec app diesel migration generate <migration_name>

# Run pending migrations
docker-compose exec app diesel migration run

# Revert the last migration
docker-compose exec app diesel migration revert

# List migrations
docker-compose exec app diesel migration list
```

### Testing

```bash
docker-compose exec app cargo test
```

## License

[Your License Here]
