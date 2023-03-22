use axum::{
    extract::{Path, State},
    response::{IntoResponse, Redirect},
    Json,
};
use http::StatusCode;
use sea_orm::{ActiveValue, DatabaseConnection};
use storage::{create_url, get_url_by_code, CreateUrl};

use crate::{errors::ApiError, validate::validate, AppState};
use entity::url::ActiveModel;
// use entity::url::Model as Url;

// #[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
// pub struct NewUrl {
//     url: String,
// }

// impl From<String> for NewUrl {
//     fn from(url: String) -> Self {
//         NewUrl { url }
//     }
// }

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

// impl From<CreateAliasRouteParams> for ActiveModel {
//     fn from(params: CreateAliasRouteParams) -> Self {
//         Self {
//             id: ActiveValue::NotSet,
//             url: params.url,
//             short_url_code: params.short_url_code,
//         }
//     }
// }

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
    // let db = &state.read().unwrap().db;
    // let base_url = db.get(&short_url_code);

    // let db_conn = &state.clone().write().unwrap().db_conn;
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

    // return Ok(ApiError::NotFound(
    //     "The short url code you requested does not exist".to_string(),
    // ));
}

pub fn generate_short_url() -> String {
    uuid::Uuid::new_v4().to_string()
}

// #[cfg(test)]
// mod tests {
//     use std::sync::Arc;
//     use std::sync::RwLock;

//     use crate::config::Config;

//     use super::*;
//     use entity::url;
//     use migration::SeaRc;
//     use sea_orm::Database;
//     use sea_orm::{entity::prelude::*, entity::*, DatabaseBackend};

//     #[tokio::test]
//     async fn test_create_url() {
//         // Create MockDatabase with mock execution result
//         // let db_conn = MockDatabase::new(DatabaseBackend::Postgres)
//         //     .append_query_results([
//         //         [url::Model {
//         //             id: 15,
//         //             url: "http://www.google.com".to_owned(),
//         //             short_url_code: "http://www.google.com".to_owned(),
//         //         }],
//         //         [url::Model {
//         //             id: 16,
//         //             url: "www.google.com".to_owned(),
//         //             short_url_code: "test16".to_owned(),
//         //         }],
//         //     ])
//         //     .append_exec_results([
//         //         MockExecResult {
//         //             last_insert_id: 15,
//         //             rows_affected: 1,
//         //         },
//         //         MockExecResult {
//         //             last_insert_id: 16,
//         //             rows_affected: 1,
//         //         },
//         //     ])
//         //     .into_connection();
//         let config = crate::config::CONFIG.clone();
//         let db_conn = Database::connect(config.database_url)
//             .await
//             .expect("Database connection failed");

//         let state = AppState { db_conn: db_conn };
//         // let state: SharedState = Arc::new(RwLock::new(state));
//         // let shared_state = SharedState::default();
//         let url = "http://www.google.com";
//         let short_url_code = "testurl15";

//         let request = CreateAliasRouteParams {
//             url: url.to_string(),
//             short_url_code: short_url_code.to_string(),
//         };
//         let expected_response = CreateAliasResponse {
//             url: url.to_string(),
//             short_url_code: short_url_code.to_string(),
//         };

//         let response = create_alias_endpoint(State(state), Json(request))
//             .await
//             .unwrap()
//             .into_response();

//         assert_eq!(response.status(), StatusCode::OK);

//         let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
//         let response_body: CreateAliasResponse = serde_json::from_slice(&body).unwrap();
//         assert_eq!(response_body, expected_response);
//     }

//     //     #[tokio::test]
//     //     async fn test_get_url() {
//     //         let short_url_code = "testurl";
//     //         let shared_state = SharedState::default();
//     //         shared_state
//     //             .write()
//     //             .unwrap()
//     //             .db
//     //             .insert(short_url_code.to_string(), "http://testurl.com".to_string());

//     //         let response = get_url_endpoint(Path(short_url_code.to_string()), State(shared_state))
//     //             .await
//     //             .unwrap()
//     //             .into_response();

//     //         assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
//     //     }
// }
