/*
 * Copyright 2024 Bagaluten GmbH <contact@bagaluten.email>
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *   http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

pub mod error;

use async_nats as nats;

#[derive(Clone, Default, Debug)]
pub struct Config {
    pub host: String,
    pub port: u16,
}

/// The Client struct holds the information to which metio cluster we are currently talking.
#[derive(Debug, Clone)]
pub struct Client {
    client: nats::Client,
}

impl Client {
    pub fn new(client: nats::Client) -> Self {
        Self { client }
    }

    /// This function returns the underlying NATS client.
    pub fn get_underlying(&self) -> nats::Client {
        self.client.clone()
    }
        
}

/// Connect to a Metio Server. 
/// If the connection was successfull a client will be returned.
///
/// # Example
/// ```no_run
/// use metio::client::{connect, Config, error::Error};
/// # async fn example() -> Result<(), Error> {
/// let cfg = Config::default();
/// let client = connect(cfg).await?;
/// // Do something with the client
/// # Ok(())
/// # }
/// ```
pub async fn connect<C>(cfg: C) -> Result<Client, error::Error>
where C: Into<Config>{
    let cfg = cfg.into();

    let client = nats::connect(&cfg.host).await.map_err(|e| error::Error::new(error::Kind::Connect, e.to_string()))?;

    tracing::info!("Connecting to server with config: {:?}", cfg);

   Ok(Client::new(client))
}

