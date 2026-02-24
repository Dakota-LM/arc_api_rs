use crate::error::MetaForgeError;
use crate::models::{Bot, PagedResponse};
use crate::MetaForgeClient;
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct BotsQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub id: Option<String>,
    pub search: Option<String>,
    #[serde(rename = "includeLoot")]
    pub include_loot: Option<bool>,
    #[serde(rename = "sortBy")]
    pub sort_by: Option<String>,
    #[serde(rename = "sortOrder")]
    pub sort_order: Option<String>,
}

impl MetaForgeClient {
    /// Returns a paged response (data + pagination + optional extra fields).
    pub async fn bots_paged(&self, q: &BotsQuery) -> Result<PagedResponse<Bot>, MetaForgeError> {
        self.get_json_with_query("arc-raiders/arcs", q).await
    }

    /// Convenience: just the list of bots for a single request/page.
    pub async fn bots(&self, q: &BotsQuery) -> Result<Vec<Bot>, MetaForgeError> {
        Ok(self.bots_paged(q).await?.data)
    }

    /// Fetch a single bot by ID using the endpoint's `id` filter.
    ///
    /// Returns `Ok(None)` if no bot matches.
    pub async fn bot_by_id(&self, id: &str) -> Result<Option<Bot>, MetaForgeError> {
        let q = BotsQuery {
            id: Some(id.to_string()),
            limit: Some(1),
            page: Some(1),
            ..Default::default()
        };

        let mut resp = self.bots_paged(&q).await?.data;
        Ok(resp.pop())
    }

    /// Fetch *all* bots that match the query by auto-paginating.
    ///
    /// - Respects your internal rate limiting + retry behavior automatically.
    /// - If `page` is None, starts at 1.
    /// - If `limit` is None, uses 100 (max per docs).
    pub async fn bots_all(&self, q: &BotsQuery) -> Result<Vec<Bot>, MetaForgeError> {
        let mut q = q.clone();
        if q.page.is_none() {
            q.page = Some(1);
        }
        if q.limit.is_none() {
            q.limit = Some(100);
        }

        let mut out: Vec<Bot> = Vec::new();
        loop {
            let resp = self.bots_paged(&q).await?;

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
