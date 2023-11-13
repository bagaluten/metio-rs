use crate::types::*;

#[cfg(feature = "client_sqlite")]
pub mod sqlite;

pub trait ClientConfig: Clone + Default + std::fmt::Debug {}

pub trait Client: Sized {
    type Config: ClientConfig;
    fn new(config: &Self::Config) -> Result<Self>;
    fn get_config(&self) -> Result<Self::Config>;

    // get the last 'count' events
    fn get_events(&self, count: u32) -> Result<Box<[Event]>>;
}

pub type Result<T> = std::result::Result<T, String>; //@TODO(kstiehl): Use a real error type
