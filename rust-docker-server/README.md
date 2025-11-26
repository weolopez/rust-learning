# rust-docker-server

Minimal Rust HTTP server designed to run in a container. Serves static files from `static/` and exposes a `/health` endpoint.

To build locally:

```bash
cargo build --manifest-path rust-docker-server/Cargo.toml --release
```

To run locally:

```bash
cargo run --manifest-path rust-docker-server/Cargo.toml
```

To build the Docker image:

```bash
docker build -t rust-docker-server -f rust-docker-server/Dockerfile .
```

To run the container:

```bash
docker run --rm -p 8080:8080 rust-docker-server
```

Tests
-----

An integration test is provided that starts the server and verifies the `/health` endpoint. To run tests:

```bash
cargo test --manifest-path rust-docker-server/Cargo.toml
```

Docker notes
------------

The included `Dockerfile` performs a two-stage build. To build and run the image locally:

```bash
docker build -t rust-docker-server -f rust-docker-server/Dockerfile .
docker run --rm -p 8080:8080 rust-docker-server
```

If you run tests inside CI, prefer building the binary with `cargo build --release` first so the integration test uses the release binary path.
