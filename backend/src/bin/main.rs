use env_logger::Env;

use figment::providers::{Format, Toml};
use figment::Figment;

use backend_lib::db::create_db_client;
use backend_lib::Config;

#[cfg(not(debug_assertions))]
use rocket::fs::{FileServer, relative};
use rocket::Build;

mod routes;
#[cfg(test)]
mod tests;

/// Creates a Rocket server
///
/// # Panics
///
/// It will panic if the configuration file (Rocket.toml) is missing
/// or has invalid fields or values inside than it expects.
pub fn server(figment: Figment) -> rocket::Rocket<Build> {
    let config: Config = figment.extract().expect("could not read configuration");
    let db = create_db_client(&config.database_url, &config.database_key);

    let mut rocket = rocket::custom(figment).manage(db).manage(config);

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

    #[cfg(debug_assertions)]
    let config = rocket::Config::debug_default();

    #[cfg(not(debug_assertions))]
    let config = rocket::Config::release_default();

    let _ = server(Figment::from(config).merge(Toml::file("Rocket.toml").nested()))
        .launch()
        .await?;

    Ok(())
}
