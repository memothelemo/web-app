use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{App, HttpServer};

use anyhow::{Context, Result};

use backend_lib::config::{server_address, server_port};
use backend_lib::db::establish_db_pool;
use backend_lib::logger;

mod api;

/// Preloads any prequisities to the program before it runs
/// the app web server (it includes logging)
fn preload() -> Result<()> {
    logger::init_logger()?;

    log::debug!("Loading .env file (optional)");
    dotenv::dotenv().ok();
    Ok(())
}

#[actix_web::main]
async fn main() -> Result<()> {
    preload()?;

    let governor_conf = GovernorConfigBuilder::default()
        .per_second(1)
        .burst_size(3)
        .finish()
        .with_context(|| "failed to setup ratelimiter")?;

    let address = server_address()?;
    let port = server_port()?;

    let db = establish_db_pool().await?;

    log::info!(
        "Launching actix-web server (address = {}; port = {})",
        address,
        port
    );

    let server = HttpServer::new(move || {
        App::new()
            // general API
            .service(api::index)
            .service(api::is_available)
            // web-app Letters API
            .service(api::letters::get_public)
            .service(api::letters::post)
            // web-app Report API
            .service(api::reports::report_letter)
            .service(api::reports::get_pending_letters)
            // middleware
            .wrap(Governor::new(&governor_conf))
            .app_data(db.clone())
    })
    .bind((address, port))?;

    log::info!("Listening at http://{}:{}", address, port);
    server.run().await.with_context(|| "server error")
}
