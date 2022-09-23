use anyhow::{Context, Result};

use std::net::Ipv4Addr;
use std::str::FromStr;

pub fn server_port() -> Result<u16> {
    #[rustfmt::skip]
    fn default() -> u16 {
        #[cfg(not(feature = "heroku"))] { 3080 }
        #[cfg(feature = "hosting")] { 80 }
    }

    if let Ok(port) = std::env::var("PORT") {
        port.parse::<u16>()
            .with_context(|| "failed to parse u16 with PORT environment variable")
    } else {
        Ok(default())
    }
}

pub fn server_address() -> Result<Ipv4Addr> {
    #[rustfmt::skip]
    fn default() -> Ipv4Addr {
        #[cfg(not(feature = "heroku"))] { Ipv4Addr::new(127, 0, 0, 1) }
        #[cfg(feature = "hosting")] { Ipv4Addr::new(127, 0, 0, 1) }
    }

    if let Ok(host) = std::env::var("HOST") {
        Ipv4Addr::from_str(&host)
            .with_context(|| "failed to parse IpAddr with HOST environment variable")
    } else {
        Ok(default())
    }
}
