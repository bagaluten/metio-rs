use super::{Client, ClientConfig, Result};
use crate::types::{Event, EventType};
use sqlite::{Connection, State};

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

    fn get_events(&self, count: u32) -> Result<Box<[Event]>> {
        let mut statement = self
            .conn
            .prepare("SELECT * FROM event limit ?")
            .map_err(|e| e.to_string())?;

        statement
            .bind::<&[(usize, i64)]>(&[(1, count.into())])
            .map_err(|e| e.to_string())?;

        let mut events: Vec<Event> = Vec::new();
        while let Ok(State::Row) = statement.next() {
            let event_id = statement
                .read::<String, _>("id")
                .map_err(|e| e.to_string())?;

            let object_id = statement
                .read::<String, _>("objectId")
                .map_err(|e| e.to_string())?;

            let event_type = statement
                .read::<String, _>("event_type")
                .map_err(|e| e.to_string())?
                .parse::<EventType>()?;

            let timestamp = statement
                .read::<String, _>("timestamp")
                .map_err(|e| e.to_string())?;

            let timestamp = timestamp
                .parse::<chrono::DateTime<chrono::Utc>>()
                .map_err(|e| e.to_string())?;

            events.push(Event {
                event_id,
                object_id,
                event_type,
                timestamp,
            })
        }

        Ok(events.into_boxed_slice())
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
                "INSERT OR REPLACE INTO event
                (id, event_type, objectId, timestamp)
                VALUES (?, ?, ?, ?);",
            )
            .map_err(|e| e.to_string())?;

        for i in 0..1000 {
            statement
                .bind(
                    &[
                        (1, format!("event-{}", i).as_str()),
                        (2, "metio.bagaluten.io/test-event/v1"),
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

    #[test]
    fn test_get_events() -> Result<()> {
        use super::*;
        let config = SqliteClientConfig {
            path: ":memory:".to_string(),
            create_testdata: true,
        };

        let client = SqliteClient::new(&config)?;
        let events = client.get_events(10)?;
        assert_eq!(events.len(), 10);
        Ok(())
    }

    #[test]
    fn create_test_data_twice() -> Result<()> {
        use super::*;
        let config = SqliteClientConfig {
            path: ":memory:".to_string(),
            create_testdata: true,
        };

        let client = SqliteClient::new(&config)?;
        let events = client.get_events(10)?;
        assert_eq!(events.len(), 10);

        client.generate_testdata()?;

        let events = client.get_events(10)?;
        assert_eq!(events.len(), 10);
        Ok(())
    }
}
