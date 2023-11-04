#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventType {
    pub package: String,
    pub group: String,
    pub name: String,
    pub version: String,
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_type() {
        let event_type = EventType {
            package: "package".to_string(),
            group: "group".to_string(),
            name: "name".to_string(),
            version: "version".to_string(),
        };

        assert_eq!(event_type.package, "package");
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
                package: "package".to_string(),
                group: "group".to_string(),
                name: "name".to_string(),
                version: "version".to_string(),
            },
            timestamp: chrono::Utc::now(),
        };

        assert_eq!(event.event_id, "event_id");
        assert_eq!(event.object_id, "object_id");
        assert_eq!(event.event_type.package, "package");
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
                package: "package".to_string(),
                group: "group".to_string(),
                name: "name".to_string(),
                version: "version".to_string(),
            },
            timestamp: chrono::Utc::now(),
        };

        let serialized = serde_json::to_string(&event).unwrap();
        println!("{}", serialized);
        let deserialized: Event = serde_json::from_str(&serialized).unwrap();

        assert_eq!(event.event_id, deserialized.event_id);
        assert_eq!(event.object_id, deserialized.object_id);
        assert_eq!(event.event_type.package, deserialized.event_type.package);
        assert_eq!(event.event_type.group, deserialized.event_type.group);
        assert_eq!(event.event_type.name, deserialized.event_type.name);
        assert_eq!(event.event_type.version, deserialized.event_type.version);
    }
}
