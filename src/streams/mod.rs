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

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn publish(&mut self, data: Vec<Event>) {
        let subject = self.name.clone();
        let _ = self.client.publish(subject, data);
    }
}
