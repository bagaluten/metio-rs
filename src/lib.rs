
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
}
