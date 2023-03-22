# URL Condenser

A web framework built upon Axum using the Rust language.
Provides url aliases, and redirects requests for them.

# Table of Contents

- [URL Condenser](#url-condenser)
- [Table of Contents](#table-of-contents)
  - [Installation](#installation)
  - [Running the Server](#running-the-server)
    - [Running Tests](#running-tests)
  - [Docker](#docker)
  - [Generating documentation](#generating-documentation)
  - [Endpoints](#endpoints)
    - [Healthcheck](#healthcheck)
      - [Response](#response)
    - [Create URL Alias](#create-url-alias)
      - [Request](#request)
      - [Response](#response-1)
    - [Use URL Alias](#use-url-alias)
      - [Response](#response-2)
  - [License](#license)

## Installation

Create an .env file at the root of your project:

```shell
touch .env
```

Now add environment values for local development:

```ini
DATABASE_URL=postgresql://username:password@localhost:5432/url-condenser
RUST_BACKTRACE=0
RUST_LOG="url_condenser=debug,axum=info"
SERVER=127.0.0.1:8000
```

## Running the Server

First you must run Postgres in docker:

```shell
docker-compose up
```

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
docker build -t url_condenser .
```

Once the image is built, you can run the container in port 3000:

```shell
docker run -it --rm --env-file=.env.docker -p 3000:3000 --name url_condenser url_condenser
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

### Create URL Alias

Creates a URL alias. If user does not provide a `short_url_code`, one will be generated for them

`POST /`

#### Request

| Param          | Type   | Description                         | Required | Validations |
| -------------- | ------ | ----------------------------------- | :------: | ----------- |
| url            | String | The base url to create an alias for |   yes    | none        |
| short_url_code | String | The url alias                       |    no    | none        |

#### Response

`200 OK`

```json
{
  "url": "http://www.google.com",
  "short_url_code": "sample-url"
}
```

Example:

```shell
curl -X POST \
  http://127.0.0.1:8000/ \
  -H 'Content-Type: application/json' \
  -d '{
    "url": "http://www.google.com",
    "short_url_code": "sample-url"
}'
```

### Use URL Alias

Redirects request from alias URL to original provided long url

`GET /{short_url_code}`

#### Response

`307 TEMPORARY REDIRECT`

Example:

```shell
curl -X GET http://127.0.0.1:8000/sample-url
```

## License

This project is licensed under:

- MIT license (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
