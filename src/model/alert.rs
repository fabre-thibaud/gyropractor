use chrono::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct Alert {
    pub id: i32,
    pub name: String,
    pub component: String,
    pub created_at: DateTime<Utc>,
    pub checked: bool,
    pub checked_at: Option<DateTime<Utc>>,
}

#[derive(Deserialize, Debug)]
pub struct AlertRequest {
    pub name: String,
    pub component: String,
}

#[derive(Deserialize, Debug)]
pub struct AlertUpdateRequest {
    pub name: String,
    pub component: String,
    pub checked: bool,
}

#[derive(Serialize)]
pub struct AlertResponse {
    pub id: i32,
    pub name: String,
    pub component: String,
    pub checked: bool,
    pub created_at: DateTime<Utc>,
    pub checked_at: Option<DateTime<Utc>>,
}

impl AlertResponse {
    pub fn of(alert: Alert) -> AlertResponse {
        AlertResponse {
            id: alert.id,
            name: alert.name,
            component: alert.component,
            checked: alert.checked,
            created_at: alert.created_at,
            checked_at: alert.checked_at,
        }
    }
}
