use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
    Json,
};
use http::StatusCode;

use crate::{errors::ApiError, validate::validate};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Validate, PartialEq, Eq)]
pub struct CreateUrlRouteParams {
    pub url: String,
    #[serde(default = "generate_short_url")]
    // This will generate a short url code if the user does not provide their own
    pub short_url_code: String,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct CreateUrlResponse {
    pub short_url_code: String,
    pub url: String,
}

/// POST "/"
/// Creates a shortened url
pub async fn create_url_endpoint(
    State(mut urls): State<HashMap<String, String>>,
    Json(payload): Json<CreateUrlRouteParams>,
) -> Result<impl IntoResponse, ApiError> {
    validate(&payload)?;

    let short_url_code = payload.short_url_code;

    let response = urls.insert(short_url_code.clone(), payload.url.clone());

    if let Some(inserted_url) = response {
        return Ok((
            StatusCode::OK,
            Json(CreateUrlResponse {
                short_url_code,
                url: inserted_url,
            }),
        ));
    } else {
        return Err(ApiError::InternalServerError(format!(
            "Failed to create short url code for url: {:?}",
            payload.url
        )));
    }
}

/// GET "/{shortened_url}"
/// Makes a GET request to original url
/// If short url code is requested that doesn't exist, a 404 error is returned
pub async fn get_url_endpoint(
    State(urls): State<HashMap<String, String>>,
    Path(short_url_code): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    let base_url = urls.get(&short_url_code);

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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[tokio::test]
//     async fn test_create_url() {
//         let hashmap: HashMap<String, String> = HashMap::new();
//         let request =

//         let response = create_url_endpoint(State(hashmap), Json(request))
//             .await
//             .unwrap()
//             .into_response();
//         assert_eq!(response.status(), StatusCode::OK);
//     }
// }
