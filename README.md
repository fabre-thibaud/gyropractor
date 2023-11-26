# :rotating_light: gyropractor

Relay tool between to push alerts into a websocket

## Running the server

First build and install the package. You need to have `rustc` and `cargo` installed in your environment.

:warning: **This has *only* been tested on Ubuntu 22.04 with Rust 1.74**

```
cargo install --profile release --locked --path . --root $HOME/.local/gyropractor
```

`gyropractor` should now be compiled and available in your path

Before running the server, you need to configure a few environment variables to setup database connection information and web server listen parameters.

You can refer to the self explanatory `.env.dist` to see what variables should be set.

You can then run the server:

```
gyropractor
```

## Running tests

You will need to have Docker and docker-compose installed in your environmnent

The K6 test suite bootstraps its own environment in Docker Compose with 3 containers:

- a `postgres` container running the database backend
- a `gyropractor` container running the server
- a `grafana/k6` container that runs the load test-suite against the `gyropractor` container

### Functional tests

To run the whole functional suite

```
docker compose run --rm k6-test functional
```

To run a specific test

```
docker-compose run --rm k6-test run test/my-test.js
```

### Load tests

:warning: **The test-suite runs for 10m30s and can be quite CPU-intensive.**

```
docker compose run --rm k6-test load
```
