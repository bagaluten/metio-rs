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

#[derive(Clone, Debug)]
pub enum Kind {
    /// This indicates that something with the given client configuration is not correct.
    /// This could be a missing field or a wrong value.
    Config,
    /// This indicates that the client could not connect to the server.
    Connect,
    /// This indicates that the client could not send the message.
    /// If this error occurs the messages that were not able to be sent are provided
    /// in the `related_messages` field.
    Send,
}

impl std::fmt::Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Config => write!(f, "Config"),
            Kind::Connect => write!(f, "Connect"),
            Kind::Send => write!(f, "Send"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Error {
    kind: Kind,
    message: String,
    related_messages: Option<Vec<String>>,
}

impl Error {
    pub fn new(kind: Kind, message: String) -> Self {
        Self {
            kind,
            message,
            related_messages: None,
        }
    }

    pub fn new_with_related(kind: Kind, message: String, related_message: Vec<String>) -> Self {
        Self {
            kind,
            message,
            related_messages: Some(related_message),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(related) = &self.related_messages {
            write!(f, "{}: {}\nRelated: {:?}", self.kind, self.message, related)
        } else {
            write!(f, "{}: {}", self.kind, self.message)
        }
    }
}
