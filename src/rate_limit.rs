use crate::config::RateConfig;

use governor::{DefaultDirectRateLimiter, Quota, RateLimiter};
use std::sync::Arc;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

/// Wraps rate limiting (governor) + concurrency limiting (semaphore).
#[derive(Clone)]
pub struct RateGate {
    limiter: Arc<DefaultDirectRateLimiter>,
    inflight: Arc<Semaphore>,
}

impl RateGate {
    pub fn new(cfg: RateConfig) -> Self {
        // governor wants a Quota. We set the sustained rate + burst.
        let quota = Quota::per_second(cfg.rps).allow_burst(cfg.burst);

        let limiter = Arc::new(RateLimiter::direct(quota));
        let inflight = Arc::new(Semaphore::new(cfg.max_inflight.get() as usize));

        Self { limiter, inflight }
    }

    /// Wait until the rate limiter allows the next request.
    ///
    /// Call this *right before* sending a request attempt.
    pub async fn wait_turn(&self) {
        self.limiter.until_ready().await;
    }

    /// Acquire a concurrency permit (caps concurrent in-flight requests).
    ///
    /// Hold onto the returned permit for the entire duration of the HTTP attempt.
    /// When dropped, the slot is released.
    pub async fn acquire_inflight(&self) -> OwnedSemaphorePermit {
        // acquire_owned gives you a permit that's not tied to a borrowed semaphore reference,
        // which plays nicer with clones and struct fields.
        self.inflight
            .clone()
            .acquire_owned()
            .await
            .expect("semaphore closed")
    }
}
