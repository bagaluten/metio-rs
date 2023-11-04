use sqlite::Connection;
use super::{ClientConfig, Client};

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
}

impl Client for SqliteClient {
    type Config = SqliteClientConfig;

    fn new(config: &Self::Config) -> Self {
        Self {
            conn: sqlite::open(config.path.clone()).unwrap(),
        }
    }

    fn get_config(&self) -> &Self::Config {
        unimplemented!()
    }
}


