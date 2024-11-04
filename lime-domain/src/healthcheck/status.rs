use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    _type: StatusType
}

impl Status {
    pub fn new(_type: StatusType) -> Self {
        Self { _type }
    }

    pub fn status_type(self) -> StatusType {
        self._type
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum StatusType {
    Ok,
    Err,
    Paused
}
impl ToString for StatusType {
    fn to_string(&self) -> String {
        match self {
            StatusType::Ok => "ok",
            StatusType::Err => "error",
            StatusType::Paused => "paused"
        }.to_string()
    }
}