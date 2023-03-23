pub mod helpers;

use entity::url;
use helpers::*;
use storage::url::*;

#[tokio::test]
async fn main() {
    let db = &prepare_mock_db();

    // Mock test create_url
    {
        let test_model = url_model();

        let request = CreateUrl {
            url: test_model.url.to_string(),
        };

        let response = create_url(&db, request).await.unwrap();

        assert_eq!(response, test_model);
    }
}
