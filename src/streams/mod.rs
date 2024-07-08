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

use crate::client::error::Error;
use crate::client::Client;
use crate::types::Event;

/// A stream is a Metio Stream. It should not be confused with
/// streams in that are used elsewhere in Rust. This is not something
/// you can call `next` on or iterate over.
#[derive(Debug, Clone)]
pub struct Stream {
    name: String,
    client: Client,
}

impl Stream {
    pub fn new(name: String, client: Client) -> Self {
        Self { name, client }
    }

    // Get the name of the stream that this object is connected to.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Push a vector of events to the stream.
    /// Every element will be pushed as a single message to the stream.
    pub async fn publish<I>(&self, events: I) -> Result<(), Error>
    where
        I: IntoIterator<Item = Event>,
    {
        let subject = self.name.clone();
        self.client.publish(subject, events).await
    }
}
