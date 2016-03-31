use std::time::Duration;


pub trait Context {
    /// Connection timeout
    ///
    /// Default is 2 seconds as redis should be pretty fast
    fn connect_timeout(&self) -> Duration {
        Duration::from_secs(2)
    }
    /// Maximal message from redis that we are willing to parse
    ///
    /// This is a small DoS prevention measure
    // TODO(tailhook) enforce it strictly
    fn max_redis_message(&self) -> usize {
        10_485_760
    }
}
