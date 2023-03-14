use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
    Json,
};
use http::StatusCode;

use crate::{errors::ApiError, validate::validate, SharedState};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Validate, PartialEq, Eq)]
pub struct CreateUrlRouteParams {
    pub url: String,
    // This will generate a short url code if the user does not provide their own
    #[serde(default = "generate_short_url")]
    pub short_url_code: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct CreateUrlResponse {
    pub url: String,
    pub short_url_code: String,
}

/// POST "/"
/// Creates a shortened url
pub async fn create_url_endpoint(
    State(state): State<SharedState>,
    Json(payload): Json<CreateUrlRouteParams>,
) -> Result<impl IntoResponse, ApiError> {
    validate(&payload)?;
    let short_url_code = payload.short_url_code;

    state
        .write()
        .unwrap()
        .db
        .insert(short_url_code.clone(), payload.url.clone());

    Ok((
        StatusCode::OK,
        Json(CreateUrlResponse {
            url: payload.url,
            short_url_code,
        }),
    ))
}

/// GET "/{shortened_url}"
/// Makes a GET request to original url
/// If short url code is requested that doesn't exist, a 404 error is returned
pub async fn get_url_endpoint(
    Path(short_url_code): Path<String>,
    State(state): State<SharedState>,
) -> Result<impl IntoResponse, ApiError> {
    let db = &state.read().unwrap().db;
    let base_url = db.get(&short_url_code);

    if let Some(url) = base_url {
        let redirect = Redirect::temporary(&url.clone());
        return Ok(redirect);
    } else {
        return Err(ApiError::NotFound(
            "The short url code you requested does not exist".to_string(),
        ));
    }
}

pub fn generate_short_url() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_url() {
        let shared_state = SharedState::default();
        let url = "http://testurl.com";
        let short_url_code = "testurl";

        let request = CreateUrlRouteParams {
            url: url.to_string(),
            short_url_code: short_url_code.to_string(),
        };
        let expected_response = CreateUrlResponse {
            url: url.to_string(),
            short_url_code: short_url_code.to_string(),
        };

        let response = create_url_endpoint(State(shared_state), Json(request))
            .await
            .unwrap()
            .into_response();

        assert_eq!(response.status(), StatusCode::OK);

        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let response_body: CreateUrlResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(response_body, expected_response);
    }

    #[tokio::test]
    async fn test_get_url() {
        let short_url_code = "testurl";
        let shared_state = SharedState::default();
        shared_state
            .write()
            .unwrap()
            .db
            .insert(short_url_code.to_string(), "http://testurl.com".to_string());

        let response = get_url_endpoint(Path(short_url_code.to_string()), State(shared_state))
            .await
            .unwrap()
            .into_response();

        assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    }
}
