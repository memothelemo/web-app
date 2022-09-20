use env_logger::Env;

use backend_lib::db::create_db_client;
use backend_lib::Config;

#[cfg(not(debug_assertions))]
use rocket::fs::{relative, FileServer};
use rocket::{Build, Config as RocketConfig};

mod routes;
#[cfg(test)]
mod tests;

/// Creates a Rocket server
///
/// # Panics
///
/// It will panic if the configuration file (Rocket.toml) is missing
/// or has invalid fields or values inside than it expects.
pub fn server() -> rocket::Rocket<Build> {
    let env_config: Config = Config {
        database_url: std::env::var("DATABASE_URL").expect("failed to get DATABASE_URL"),
        database_key: std::env::var("DATABASE_KEY").expect("failed to get DATABASE_URL"),
        salt_code: std::env::var("SALT_CODE").expect("failed to get SALT_CODE"),
    };

    let db = create_db_client(&env_config.database_url, &env_config.database_key);

    #[cfg(not(debug_assertions))]
    let config = RocketConfig {
        port: std::env::var("PORT").expect("failed to get PORT"),
        ..RocketConfig::debug_default()
    };

    #[cfg(debug_assertions)]
    let config = RocketConfig {
        port: 8000,
        ..RocketConfig::debug_default()
    };

    let mut rocket = rocket::custom(config).manage(db).manage(env_config);
    rocket = routes::api::apply(rocket);

    #[cfg(not(debug_assertions))]
    {
        rocket = rocket.mount("/", FileServer::from(relative!("../frontend/build")));
    }

    rocket
}

/// Preloads any prequisities to the program before it
/// runs the server (it includes logging)
fn preload() {
    #[cfg(debug_assertions)]
    let default_env = "debug";

    #[cfg(not(debug_assertions))]
    let default_env = "info";

    env_logger::builder()
        .parse_env(Env::new().default_filter_or(default_env))
        .init();
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    preload();

    let _ = server().launch().await?;
    Ok(())
}
