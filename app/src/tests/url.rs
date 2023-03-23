#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use tower::Service;
    use tower::ServiceExt;

    use crate::{
        app,
        handlers::url::{CreateAliasResponse, CreateAliasRouteParams},
    };

    fn create_url_request() -> CreateAliasRouteParams {
        CreateAliasRouteParams {
            url: "http://testurl.com".to_string(),
        }
    }

    // fn create_post_response() -> CreateUrlResponse {
    //     CreateUrlResponse {
    //         url: "http://testurl.com".to_string(),
    //         short_url_code: "testurl".to_string(),
    //     }
    // }

    #[tokio::test]
    async fn test_full_url_flow() {
        let mut app = app().await;

        // Create shortened URL
        let request = Request::builder()
            .method(http::Method::POST)
            .uri("/")
            .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Body::from(
                serde_json::to_vec(&create_url_request()).unwrap(),
            ))
            .unwrap();
        let response = app
            // .await
            .ready()
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
        let create_response: CreateAliasResponse = serde_json::from_slice(&body).unwrap();

        // Make a request using the shortened URL
        let request = Request::builder()
            .uri(format!("/{}", create_response.short_url_code))
            .body(Body::empty())
            .unwrap();
        let response = app
            // .await
            .ready()
            .await
            .unwrap()
            .call(request)
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::TEMPORARY_REDIRECT);
    }
}
