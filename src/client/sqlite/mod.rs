use super::ClientConfig;
use super::{AsyncClient, EventFilter, Result};
use crate::types::{Event, EventType};
use async_trait::async_trait;
use futures::stream::TryStreamExt;
use sqlx::{Executor, Row, SqlitePool};
use std::collections::HashMap;

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
    pool: sqlx::SqlitePool,
    config: SqliteClientConfig,
}

#[async_trait]
impl AsyncClient for SqliteClient {
    type Config = SqliteClientConfig;

    async fn get_config(&self) -> Result<Self::Config> {
        Ok(self.config.clone())
    }

    async fn get_events(&self, count: u32) -> Result<Box<[Event]>> {
        return self.get_events_filtered(count, EventFilter::None).await;
    }

    async fn get_events_filtered(&self, count: u32, filter: EventFilter) -> Result<Box<[Event]>> {
        let mut events: Vec<Event> = Vec::new();
        let mut query_builder = build_query(count, filter);
        let query = query_builder.build();
        use sqlx::Execute;
        println!("{}", query.sql());

        let mut result = self.pool.fetch(query);
        while let Some(row) = result.try_next().await.map_err(|e| e.to_string())? {
            let event_id: String = row.get("id");
            let object_id: String = row.get("objectId");
            let event_type: String = row.get("event_type");
            let timestamp: String = row.get("timestamp");
            let payload: String = row.get("payload");

            let event_type = event_type.parse::<EventType>()?;
            let timestamp = timestamp
                .parse::<chrono::DateTime<chrono::Utc>>()
                .map_err(|e| e.to_string())?;

            let payload = serde_json::from_str::<HashMap<String, String>>(&payload)
                .map_err(|e| e.to_string())?;

            let event = Event {
                event_id,
                object_id,
                event_type,
                timestamp,
                payload,
            };

            events.push(event);
        }
        return Ok(events.into_boxed_slice());
    }
}

fn build_query(count: u32, filter: EventFilter) -> sqlx::QueryBuilder<'static, sqlx::Sqlite> {
    let mut query_builder =
        sqlx::QueryBuilder::<'static, sqlx::Sqlite>::new("SELECT * FROM event ");

    match filter {
        EventFilter::None => (),
        EventFilter::ByType(event_type) => {
            query_builder
                .push("WHERE event_type = ")
                .push_bind(event_type.to_string());
            ()
        }
        EventFilter::ByObject(object_id) => {
            query_builder
                .push("WHERE object_id = ")
                .push_bind(object_id);
            ()
        }
        EventFilter::ById(event_id) => {
            query_builder.push("WHERE id = ").push_bind(event_id);
            ()
        }
    };

    query_builder.push(" LIMIT ").push_bind(count);

    query_builder
}

impl SqliteClient {
    pub async fn new(config: &SqliteClientConfig) -> Result<Self> {
        let pool = SqlitePool::connect(config.path.as_str())
            .await
            .map_err(|e| e.to_string())?;

        let client = Self {
            pool,
            config: config.clone(),
        };

        client.ensure_schema().await?;

        if config.create_testdata {
            client.generate_testdata().await?;
        }

        Ok(client)
    }

    // Generate some test events.
    // This function can be quite handy when you develop your
    // application and need some data for testing.
    async fn generate_testdata(&self) -> Result<()> {
        let query = sqlx::query("DELETE FROM event;");
        self.pool.execute(query).await.map_err(|e| e.to_string())?;

        const INSERT_QUERY: &str= "INSERT INTO event (id, event_type, objectId, timestamp, payload) VALUES (?, ?, ?, ?, ?)";

        for i in 0..1000 {
            let query = sqlx::query(INSERT_QUERY);
            query
                .bind(format!("event-{}", i))
                .bind("metio.bagaluten.io/test-event/v1")
                .bind("testObject")
                .bind(chrono::Utc::now().to_string())
                .bind("{\"test\": \"test\"}")
                .execute(&self.pool)
                .await
                .map_err(|e| e.to_string())?;
        }

        Ok(())
    }

    // ensure all tables are created
    async fn ensure_schema(&self) -> Result<()> {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS event (
            id TEXT PRIMARY KEY,
            event_type TEXT NOT NULL,
            objectId TEXT,
            timestamp INTEGER NOT NULL,
            payload JSON NOT NULL
        );",
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::Result;
    use futures::executor::block_on;

    #[test]
    fn test_sqlite_client() -> Result<()> {
        use super::*;
        let config = SqliteClientConfig {
            path: ":memory:".to_string(),
            create_testdata: true,
        };

        block_on(SqliteClient::new(&config))?;
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
        let client = block_on(SqliteClient::new(&config))?;
        let events = block_on(client.get_events(10))?;
        assert_eq!(events.len(), 10);

        let client = block_on(SqliteClient::new(&config))?;
        let events = block_on(client.get_events(10))?;
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

        let client = block_on(SqliteClient::new(&config))?;
        let events = block_on(client.get_events(10))?;
        assert_eq!(events.len(), 10);

        block_on(client.generate_testdata())?;

        let events = block_on(client.get_events(10))?;
        assert_eq!(events.len(), 10);
        Ok(())
    }

    #[test]
    fn test_get_events_filtered() -> Result<()> {
        use super::*;
        let config = SqliteClientConfig {
            path: ":memory:".to_string(),
            create_testdata: true,
        };

        let client = block_on(SqliteClient::new(&config))?;
        let events = block_on(client.get_events_filtered(
            10,
            EventFilter::ByType("metio.bagaluten.io/test-event/v1".parse()?),
        ))?;

        assert_ne!(events.len(), 0);
        Ok(())
    }
}
