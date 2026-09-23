# Pigeon Voyageur (pigeonv)

A self-expiring paste service. Drop in a message and a date, get a link.
The paste is served over HTTP and returns 404 once the date has passed.
Built with Rust (Axum + askama + SQLite).

## Quickstart
```bash
    git clone https://codeberg.org/efournierrobert/pigeonv.git
    cd pigeonv
    mkdir -p db        # only on a fresh clone — db/ is gitignored
    cargo run           # run from the repo root
```

Then open http://127.0.0.1:8080/ . Migrations in `migrations/` apply
themselves on startup — there's no separate migrate step.

## Caveats

- Run it from the repo root — the database path and the `/static` mount
  are resolved relative to the working directory.
- Binds to 127.0.0.1:8080 only (localhost). Not network-accessible, and
  there's no `PORT` env var.
- Data lives in `db/pigeonv.db` (gitignored) — each machine keeps its own.
- The interface and error messages are in French.
- A paste needs a date after today (UTC); when it expires the link 404s,
  though the row is never deleted from the database.
