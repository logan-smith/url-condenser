# URL Condenser

A web framework built upon Axum using the Rust language.
Provides shortened urls, and redirects requests for them.

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
    - [Create Shortened URL](#create-shortened-url)
    - [Use Short URL](#use-short-url)
  - [License](#license)

## Installation

Create an .env file at the root of your project:

```shell
touch .env
```

Now add environment values for local development:

```ini
RUST_BACKTRACE=0
RUST_LOG="url_condenser=debug,axum=info"
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

### Create Shortened URL

Creates a URL alias

`POST /`

#### Request

| Param          | Type   | Description                         | Required | Validations |
| -------------- | ------ | ----------------------------------- | :------: | ----------- |
| url            | String | The base url to create an alias for |   yes    | none        |
| short_url_code | String | The shortened url alias             |    no    | none        |

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
    "short_url_code": "sample-url",
}'
```

### Use Short URL

Redirects request from short URL to original provided long url

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
