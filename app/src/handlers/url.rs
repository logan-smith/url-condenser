use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
    Json,
};
use http::StatusCode;
use storage::{create_url, get_url_by_code, CreateUrl};

use crate::{errors::ApiError, validate::validate, AppState};

#[derive(Clone, Debug, Deserialize, Serialize, Validate, PartialEq, Eq)]
pub struct CreateAliasRouteParams {
    pub url: String,
    // This will generate a short url code if the user does not provide their own
    #[serde(default = "generate_short_url")]
    pub short_url_code: String,
}

impl From<CreateAliasRouteParams> for CreateUrl {
    fn from(params: CreateAliasRouteParams) -> Self {
        Self {
            url: params.url,
            short_url_code: params.short_url_code,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct CreateAliasResponse {
    pub url: String,
    pub short_url_code: String,
}

/// POST "/"
/// Creates a shortened url
pub async fn create_alias_endpoint(
    State(state): State<AppState>,
    Json(payload): Json<CreateAliasRouteParams>,
) -> Result<impl IntoResponse, ApiError> {
    validate(&payload)?;
    let new_url = payload.clone();

    let res = create_url(&state.db_conn, new_url.into()).await.unwrap(); // todo: seorm error handling

    Ok((
        StatusCode::OK,
        Json(CreateAliasResponse {
            url: res.url,
            short_url_code: res.short_url_code,
        }),
    ))
    // Ok((StatusCode::OK, "string"))
}

/// GET "/{shortened_url}"
/// Makes a GET request to original url
/// If short url code is requested that doesn't exist, a 404 error is returned
pub async fn get_alias_endpoint(
    Path(short_url_code): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let response = get_url_by_code(&state.db_conn, short_url_code)
        .await
        .unwrap();

    if let Some(response) = response {
        let redirect = Redirect::temporary(&response.url.clone());
        Ok(redirect)
    } else {
        Err(ApiError::NotFound(
            "The short url code you requested does not exist".to_string(),
        ))
    }
}

pub fn generate_short_url() -> String {
    uuid::Uuid::new_v4().to_string()
}
