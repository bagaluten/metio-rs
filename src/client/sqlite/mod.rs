use super::{Client, ClientConfig, Result};
use sqlite::Connection;

#[derive(Clone, Default, Debug)]
pub struct SqliteClientConfig {
    pub path: String,
}

impl ClientConfig for SqliteClientConfig {
    fn new() -> Self {
        Self {
            path: String::from(""),
        }
    }
}

pub struct SqliteClient {
    conn: Connection,
    config: SqliteClientConfig,
}

impl Client for SqliteClient {
    type Config = SqliteClientConfig;

    fn new(config: &Self::Config) -> Result<Self> {
        Ok(Self {
            conn: sqlite::open(config.path.clone()).unwrap(),
            config: config.clone(),
        })
    }

    fn get_config(&self) -> Result<Self::Config> {
       Ok(self.config.clone())
    }
}
