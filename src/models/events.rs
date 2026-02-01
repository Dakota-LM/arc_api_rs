use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EventsScheduleResponse {
    pub data: Vec<ScheduledEvent>,
    pub cached_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScheduledEvent {
    pub name: String,
    pub map: String,
    pub icon: String,

    /// Epoch milliseconds
    pub start_time: i64,

    /// Epoch milliseconds
    pub end_time: i64,
}
