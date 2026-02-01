use crate::error::MetaForgeError;
use crate::models::{Item, PagedResponse};
use crate::MetaForgeClient;
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct ItemsQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,

    pub id: Option<String>,
    pub item_type: Option<String>,
    pub rarity: Option<String>,
    pub search: Option<String>,
    pub loadout_slot: Option<String>,
    pub workbench: Option<String>,
    pub subcategory: Option<String>,
    pub shield_type: Option<String>,

    #[serde(rename = "includeComponents")]
    pub include_components: Option<bool>,

    #[serde(rename = "sortBy")]
    pub sort_by: Option<String>,

    #[serde(rename = "sortOrder")]
    pub sort_order: Option<String>,

    pub minimal: Option<bool>,
}

impl MetaForgeClient {
    /// Returns a paged response (data + pagination + optional extra fields).
    pub async fn items_paged(&self, q: &ItemsQuery) -> Result<PagedResponse<Item>, MetaForgeError> {
        self.get_json_with_query("arc-raiders/items", q).await
    }

    /// Convenience: just the list of items for a single request/page.
    pub async fn items(&self, q: &ItemsQuery) -> Result<Vec<Item>, MetaForgeError> {
        Ok(self.items_paged(q).await?.data)
    }

    /// Fetch a single item by ID using the endpoint's `id` filter.
    ///
    /// Returns `Ok(None)` if no item matches.
    pub async fn item_by_id(&self, id: &str) -> Result<Option<Item>, MetaForgeError> {
        let q = ItemsQuery {
            id: Some(id.to_string()),
            limit: Some(1),
            page: Some(1),
            ..Default::default()
        };

        let mut resp = self.items_paged(&q).await?.data;
        Ok(resp.pop())
    }

    /// Fetch *all* items that match the query by auto-paginating.
    ///
    /// - Respects your internal rate limiting + retry behavior automatically.
    /// - If `page` is None, starts at 1.
    /// - If `limit` is None, uses 100 (max per docs).
    pub async fn items_all(&self, q: &ItemsQuery) -> Result<Vec<Item>, MetaForgeError> {
        let mut q = q.clone();
        if q.page.is_none() {
            q.page = Some(1);
        }
        if q.limit.is_none() {
            q.limit = Some(100);
        }

        let mut out: Vec<Item> = Vec::new();

        loop {
            let resp = self.items_paged(&q).await?;

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
