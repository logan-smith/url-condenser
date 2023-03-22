// use diesel::{pg::PgConnection, r2d2::ConnectionManager};

// pub type Pool<T> = r2d2::Pool<ConnectionManager<T>>;
// pub type PostgresPool = Pool<PgConnection>;

// #[cfg(feature = "postgres")]
// pub type PoolType = PostgresPool;

use entity::sea_orm;
use sea_orm::DatabaseConnection;

pub struct Database {
    pub connection: DatabaseConnection,
}

impl Database {
    pub async fn new() -> Self {
        let config = CONFIG.clone();
        let connection = sea_orm::Database::connect(config.database_url)
            .await
            .expect("Could not connect to database");

        Database { connection }
    }

    pub fn get_connection(&self) -> &DatabaseConnection {
        &self.connection
    }
}
