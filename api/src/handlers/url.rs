use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
    Json,
};
use http::StatusCode;
use storage::{create_url, get_url, CreateUrl};

use crate::{errors::ApiError, validate::validate, AppState};

#[derive(Clone, Debug, Deserialize, Serialize, Validate, PartialEq, Eq)]
pub struct CreateAliasRouteParams {
    pub url: String,
}

impl From<CreateAliasRouteParams> for CreateUrl {
    fn from(params: CreateAliasRouteParams) -> Self {
        Self { url: params.url }
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

    let res = create_url(&state.db_conn, new_url.into()).await?;
    let url_code = format!("{:x}", res.id);

    Ok((
        StatusCode::OK,
        Json(CreateAliasResponse {
            url: res.url,
            short_url_code: url_code,
        }),
    ))
}

/// GET "/{shortened_url}"
/// Makes a GET request to original url
/// If short url code is requested that doesn't exist, a 404 error is returned
pub async fn get_alias_endpoint(
    Path(short_url_code): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, ApiError> {
    let url_code = i32::from_str_radix(&short_url_code, 16)?;
    let response = get_url(&state.db_conn, url_code).await?;

    if let Some(response) = response {
        let redirect = Redirect::temporary(&response.url.clone());
        Ok(redirect)
    } else {
        Err(ApiError::NotFound(
            "The url alias you requested does not exist.".to_string(),
        ))
    }
}
