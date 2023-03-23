// use crate::database::Pool;
// use crate::errors::ApiError;

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "urls")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub url: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

pub struct CreateAliasRouteParams {
    pub url: String,
}

impl From<CreateAliasRouteParams> for ActiveModel {
    fn from(params: CreateAliasRouteParams) -> Self {
        Self {
            id: sea_orm::ActiveValue::NotSet,
            url: sea_orm::ActiveValue::Set(params.url),
        }
    }
}
