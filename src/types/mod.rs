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

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The Type information of the event.
///
/// Types in Metio are useful since it allows for a more structured way of defining or parsing
/// events. For some events there event exists a schema registry that can be used to validate.
#[derive(Debug, Clone)]
pub struct EventType {
    /// The group of the event type.
    /// If its not set `core` is assumed.
    pub group: String,

    /// The name of the event type.
    pub name: String,

    /// The version of the event type.
    pub version: String,
}

#[cfg(feature = "serde")]
impl Serialize for EventType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let event_type = self.to_string();
        serializer.serialize_str(&event_type)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for EventType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let parts = s.split('/').collect::<Vec<&str>>();

        if parts.len() != 3 {
            return Err(serde::de::Error::custom(format!(
                "invalid event type: {}",
                s
            )));
        }

        Ok(Self {
            group: parts[0].to_string(),
            name: parts[1].to_string(),
            version: parts[2].to_string(),
        })
    }
}

impl std::str::FromStr for EventType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s.split('/').collect::<Vec<&str>>();

        if parts.len() != 3 {
            return Err(format!("invalid event type: {}", s));
        }

        Ok(Self {
            group: parts[0].to_string(),
            name: parts[1].to_string(),
            version: parts[2].to_string(),
        })
    }
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.group, self.name, self.version)
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Event {
    #[cfg_attr(feature = "serde", serde(rename = "eventId"))]
    pub event_id: String,

    #[cfg_attr(feature = "serde", serde(rename = "objectId"))]
    pub object_id: Option<String>,

    #[cfg_attr(feature = "serde", serde(rename = "eventType"))]
    pub event_type: EventType,

    pub timestamp: chrono::DateTime<chrono::Utc>,

    pub payload: std::collections::HashMap<String, String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_type() {
        let event_type = EventType {
            group: "group".to_string(),
            name: "name".to_string(),
            version: "version".to_string(),
        };

        assert_eq!(event_type.group, "group");
        assert_eq!(event_type.name, "name");
        assert_eq!(event_type.version, "version");
    }

    #[test]
    fn test_event() {
        let event = Event {
            event_id: "event_id".to_string(),
            object_id: Some("object_id".to_string()),
            event_type: EventType {
                group: "group".to_string(),
                name: "name".to_string(),
                version: "version".to_string(),
            },
            timestamp: chrono::Utc::now(),
            payload: std::collections::HashMap::new(),
        };

        assert_eq!(event.event_id, "event_id");
        assert_eq!(event.object_id, Some("object_id".to_string()));
        assert_eq!(event.event_type.group, "group");
        assert_eq!(event.event_type.name, "name");
        assert_eq!(event.event_type.version, "version");
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serialization() {
        let event = Event {
            event_id: "event_id".to_string(),
            object_id: Some("object_id".to_string()),
            event_type: EventType {
                group: "group".to_string(),
                name: "name".to_string(),
                version: "version".to_string(),
            },
            timestamp: chrono::Utc::now(),
            payload: std::collections::HashMap::new(),
        };

        let serialized = serde_json::to_string(&event).unwrap();
        let deserialized: Event = serde_json::from_str(&serialized).unwrap();

        assert_eq!(event.event_id, deserialized.event_id);
        assert_eq!(event.object_id, deserialized.object_id);
        assert_eq!(event.event_type.group, deserialized.event_type.group);
        assert_eq!(event.event_type.name, deserialized.event_type.name);
        assert_eq!(event.event_type.version, deserialized.event_type.version);
    }

    #[test]
    fn test_event_type_from_str() {
        let event_type = "group/name/version".parse::<EventType>().unwrap();

        assert_eq!(event_type.group, "group");
        assert_eq!(event_type.name, "name");
        assert_eq!(event_type.version, "version");
    }

    #[test]
    fn test_event_type_from_str_invalid() {
        let event_type = "group/name".parse::<EventType>();

        assert!(event_type.is_err());
    }
}
