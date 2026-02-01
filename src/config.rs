use std::{num::NonZeroU32, time::Duration};

/// Configuration for the MetaForge client.
///
/// Defaults are conservative to avoid hammering the API.
#[derive(Debug, Clone)]
pub struct MetaForgeConfig {
    /// Base URL for the ARC Raiders API.
    ///
    /// Default should match what the MetaForge docs show.
    pub base_url: String,

    /// Rate limiting / concurrency settings.
    pub rate: RateConfig,

    /// Retry policy for transient failures.
    pub retries: RetryConfig,
}

impl Default for MetaForgeConfig {
    fn default() -> Self {
        Self {
            // MetaForge docs page is /arc-raiders/api; the API base used in examples is:
            // https://metaforge.app/api/arc-raiders
            //
            // If you confirm a different base (or there’s versioning), this is the only line to change.
            base_url: "https://metaforge.app/api".to_string(),

            // Polite defaults: 5 req/sec, burst 10, max 8 in-flight
            rate: RateConfig::polite_defaults(),

            // Retry up to 5 total attempts with exponential backoff
            retries: RetryConfig::default(),
        }
    }
}

/// Rate limiting and concurrency configuration.
#[derive(Debug, Clone)]
pub struct RateConfig {
    /// Requests per second allowed.
    pub rps: NonZeroU32,

    /// Allowed burst size (tokens that can accumulate).
    pub burst: NonZeroU32,

    /// Maximum concurrent in-flight requests.
    pub max_inflight: NonZeroU32,
}

impl RateConfig {
    /// Conservative defaults that should play nice with a public API.
    pub fn polite_defaults() -> Self {
        Self {
            rps: NonZeroU32::new(5).expect("nonzero"),
            burst: NonZeroU32::new(10).expect("nonzero"),
            max_inflight: NonZeroU32::new(8).expect("nonzero"),
        }
    }

    /// Very conservative defaults for apps that poll frequently (UI refresh loops, bots, etc).
    pub fn very_polite() -> Self {
        Self {
            rps: NonZeroU32::new(2).expect("nonzero"),
            burst: NonZeroU32::new(4).expect("nonzero"),
            max_inflight: NonZeroU32::new(4).expect("nonzero"),
        }
    }
}

/// Retry policy configuration.
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of attempts (including the first try).
    pub max_attempts: u32,

    /// Base backoff duration for attempt 1->2.
    pub base_backoff: Duration,

    /// Max backoff cap.
    pub max_backoff: Duration,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 5,
            base_backoff: Duration::from_millis(200),
            max_backoff: Duration::from_secs(5),
        }
    }
}
