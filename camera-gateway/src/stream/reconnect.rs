use std::cmp;
use std::time::Duration;

/// Configuration for exponential backoff reconnection.
#[derive(Debug, Clone)]
pub struct ReconnectPolicy {
    /// Initial delay before the first retry.
    pub initial_delay: Duration,
    /// Maximum delay between retries.
    pub max_delay: Duration,
    /// Maximum number of retry attempts. `None` means unlimited.
    pub max_retries: Option<u32>,
}

impl Default for ReconnectPolicy {
    fn default() -> Self {
        Self {
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(60),
            max_retries: Some(10),
        }
    }
}

/// Tracks the state of an exponential backoff reconnection sequence.
#[derive(Debug)]
pub struct BackoffState {
    policy: ReconnectPolicy,
    attempt: u32,
    total_attempts: u32,
}

impl BackoffState {
    pub fn new(policy: ReconnectPolicy) -> Self {
        Self {
            policy,
            attempt: 0,
            total_attempts: 0,
        }
    }

    /// Calculate the delay for the next retry attempt using exponential backoff.
    ///
    /// Formula: `min(initial_delay * 2^attempt, max_delay)`
    /// A small jitter of ±10% is applied to prevent thundering herd.
    pub fn next_delay(&mut self) -> Option<Duration> {
        if let Some(max) = self.policy.max_retries {
            if self.attempt >= max {
                return None;
            }
        }

        let base = self.policy.initial_delay.as_millis() as u64;
        let power = self.attempt.min(20); // Prevent overflow
        let doubled = base.saturating_mul(1u64 << power);
        let capped = cmp::min(doubled, self.policy.max_delay.as_millis() as u64);

        // Apply ±10% jitter
        let jitter_range = cmp::max(capped / 10, 1);
        let jitter = (self.total_attempts as u64 * 7 + self.attempt as u64 * 13) % jitter_range;
        let delay_ms = capped
            .saturating_sub(jitter_range / 2)
            .saturating_add(jitter);

        self.attempt += 1;
        self.total_attempts += 1;

        Some(Duration::from_millis(delay_ms))
    }

    /// Record a successful connection — reset the backoff state.
    pub fn reset(&mut self) {
        self.attempt = 0;
    }

    /// Get the current attempt number (0-indexed).
    pub fn attempt(&self) -> u32 {
        self.attempt
    }

    /// Get the total number of attempts made since the last reset.
    pub fn total_attempts(&self) -> u32 {
        self.total_attempts
    }

    /// Check if retries are exhausted.
    pub fn is_exhausted(&self) -> bool {
        if let Some(max) = self.policy.max_retries {
            self.attempt >= max
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_policy() {
        let policy = ReconnectPolicy::default();
        assert_eq!(policy.initial_delay, Duration::from_secs(1));
        assert_eq!(policy.max_delay, Duration::from_secs(60));
        assert_eq!(policy.max_retries, Some(10));
    }

    #[test]
    fn test_first_delay() {
        let mut state = BackoffState::new(ReconnectPolicy::default());
        let delay = state.next_delay().unwrap();
        // First delay should be around initial_delay (1s) with jitter
        assert!(delay >= Duration::from_millis(0));
        assert!(delay <= Duration::from_secs(2));
    }

    #[test]
    fn test_exponential_growth() {
        let policy = ReconnectPolicy {
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            max_retries: Some(20),
        };
        let mut state = BackoffState::new(policy);

        let delays: Vec<Duration> = (0..8).map(|_| state.next_delay().unwrap()).collect();

        // Verify general upward trend (ignoring jitter)
        for window in delays.windows(2) {
            // Each delay should be roughly double the previous
            let ratio = window[1].as_millis() as f64 / cmp::max(window[0].as_millis(), 1) as f64;
            // With jitter, ratio can vary, but should generally be > 0.5
            assert!(ratio > 0.5, "ratio was {ratio}");
        }
    }

    #[test]
    fn test_max_delay_cap() {
        let policy = ReconnectPolicy {
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(1),
            max_retries: Some(20),
        };
        let mut state = BackoffState::new(policy);

        // Run many attempts to ensure we hit the cap
        for _ in 0..20 {
            if let Some(delay) = state.next_delay() {
                assert!(
                    delay <= Duration::from_secs(2),
                    "delay {delay:?} exceeded max"
                );
            }
        }
    }

    #[test]
    fn test_max_retries_exhausted() {
        let policy = ReconnectPolicy {
            initial_delay: Duration::from_millis(10),
            max_delay: Duration::from_secs(1),
            max_retries: Some(3),
        };
        let mut state = BackoffState::new(policy);

        assert!(state.next_delay().is_some());
        assert!(state.next_delay().is_some());
        assert!(state.next_delay().is_some());
        assert!(state.next_delay().is_none());
        assert!(state.is_exhausted());
    }

    #[test]
    fn test_unlimited_retries() {
        let policy = ReconnectPolicy {
            initial_delay: Duration::from_millis(10),
            max_delay: Duration::from_millis(100),
            max_retries: None,
        };
        let mut state = BackoffState::new(policy);

        for _ in 0..100 {
            assert!(state.next_delay().is_some());
        }
        assert!(!state.is_exhausted());
    }

    #[test]
    fn test_reset() {
        let policy = ReconnectPolicy {
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            max_retries: Some(5),
        };
        let mut state = BackoffState::new(policy);

        state.next_delay().unwrap();
        state.next_delay().unwrap();
        assert_eq!(state.attempt(), 2);

        state.reset();
        assert_eq!(state.attempt(), 0);
        // Should be able to retry again
        assert!(state.next_delay().is_some());
    }

    #[test]
    fn test_total_attempts_not_reset() {
        let policy = ReconnectPolicy {
            initial_delay: Duration::from_millis(10),
            max_delay: Duration::from_millis(100),
            max_retries: Some(5),
        };
        let mut state = BackoffState::new(policy);

        state.next_delay().unwrap();
        state.next_delay().unwrap();
        assert_eq!(state.total_attempts(), 2);

        state.reset();
        assert_eq!(state.total_attempts(), 2); // Not reset
        assert_eq!(state.attempt(), 0); // But attempt counter is reset
    }

    #[test]
    fn test_delay_monotonic_before_cap() {
        let policy = ReconnectPolicy {
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(10),
            max_retries: Some(10),
        };
        let mut state = BackoffState::new(policy);

        let mut prev = Duration::ZERO;
        for _ in 0..6 {
            let delay = state.next_delay().unwrap();
            // Allow some jitter tolerance: delay should be >= prev * 0.4
            let min_expected = prev.as_millis() as f64 * 0.4;
            assert!(
                delay.as_millis() as f64 >= min_expected,
                "delay {delay:?} < expected minimum {min_expected}ms, prev was {prev:?}"
            );
            prev = delay;
        }
    }
}
