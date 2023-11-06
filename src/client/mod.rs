#[cfg(feature = "client_sqlite")]
pub mod sqlite;

pub trait ClientConfig: Clone + Default + std::fmt::Debug {
    fn new() -> Self;
}

pub trait Client: Sized {
    type Config: ClientConfig;
    fn new(config: &Self::Config) -> Result<Self>;
    fn get_config(&self) -> Result<Self::Config>;
}

pub type Result<T> = std::result::Result<T, String>; //@TODO(kstiehl): Use a real error type
