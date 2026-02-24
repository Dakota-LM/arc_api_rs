use crate::error::MetaForgeError;
use crate::models::traders::TradersResponse;
use crate::MetaForgeClient;

impl MetaForgeClient {
    /// Fetch all trader inventories.
    pub async fn traders(&self) -> Result<TradersResponse, MetaForgeError> {
        self.get_json("arc-raiders/traders").await
    }
}
