use crate::error::MetaForgeError;
use crate::models::EventsScheduleResponse;
use crate::MetaForgeClient;

impl MetaForgeClient {
    /// Fetch the current events schedule.
    pub async fn events_schedule(&self) -> Result<EventsScheduleResponse, MetaForgeError> {
        self.get_json("arc-raiders/events-schedule").await
    }
}
