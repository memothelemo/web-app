use anyhow::{anyhow, Context, Result};

use std::net::Ipv4Addr;
use std::str::FromStr;

macro_rules! generate_16_bytes_fn {
    ($name:ident, $env:literal) => {
        pub fn $name() -> Result<Vec<u8>> {
            let key = std::env::var($env).with_context(|| concat!($env, " is not present"))?;
            if key.len() != 16 {
                Err(anyhow!(concat!($env, " is not 16 characters/bytes long!")))
            } else {
                Ok(key.as_bytes().to_vec())
            }
        }
    };
}

// not secure but close enough to be
pub struct AuthParams {
    pub salt: [u8; 16],
    pub token: Vec<u8>,
    pub reg: [u8; 16],

    pub secret_key: [u8; 16],
}

generate_16_bytes_fn!(reg_key, "REGISTER_KEY");
generate_16_bytes_fn!(salt_key, "SALT_KEY");
generate_16_bytes_fn!(encoding_key, "ENCODING_KEY");

generate_16_bytes_fn!(secret_aes_key, "SECRET_KEY");

pub fn server_port() -> Result<u16> {
    #[rustfmt::skip]
    fn default() -> u16 {
        #[cfg(not(feature = "hosting"))] { 3080 }
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
        #[cfg(not(feature = "hosting"))] { Ipv4Addr::new(127, 0, 0, 1) }
        #[cfg(feature = "hosting")] { Ipv4Addr::new(127, 0, 0, 1) }
    }

    if let Ok(host) = std::env::var("HOST") {
        Ipv4Addr::from_str(&host)
            .with_context(|| "failed to parse IpAddr with HOST environment variable")
    } else {
        Ok(default())
    }
}
