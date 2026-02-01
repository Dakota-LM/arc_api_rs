use crate::config::MetaForgeConfig;
use crate::error::MetaForgeError;
use crate::rate_limit::RateGate;

use reqwest::{Method, StatusCode};
use std::time::Duration;

/// Main client type users interact with.
///
/// Public endpoint methods live in `src/endpoints/*` as `impl MetaForgeClient { ... }`.
#[derive(Clone)]
pub struct MetaForgeClient {
    http: reqwest::Client,
    config: MetaForgeConfig,
    gate: RateGate,
}

impl MetaForgeClient {
    /// Convenience constructor (good for quick scripts).
    ///
    /// For libraries/services, prefer `with_client(...)` so the app controls transport config.
    pub fn new() -> Self {
        let http = reqwest::Client::builder()
            .user_agent("arc-api-rs/0.1 (+https://github.com/Dakota-LM/arc-rs)")
            .timeout(Duration::from_secs(15))
            .build()
            .expect("failed to build reqwest client");

        Self::with_client(http)
    }

    /// Inject a preconfigured reqwest client (recommended).
    pub fn with_client(http: reqwest::Client) -> Self {
        Self::with_client_and_config(http, MetaForgeConfig::default())
    }

    /// Inject a client and fully control configuration (rate limits, base url, etc).
    pub fn with_client_and_config(http: reqwest::Client, config: MetaForgeConfig) -> Self {
        let gate = RateGate::new(config.rate.clone());
        Self { http, config, gate }
    }

    /// Useful for tests (wiremock) or alternate deployments.
    pub fn with_client_and_base(http: reqwest::Client, base_url: impl Into<String>) -> Self {
        let mut cfg = MetaForgeConfig::default();
        cfg.base_url = base_url.into();
        Self::with_client_and_config(http, cfg)
    }

    /// Expose base url for debugging / introspection.
    pub fn base_url(&self) -> &str {
        &self.config.base_url
    }

    // -------------------------------------------------------------------------
    // Internal request pipeline (THE choke point)
    // -------------------------------------------------------------------------

    /// GET + JSON response (no query).
    #[allow(dead_code)]
    pub(crate) async fn get_json<T>(&self, path: &str) -> Result<T, MetaForgeError>
    where
        T: serde::de::DeserializeOwned,
    {
        self.request_json::<T, ()>(Method::GET, path, None, None::<&()>).await
    }

    /// GET + JSON response with query params (`serde::Serialize`).
    pub(crate) async fn get_json_with_query<T, Q>(
        &self,
        path: &str,
        query: &Q,
    ) -> Result<T, MetaForgeError>
    where
        T: serde::de::DeserializeOwned,
        Q: serde::Serialize + ?Sized + std::fmt::Debug,
    {
        println!("GET with base url: '{}', and query: {:?}", path, query);
        self.request_json::<T, Q>(Method::GET, path, Some(query), None::<&()>).await
    }

