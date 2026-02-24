use crate::error::MetaForgeError;
use crate::models::{Arc, PagedResponse};
use crate::MetaForgeClient;
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ArcsQuery {
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
    pub async fn arcs_paged(&self, q: &ArcsQuery) -> Result<PagedResponse<Arc>, MetaForgeError> {
        self.get_json_with_query("arc-raiders/arcs", q).await
    }

    /// Convenience: just the list of items for a single request/page.
    pub async fn arcs(&self, q: &ArcsQuery) -> Result<Vec<Arc>, MetaForgeError> {
        Ok(self.arcs_paged(q).await?.data)
    }

    /// Fetch a single item by ID using the endpoint's `id` filter.
    ///
    /// Returns `Ok(None)` if no item matches.
    pub async fn arc_by_id(&self, id: &str) -> Result<Option<Arc>, MetaForgeError> {
        let q = ArcsQuery {
            id: Some(id.to_string()),
            limit: Some(1),
            page: Some(1),
            ..Default::default()
        };

        let mut resp = self.arcs_paged(&q).await?.data;
        Ok(resp.pop())
    }

    /// Fetch *all* items that match the query by auto-paginating.
    ///
    /// - Respects your internal rate limiting + retry behavior automatically.
    /// - If `page` is None, starts at 1.
    /// - If `limit` is None, uses 100 (max per docs).
    pub async fn arcs_all(&self, q: &ArcsQuery) -> Result<Vec<Arc>, MetaForgeError> {
        let mut q = q.clone();
        if q.page.is_none() {
            q.page = Some(1);
        }
        if q.limit.is_none() {
            q.limit = Some(100);
        }

        let mut out: Vec<Arc> = Vec::new();
        loop {
            let resp = self.arcs_paged(&q).await?;

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
