pub enum Kind {
    Config,
    Connect,
}

pub struct Error {
    pub kind: Kind,
    pub message: String,
}

impl Error {
    pub fn new(kind: Kind, message: String) -> Self {
        Self { kind, message }
    }
}
