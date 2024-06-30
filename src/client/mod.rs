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

use tracing::Instrument;

use self::error::Error;
use crate::types::Event;
use async_nats as nats;

#[derive(Clone, Default, Debug)]
pub struct Config {
    pub host: String,
    pub prefix: Option<String>,
}

/// The Client struct holds the information to which metio cluster we are currently talking.
#[derive(Debug, Clone)]
pub struct Client {
    client: nats::Client,
    prefix: Option<String>,
}

impl Client {
    /// This function returns the underlying NATS client.
    pub fn get_underlying(&self) -> nats::Client {
        self.client.clone()
    }

    /// Publish a list of events to a subject.
    pub async fn publish(&self, subject: String, data: Vec<Event>) -> Result<(), Error> {
        let subject = match &self.prefix {
            Some(prefix) => format!("{}.{}", prefix, subject),
            None => subject,
        };
        let mut failed_events: Vec<(Event, String)> = Vec::new();
        for event in data {
            let res: Result<(), String> = async {
                let bytes = serde_json::to_vec(&event).map_err(|e| e.to_string())?;
                self.internal_publish(&subject, bytes).await?;
                Ok(())
            }
            .instrument(tracing::trace_span!("publish", event_id = event.event_id))
            .await;

            if let Err(e) = res {
                failed_events.push((event, e));
            }
        }

        Ok(())
    }

    #[tracing::instrument(level = "debug", skip(self))]
    async fn internal_publish(&self, subject: &String, data: Vec<u8>) -> Result<(), String> {
        self.client
            .publish(subject.clone(), data.into())
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
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
where
    C: Into<Config>,
{
    let cfg = cfg.into();

    let client = nats::connect(&cfg.host)
        .await
        .map_err(|e| error::Error::new(error::Kind::Connect, e.to_string()))?;

    tracing::info!("Connecting to server with config: {:?}", cfg);

    Ok(Client {
        client,
        prefix: cfg.prefix,
    })
}
