use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Note {
    pub id: u64,
    pub title: String,
    pub data: String,
    pub created_on: NaiveDateTime,
    pub updated_on: NaiveDateTime,
    pub user_id: u64,
}

impl Note {
    pub fn new() -> Self {
        Note {
            id: 0,
            title: "no note found".to_string(),
            data: "no not found".to_string(),
            created_on: NaiveDateTime::MIN,
            updated_on: NaiveDateTime::MIN,
            user_id: 0,
        }
    }
}
