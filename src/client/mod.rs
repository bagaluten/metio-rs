use crate::types::*;
use async_trait::async_trait;

#[cfg(feature = "client_sqlite")]
pub mod sqlite;

pub trait ClientConfig: Clone + Default + std::fmt::Debug {}

pub trait Client {
    type Config: ClientConfig;
    fn get_config(&self) -> Result<Self::Config>;

    // get the last 'count' events
    fn get_events(&self, count: u32) -> Result<Box<[Event]>>;
    fn get_events_filtered(&self, count: u32, filter: EventFilter) -> Result<Box<[Event]>>;
}

#[async_trait]
pub trait AsyncClient: Sync + Sized {
    type Config: ClientConfig;
    async fn get_config(&self) -> Result<Self::Config>;

    // get the last 'count' events
    async fn get_events(&self, count: u32) -> Result<Box<[Event]>>;
    async fn get_events_filtered(&self, count: u32, filter: EventFilter) -> Result<Box<[Event]>>;
}

pub enum EventFilter {
    None,
    ByType(EventType),
    ByObject(String),
    ById(String),
}

pub type Result<T> = std::result::Result<T, String>; //@TODO(kstiehl): Use a real error type
