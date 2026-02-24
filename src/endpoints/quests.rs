use crate::MetaForgeClient;
use crate::error::MetaForgeError;
use crate::models::{PagedResponse, Quest};
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct QuestQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub id: Option<String>,
    pub search: Option<String>,
    #[serde(rename = "sortBy")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    pub sort_order: Option<String>,
}

impl MetaForgeClient {
    /// Returns a paged response (data + pagination + optional extra fields).
    pub async fn quests_paged(
        &self,
        q: &QuestQuery,
    ) -> Result<PagedResponse<Quest>, MetaForgeError> {
        self.get_json_with_query("arc-raiders/quests", q).await
    }

    /// Convenience: just the list of items for a single request/page.
    pub async fn quests(&self, q: &QuestQuery) -> Result<Vec<Quest>, MetaForgeError> {
        Ok(self.quests_paged(q).await?.data)
    }

    /// Fetch a single item by ID using the endpoint's `id` filter.
    ///
    /// Returns `Ok(None)` if no item matches.
    pub async fn quest_by_id(&self, id: &str) -> Result<Option<Quest>, MetaForgeError> {
        let q = QuestQuery {
            id: Some(id.to_string()),
            limit: Some(1),
            page: Some(1),
            ..Default::default()
        };

        let mut resp = self.quests_paged(&q).await?.data;
        Ok(resp.pop())
    }

    /// Fetch *all* items that match the query by auto-paginating.
    ///
    /// - Respects your internal rate limiting + retry behavior automatically.
    /// - If `page` is None, starts at 1.
    /// - If `limit` is None, uses 100 (max per docs).
    pub async fn quests_all(&self, q: &QuestQuery) -> Result<Vec<Quest>, MetaForgeError> {
        let mut q = q.clone();
        if q.page.is_none() {
            q.page = Some(1);
        }
        if q.limit.is_none() {
            q.limit = Some(100);
        }

        let mut out: Vec<Quest> = Vec::new();
        loop {
            let resp = self.quests_paged(&q).await?;

            out.extend(resp.data);

            if !resp.pagination.has_next_page {
                break;
            }

            // Next page
            q.page = Some(q.page.unwrap().saturating_add(1));
        }

        Ok(out)
    }
}
