pub mod endpoints;
pub mod models;

mod client;
mod config;
mod error;
mod rate_limit;

// Core client + config
pub use client::MetaForgeClient;
pub use config::MetaForgeConfig;
pub use error::MetaForgeError;

// Convenience re-export so examples can do:
// use arc_api_rs::ItemsQuery;
pub use endpoints::items::ItemsQuery;
pub use endpoints::arcs::ArcsQuery;
// pub use endpoints::quests::QuestsQuery;
// pub use endpoints::traders::TradersQuery;
pub use endpoints::game_map_data::MapDataQuery;
pub use models::Item;
