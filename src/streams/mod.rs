use crate::client::Client;


#[derive(Debug, Clone)]
pub struct Stream {
    name: String,
    client: Client,
}

impl Stream {
    pub fn new(name: String, client: Client) -> Self {
        Self { name, client }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}


