pub struct EventType {
    pub package: String,
    pub group: String,
    pub name: String,
    pub version: String,
}

pub struct Event {
    pub event_id: String,
    pub object_id: String,
    pub event_type: EventType,
}
