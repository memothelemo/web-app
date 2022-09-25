pub mod config;
pub mod db;
pub mod logger;
pub mod models;
pub mod reqs;
pub mod schema;
pub mod utils;

#[macro_export]
macro_rules! vec_to_sized {
    ($unsized:expr, $sized:expr) => {
        for (&x, p) in $unsized.iter().zip($sized.iter_mut()) {
            *p = x;
        }
    };
}
