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

2. Set up the database:

```bash
export DATABASE_URL=sqlite://db.sqlite3

cargo install sqlx-cli

sqlx database create

sqlx migrate run
```

3. Install dependencies:

```bash
cargo build
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

## API Examples

Test the API with these curl commands:

```bash
# Create a new todo
curl -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Buy groceries", "completed": false}'

# Create another todo
curl -X POST http://localhost:3000/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Learn Rust", "completed": false}'

# Get all todos
curl http://localhost:3000/todos

# Get a specific todo (replace 1 with the actual ID)
curl http://localhost:3000/todos/1

# Update a todo (replace 1 with the actual ID)
curl -X PATCH http://localhost:3000/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"completed": true}'

# Delete a todo (replace 1 with the actual ID)
curl -X DELETE http://localhost:3000/todos/1
```

## License

This project is open-sourced under the MIT License - see the LICENSE file for details.

## Acknowledgments

- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum)
- [SQLx](https://github.com/launchbadge/sqlx)
