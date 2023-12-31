#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventType {
    pub group: String,
    pub name: String,
    pub version: String,
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
    pub object_id: String,

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
            object_id: "object_id".to_string(),
            event_type: EventType {
                group: "group".to_string(),
                name: "name".to_string(),
                version: "version".to_string(),
            },
            timestamp: chrono::Utc::now(),
            payload: std::collections::HashMap::new(),
        };

        assert_eq!(event.event_id, "event_id");
        assert_eq!(event.object_id, "object_id");
        assert_eq!(event.event_type.group, "group");
        assert_eq!(event.event_type.name, "name");
        assert_eq!(event.event_type.version, "version");
    }

    #[test]
    #[cfg(feature = "serde")]
    fn test_serialization() {
        let event = Event {
            event_id: "event_id".to_string(),
            object_id: "object_id".to_string(),
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
