use reqwest::header::InvalidHeaderValue;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DelError {
    pub error: bool,
    pub status: Option<u16>,
    pub message: String,
}

impl From<reqwest::Error> for DelError {
    fn from(err: reqwest::Error) -> Self {
        Self {
            error: true,
            status: err.status().map(|s| s.as_u16()),
            message: err.to_string(),
        }
    }
}

impl From<InvalidHeaderValue> for DelError {
    fn from(err: InvalidHeaderValue) -> Self {
        Self {
            error: true,
            status: None,
            message: err.to_string(),
        }
    }
}
