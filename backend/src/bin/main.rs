use env_logger::Env;

use figment::providers::{Format, Toml};
use figment::Figment;

use frontend_lib::db::create_db_client;
use frontend_lib::Config;

use rocket::Build;

/// Creates a Rocket server
///
/// # Panics
///
/// It will panic if the configuration file (Rocket.toml) is missing
/// or has invalid fields or values inside than it expects.
pub fn server(figment: Figment) -> rocket::Rocket<Build> {
    let config: Config = figment.extract().expect("could not read configuration");
    let db = create_db_client(&config.database_url, &config.database_key);

    rocket::custom(figment).manage(db).manage(config)
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
    let _ = server(
        Figment::from(rocket::Config::debug_default()).merge(Toml::file("Rocket.toml").nested()),
    )
    .launch()
    .await?;
    Ok(())
}
