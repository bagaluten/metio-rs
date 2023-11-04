#[cfg(feature = "client_sqlite")]
pub mod sqlite;

pub trait ClientConfig: Clone + Default + std::fmt::Debug {
    fn new() -> Self;
}

pub trait Client {
    type Config: ClientConfig;
    fn new(config: &Self::Config) -> Self;
    fn get_config(&self) -> &Self::Config;
}

pub type Result<T> = std::result::Result<T, String>; //@TODO(kstiehl): Use a real error type
