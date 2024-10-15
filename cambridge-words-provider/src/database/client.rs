use crate::configuration::PostgresDatabaseConfig;

use super::prepare_postgres_con;

use anyhow::Result;
use tracing::error;

#[derive(Clone)]
pub struct DatabaseClient {
    pub postgres_con: sqlx::postgres::PgPool,
}


impl DatabaseClient {
    pub async fn new(
        postgres_config: &PostgresDatabaseConfig,
    ) -> Result<Self> {

        let postgres_con = prepare_postgres_con(
            postgres_config,
        ).await;

        match postgres_con {
            Ok(_) => {},
            Err(e) => {
                error!("Failed to prepare Postgres connection: {}", e);
                return Err(e.into());
            }
        }

        Ok(Self {
            postgres_con: postgres_con.unwrap(),
        })
    }
}