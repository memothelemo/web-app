use super::{create_db_client, MaybeQueryable, Queryable};

use crate::db::DbClient;
use crate::Config;

use anyhow::Result;
use derive_more::Display;
use serde::Deserialize;

#[cfg(feature = "test_log")]
use env_logger::Env;

use figment::providers::{Format, Toml};
use figment::Figment;

pub fn figment() -> Figment {
    Figment::from(rocket::Config::debug_default()).merge(Toml::file("Rocket.toml").nested())
}

pub fn client() -> DbClient {
    #[cfg(feature = "test_log")]
    env_logger::builder()
        .parse_env(Env::new().default_filter_or("debug"))
        .try_init()
        .ok();

    let config: Config = figment().extract().expect("could not load configuration");
    create_db_client(config.database_url, config.database_key)
}

mod queryable {
    use super::*;

    #[derive(Display)]
    #[display(fmt = "MyQuery({_0})")]
    struct MyQuery(&'static str);

    #[rocket::async_trait]
    impl Queryable for MyQuery {
        type Output = Info;

        async fn query_inner(self, client: &DbClient) -> Result<reqwest::Response> {
            Ok(client
                .from("query_test")
                .select("*")
                .eq("name", self.0)
                .single()
                .execute()
                .await?)
        }
    }

    #[derive(Debug, Deserialize, PartialEq, Eq)]
    struct Info {
        id: u64,
        created_at: String,
        name: String,
    }

    #[tokio::test]
    #[should_panic]
    async fn queryable_not_exists() {
        let db = client();
        MyQuery("foo").query(&db).await.expect("query failed");
    }

    #[tokio::test]
    async fn queryable() {
        let db = client();
        MyQuery("memo").query(&db).await.expect("query failed");
    }

    #[tokio::test]
    async fn maybe_queryable() {
        #[derive(Display)]
        #[display(fmt = "MyQuery({_0})")]
        struct MaybeQuery(&'static str);

        #[rocket::async_trait]
        impl MaybeQueryable for MaybeQuery {
            type Output = Info;

            async fn query_inner(self, client: &DbClient) -> Result<reqwest::Response> {
                Ok(client
                    .from("query_test")
                    .select("*")
                    .eq("name", self.0)
                    .single()
                    .execute()
                    .await?)
            }
        }

        let db = client();
        let query = MaybeQuery("foo").query(&db).await.expect("query failed");
        assert_eq!(query, None);

        let query = MaybeQuery("memo").query(&db).await.expect("query failed");
        assert!(query.is_some());
    }
}
