use std::sync::Arc;
use std::time::Duration;

use tokio::sync::Mutex;
use tokio::time::Instant;

/// Enforces a minimum interval between requests.
///
/// All clones share the same throttle state, so a single `RateLimiter`
/// can be used across the client and its background logout task.
#[derive(Clone)]
pub(crate) struct RateLimiter {
    interval: Duration,
    last_request: Arc<Mutex<Instant>>,
}

impl RateLimiter {
    /// Create a limiter that allows at most `max_per_sec` requests per second.
    pub fn new(max_per_sec: u32) -> Self {
        Self {
            interval: Duration::from_secs_f64(1.0 / max_per_sec as f64),
            last_request: Arc::new(Mutex::new(Instant::now())),
        }
    }

    /// Wait until the minimum interval has elapsed since the last request.
    pub async fn wait(&self) {
        let mut last = self.last_request.lock().await;
        let elapsed = last.elapsed();
        if elapsed < self.interval {
            tokio::time::sleep(self.interval - elapsed).await;
        }
        *last = Instant::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn wait_enforces_interval() {
        let limiter = RateLimiter::new(10); // 100ms interval
        let start = Instant::now();

        limiter.wait().await;
        limiter.wait().await;

        assert!(start.elapsed() >= Duration::from_millis(100));
    }

    #[tokio::test]
    async fn wait_no_delay_when_interval_elapsed() {
        let limiter = RateLimiter::new(10); // 100ms interval

        limiter.wait().await;
        tokio::time::sleep(Duration::from_millis(150)).await;

        let before = Instant::now();
        limiter.wait().await;

        // Should return nearly immediately (well under the 100ms interval)
        assert!(before.elapsed() < Duration::from_millis(50));
    }

    #[tokio::test]
    async fn clones_share_state() {
        let limiter = RateLimiter::new(10); // 100ms interval
        let clone = limiter.clone();

        let start = Instant::now();
        limiter.wait().await;
        clone.wait().await;

        assert!(start.elapsed() >= Duration::from_millis(100));
    }
}
