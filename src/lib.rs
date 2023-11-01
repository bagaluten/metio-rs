
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
    pub event_id: String,
    pub object_id: String,
    pub event_type: EventType,
}
