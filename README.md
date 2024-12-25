# todo-list-axum

A Todo List API built with Rust and Axum web framework.

## Prerequisites

- Rust (latest stable version)
- SQLite

## Setup

1. Clone the repository:

```bash
git clone https://github.com/sarmadgulzar/todo-list-axum.git
cd todo-list-axum
```

2. Install dependencies:

```bash
cargo build
```

3. Set up the database:

```bash
# Install sqlx-cli
cargo install sqlx-cli

# Create the database
sqlx database create

# Run migrations
sqlx migrate run
```

4. Run the server:

```bash
cargo run
```

## API Endpoints

- `GET /todos`: Get all todos.
- `POST /todos`: Create a new todo.
- `GET /todos/:id`: Get a todo by ID.
- `PATCH /todos/:id`: Update a todo by ID.
- `DELETE /todos/:id`: Delete a todo by ID.

## License

This project is open-sourced under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum)
- [SQLx](https://github.com/launchbadge/sqlx)
