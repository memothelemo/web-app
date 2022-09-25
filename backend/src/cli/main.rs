use actix_governor::{Governor, GovernorConfigBuilder};

use actix_web::middleware;
use actix_web::{web, App, HttpServer};

use anyhow::{Context, Result};

use backend_lib::config::AuthParams;
use backend_lib::config::{
    encoding_key, reg_key, salt_key, secret_aes_key, server_address, server_port,
};

use backend_lib::db::establish_db_pool;
use backend_lib::{logger, vec_to_sized};

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

    let salt = salt_key()?;
    let encode_key = encoding_key()?;
    let register_key = reg_key()?;

    let secret_key = secret_aes_key()?;

    let db = establish_db_pool().await?;

    log::info!(
        "Launching actix-web server (address = {}; port = {})",
        address,
        port
    );

    let server = HttpServer::new(move || {
        let mut salt_sized = [0u8; 16];
        vec_to_sized!(salt, salt_sized);

        let mut reg_sized = [0u8; 16];
        vec_to_sized!(register_key, reg_sized);

        let mut secret_key_sized = [0u8; 16];
        vec_to_sized!(secret_key, secret_key_sized);

        App::new()
            .configure(api::apply)
            // middleware
            .wrap(Governor::new(&governor_conf))
            .wrap(middleware::Logger::default())
            .app_data(db.clone())
            .app_data(web::Data::new(AuthParams {
                salt: salt_sized,
                token: encode_key.clone(),
                reg: reg_sized,

                secret_key: secret_key_sized,
            }))
    })
    .bind((address, port))?;

    log::info!("Listening at http://{}:{}", address, port);
    server.run().await.with_context(|| "server error")
}