    /// Generic request that expects JSON response.
    ///
    /// - Handles rate limiting + concurrency gating
    /// - Retries transient failures
    /// - Honors 429 Retry-After where possible
    pub(crate) async fn request_json<T, Q>(
        &self,
        method: Method,
        path: &str,
        query: Option<&Q>,
        body_json: Option<&impl serde::Serialize>,
    ) -> Result<T, MetaForgeError>
    where
        T: serde::de::DeserializeOwned,
        Q: serde::Serialize + ?Sized,
    {
        let url = self.join_url(path)?;

        // Concurrency + rate limiter gates (every attempt must pass through)
        // so retries don't bypass your politeness.
        let mut attempt: u32 = 0;

        loop {
            attempt += 1;

            let _permit = self.gate.acquire_inflight().await;
            self.gate.wait_turn().await;

            let mut req = self.http.request(method.clone(), &url);

            if let Some(q) = query {
                req = req.query(q);
            }
            if let Some(b) = body_json {
                req = req.json(b);
            }

            // Log the full request URL with query parameters
            let built_req = req.build().map_err(|e| MetaForgeError::Transport(e))?;
            println!("Full request URL: {}", built_req.url());
            
            // Rebuild the request since we consumed it with build()
            let mut req = self.http.request(method.clone(), &url);
            if let Some(q) = query {
                req = req.query(q);
            }
            if let Some(b) = body_json {
                req = req.json(b);
            }

            let resp = match req.send().await {
                Ok(r) => r,
                Err(e) => {
                    // Network-y errors: retry a few times.
                    if attempt >= self.config.retries.max_attempts {
                        return Err(MetaForgeError::Transport(e));
                    }

                    tokio::time::sleep(backoff_duration(
                        attempt,
                        self.config.retries.base_backoff,
                        self.config.retries.max_backoff,
                    ))
                    .await;
                    continue;
                }
            };

            // 429: rate-limited by server. Respect Retry-After if present.
            if resp.status() == StatusCode::TOO_MANY_REQUESTS {
                let wait = retry_after_duration(&resp).unwrap_or_else(|| {
                    backoff_duration(
                        attempt,
                        self.config.retries.base_backoff,
                        self.config.retries.max_backoff,
                    )
                });

                if attempt >= self.config.retries.max_attempts {
                    return Err(MetaForgeError::RateLimited { wait });
                }

                tokio::time::sleep(wait).await;
                continue;
            }

            // Retry 5xx a few times (server transient).
            if resp.status().is_server_error() {
                // Try to read the response body for debugging
                let status = resp.status();
                let body = resp.text().await.unwrap_or_else(|_| "(unable to read body)".to_string());
                
                // Check if this is a "not allowed" error - these should fail immediately
                let is_not_allowed = if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                    println!("Server error {} - Full JSON: {}", status, serde_json::to_string_pretty(&json).unwrap_or_else(|_| format!("{:?}", json)));
                    
                    // Check if error field is an object with nested details
                    if let Some(error) = json.get("error") {
                        println!("  Error: {:?}", error);
                        if let Some(error_obj) = error.as_object() {
                            if let Some(inner_msg) = error_obj.get("message") {
                                println!("    Inner Message: {:?}", inner_msg);
                            }
                            if let Some(inner_details) = error_obj.get("details") {
                                println!("    Inner Details: {:?}", inner_details);
                            }
                        }
                    }
                    if let Some(msg) = json.get("message") {
                        println!("  Message: {:?}", msg);
                    }
                    if let Some(details) = json.get("details") {
                        println!("  Details: {:?}", details);
                    }
                    
                    // Check if error contains "not allowed"
                    json.get("error")
                        .and_then(|e| e.as_str())
                        .map(|s| s == "not allowed")
                        .unwrap_or(false)
                } else {
                    println!("Server error {} - Response body: {}", status, body);
                    false
                };
                
                // Don't retry "not allowed" errors
                if is_not_allowed {
                    println!("Not retrying 'not allowed' error");
                    return Err(MetaForgeError::HttpStatus(status));
                }
                
                if attempt >= self.config.retries.max_attempts {
                    return Err(MetaForgeError::HttpStatus(status));
                }

                tokio::time::sleep(backoff_duration(
                    attempt,
                    self.config.retries.base_backoff,
                    self.config.retries.max_backoff,
                ))
                .await;
                continue;
            }

            // Any other non-success is a hard error (4xx etc.)
            if !resp.status().is_success() {
                // Try to read the response body for debugging
                let status = resp.status();
                let body = resp.text().await.unwrap_or_else(|_| "(unable to read body)".to_string());
                
                // Try to parse JSON error details and look for nested error information
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                    println!("HTTP error {} - Full JSON: {}", status, serde_json::to_string_pretty(&json).unwrap_or_else(|_| format!("{:?}", json)));
                    
                    // Check if error field is an object with nested details
                    if let Some(error) = json.get("error") {
                        println!("  Error: {:?}", error);
                        if let Some(error_obj) = error.as_object() {
                            if let Some(inner_msg) = error_obj.get("message") {
                                println!("    Inner Message: {:?}", inner_msg);
                            }
                            if let Some(inner_details) = error_obj.get("details") {
                                println!("    Inner Details: {:?}", inner_details);
                            }
                        }
                    }
                    if let Some(msg) = json.get("message") {
                        println!("  Message: {:?}", msg);
                    }
                    if let Some(details) = json.get("details") {
                        println!("  Details: {:?}", details);
                    }
                } else {
                    println!("HTTP error {} - Response body: {}", status, body);
                }
                
                return Err(MetaForgeError::HttpStatus(status));
            }

            // Success: decode JSON
            return Ok(resp.json::<T>().await?);
        }
    }

    fn join_url(&self, path: &str) -> Result<String, MetaForgeError> {
        let base = self.config.base_url.trim_end_matches('/');
        let p = path.trim_start_matches('/');
        println!("Url: {base}/{p}");
        Ok(format!("{base}/{p}"))
    }
}

// -----------------------------------------------------------------------------
// Helpers
// -----------------------------------------------------------------------------

fn backoff_duration(attempt: u32, base: Duration, max: Duration) -> Duration {
    // base * 2^(attempt-1), capped at max
    let pow = attempt.saturating_sub(1).min(16); // prevent absurd overflow
    let mult = 1u32 << pow;

    let ms = base
        .as_millis()
        .saturating_mul(mult as u128)
        .min(max.as_millis());

    Duration::from_millis(ms as u64)
}

fn retry_after_duration(resp: &reqwest::Response) -> Option<Duration> {
    let h = resp.headers().get(reqwest::header::RETRY_AFTER)?.to_str().ok()?;

    // Most common: seconds
    if let Ok(seconds) = h.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }

    // HTTP-date support is optional; can be added later if needed.
    None
}
