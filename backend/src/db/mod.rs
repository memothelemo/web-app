use actix_web::web;
use anyhow::{Context, Result};

use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

pub mod letters;
pub mod reports;
pub mod state;

pub type DbPool = web::Data<r2d2::Pool<ConnectionManager<PgConnection>>>;

pub async fn establish_db_pool() -> Result<DbPool> {
    log::info!("Establishing Postgres connection");
    let database_url = std::env::var("DATABASE_URL")
        .with_context(|| "failed to get DATABASE_URL environment variable")?;

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Ok(web::Data::new(r2d2::Pool::builder().build(manager)?))
}
