use crate::database::Pool;
use crate::errors::ApiError;
use crate::schema::urls;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(
    Clone, Debug, Serialize, Deserialize, PartialEq, Eq, Queryable, Identifiable, Insertable,
)]
pub struct Url {
    id: i32,
    body: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct NewUrl {
    url: String,
}

impl From<String> for NewUrl {
    fn from(url: String) -> Self {
        NewUrl { url }
    }
}
