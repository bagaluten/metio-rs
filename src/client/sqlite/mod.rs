use super::{Client, ClientConfig, Result};
use sqlite::Connection;

#[derive(Clone, Debug)]
pub struct SqliteClientConfig {
    // path to the sqlite database file.
    // use ':memory:' to create an in-memory database (default).
    pub path: String,

    // defines whether to create test data or not.
    pub create_testdata: bool,
}

impl Default for SqliteClientConfig {
    fn default() -> Self {
        Self {
            path: ":memory:".to_string(),
            create_testdata: false,
        }
    }
}

impl ClientConfig for SqliteClientConfig {}

pub struct SqliteClient {
    conn: Connection,
    config: SqliteClientConfig,
}

impl Client for SqliteClient {
    type Config = SqliteClientConfig;

    fn new(config: &Self::Config) -> Result<Self> {
        let conn = sqlite::open(config.path.clone()).map_err(|e| e.to_string())?;
        let client = Self {
            conn,
            config: config.clone(),
        };

        client.ensure_schema()?;

        if config.create_testdata {
            client.generate_testdata()?;
        }

        Ok(client)
    }

    fn get_config(&self) -> Result<Self::Config> {
        Ok(self.config.clone())
    }
}

impl SqliteClient {
    // Generate some test events.
    // This function can be quite handy when you develop your
    // application and need some data for testing.
    fn generate_testdata(&self) -> Result<()> {
        let mut statement = self
            .conn
            .prepare(
                "INSERT INTO event (id, event_type, objectId, timestamp)
                VALUES (?, ?, ?, ?);",
            )
            .map_err(|e| e.to_string())?;

        for i in 0..1000 {
            statement
                .bind(
                    &[
                        (1, format!("event-{}", i).as_str()),
                        (2, "metio.bagaluten.io/test-event"),
                        (3, "testObject"),
                        (4, chrono::Utc::now().to_string().as_str()),
                    ][..],
                )
                .map_err(|e| e.to_string())?;
            statement.next().map_err(|e| e.to_string())?;
            statement.reset().map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    // ensure all tables are created
    fn ensure_schema(&self) -> Result<()> {
        let _ = self
            .conn
            .execute(
                "CREATE TABLE IF NOT EXISTS event (
                id TEXT PRIMARY KEY,
                event_type TEXT NOT NULL,
                objectId TEXT,
                timestamp INTEGER NOT NULL
            );",
            )
            .map_err(|e| e.to_string())?;

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::Result;

    #[test]
    fn test_sqlite_client() -> Result<()> {
        use super::*;
        let config = SqliteClientConfig {
            path: ":memory:".to_string(),
            create_testdata: true,
        };
        SqliteClient::new(&config)?;
        Ok(())
    }

    #[test]
    fn default_config() {
        use super::*;
        let congig = SqliteClientConfig::default();
        assert_eq!(congig.path, ":memory:".to_string());
        assert_eq!(congig.create_testdata, false);
    }
}
