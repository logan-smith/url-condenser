use ::entity::url;

pub fn url_model() -> url::Model {
    url::Model {
        id: 15,
        url: "http://www.google.com".to_owned(),
        short_url_code: "http://www.google.com".to_owned(),
    }
}

#[cfg(feature = "mock")]
pub fn prepare_mock_db() -> sea_orm::DatabaseConnection {
    use sea_orm::*;
    MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            [url::Model {
                id: 15,
                url: "http://www.google.com".to_owned(),
                short_url_code: "http://www.google.com".to_owned(),
            }],
            [url::Model {
                id: 16,
                url: "www.google.com".to_owned(),
                short_url_code: "test16".to_owned(),
            }],
        ])
        .append_exec_results([
            MockExecResult {
                last_insert_id: 15,
                rows_affected: 1,
            },
            MockExecResult {
                last_insert_id: 16,
                rows_affected: 1,
            },
        ])
        .into_connection()
}
