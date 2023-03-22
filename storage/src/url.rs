use ::entity::url;
use sea_orm::*;

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct CreateUrl {
    pub url: String,
    pub short_url_code: String,
}

pub async fn create_url(db: &DbConn, data: CreateUrl) -> Result<url::Model, DbErr> {
    url::ActiveModel {
        id: NotSet,
        url: Set(data.url.to_owned()),
        short_url_code: Set(data.short_url_code.to_owned()),
    }
    .insert(db)
    .await
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    #[cfg(feature = "mock")]
    async fn test_create_url() {
        use super::*;
        use crate::helpers::*;

        let db = &prepare_mock_db();
        let test_model = url_model();

        let request = CreateUrl {
            url: test_model.url.to_string(),
            short_url_code: test_model.short_url_code.to_string(),
        };

        let response = create_url(&db, request).await.unwrap();

        assert_eq!(response, test_model);
    }
}
