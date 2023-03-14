# URL Condenser

A web framework built upon Axum using the Rust language.
Provides shortened urls, and redirects requests for them.

# Table of Contents

- [API Exercise](#api-exercise)
- [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Running the Server](#running-the-server)
    - [Running Tests](#running-tests)
  - [Docker](#docker)
  - [Generating documentation](#generating-documentation)
  - [Endpoints](#endpoints)
    - [Healthcheck](#healthcheck)
      - [Response](#response)
  - [License](#license)

## Installation

Create an .env file at the root of your project:

```shell
touch .env
```

Now add environment values for local development:

```ini
RUST_BACKTRACE=0
RUST_LOG="api_exercise=debug,axum=info"
SERVER=127.0.0.1:8000
```

## Running the Server

To startup the server:

```shell
cargo run
```

### Running Tests

To run all of the tests:

```shell
cargo test
```

## Docker

To build a Docker image of the application:

```shell
docker build -t api_exercise .
```

Once the image is built, you can run the container in port 3000:

```shell
docker run -it --rm --env-file=.env.docker -p 3000:3000 --name api_exercise api_exercise
```

## Generating documentation

```shell
cargo doc --no-deps --open
```

## Endpoints

### Healthcheck

Determine if the system is healthy.

`GET /health`

#### Response

`200 OK`

Example:

```shell
curl -X GET http://127.0.0.1:3000/health
```

## License

This project is licensed under:

- MIT license (LICENSE-MIT or http://opensource.org/licenses/MIT)
