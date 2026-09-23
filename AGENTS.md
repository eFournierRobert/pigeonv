# AGENTS.md

Pigeonv is a single-binary Rust web app (a self-expiring paste service): axum 0.8 + askama templates + SQLite via sqlx, one `main.rs`, no tests, no CI, no formatter config.

## Commands

- `cargo run` — build and start the server. It listens on **0.0.0.0:8080 hardcoded** (no `PORT` env, all interfaces — locally it is network-reachable); the process panics if the port is taken.
- Docker: `docker compose -f docker/docker-compose.yml up --build`. Compose mounts `./db` (relative to the compose file, i.e. `docker/db/`) so data persists; the final image contains the binary plus `static/`.
- `cargo check` / `cargo clippy` / `cargo build --release` — the only verification available. There is no test suite (`cargo test` runs nothing).
- Migrations need no CLI step: `sqlx::migrate!().run(...)` applies `migrations/*.sql` automatically at startup. To change the schema, add a **new** timestamped `.sql` file to `migrations/` (never edit existing ones).

## Gotchas

- **Run from the repo root.** Both the DB url (`sqlite://db/pigeonv.db`) and the static mount (`ServeDir::new("static")`) are relative to the process CWD.
- The `db/` directory is gitignored and must exist before first run (sqlx can create the database file but not its parent directory). Each machine has its own local DB.
- **Askama templates live in `templates/`**, outside `src/`. Each `src/templates/*.rs` is a struct annotated `#[template(path = "...html")]` — adding a field to a template requires updating both the HTML and the struct. All page templates `{% include "title-bar.html" %}`.
- **The app is in French**: UI strings, error messages, and the form field name `valeur` (meaning "message"). The form key `valeur` must match the `name="valeur"` attribute in `templates/index.html` — renaming one breaks deserialization of `NewMessageForm`.
- **Expiration semantics**: a message's expiration `NaiveDate` must be *strictly after* today (UTC); on read, an expired message returns 404 ("Lien invalide") but the row is **never deleted** from SQLite.
- SQL uses bound parameters, never string interpolation; SQLite is compiled via `libsqlite3-sys` (bundled) so no system SQLite or `DATABASE_URL` is required.

## Layout

- `src/main.rs` — server setup, router, and `AppState { db }` (the only shared state).
- `src/handlers/` — axum route handlers only; `src/services/` — business logic + validation (returns `ServiceErrors`); `src/database/` — raw sqlx queries.
- Routes: `GET /` (form), `POST /submit` (creates paste, returns link page), `GET /m/{uuid}`, `/static/*`.
- Errors: `ServiceErrors` in `src/templates/error.rs` maps each case to an HTTP status + French message rendered via `error.html`.
- Logging: DB failures are logged via `tracing::error!` in `services/mod.rs` before being mapped. Note the select branch also logs when a uuid simply doesn't exist (sqlx `RowNotFound`), so `ERROR` lines on the console are expected for requests to dead/expired links — they don't indicate a real failure.
