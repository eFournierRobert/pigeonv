# Pigeon Voyageur (pigeonv)

A self-expiring paste service. Drop in a message and a date, get a link.
The paste is served over HTTP and returns 404 once the date has passed.
Built with Rust (Axum + askama + SQLite).

## Quickstart
```bash
    git clone https://github.com/eFournierRobert/pigeonv.git
    cd pigeonv
    mkdir -p db        # only on a fresh clone — db/ is gitignored
    cargo run           # run from the repo root
```

Then open http://127.0.0.1:8080/ . Migrations in `migrations/` apply
themselves on startup — there's no separate migrate step.

## Quickstart (Docker)

```bash
    git clone https://github.com/eFournierRobert/pigeonv.git
    cd pigeonv
    docker compose -f docker/docker-compose.yml up --build
```

Then open http://127.0.0.1:8080/ . No `db/` prep needed — the compose file
mounts `./db` for the database, so pastes persist across rebuilds.

## Caveats

- Run it from the repo root — the database path and the `/static` mount
  are resolved relative to the working directory.
- Listens on port 8080 (hardcoded, no `PORT` env var) bound to all
  interfaces, so a local run is reachable on your network.
- Data lives in `db/pigeonv.db` (gitignored) — each machine keeps its own.
  With Docker, the compose file persists it in `docker/db/` instead.
- The interface and error messages are in French.
- A paste needs a date after today (UTC); when it expires the link 404s,
  though the row is never deleted from the database.
