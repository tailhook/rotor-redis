use std::time::Duration;


pub trait Context {
    /// Connection timeout
    ///
    /// Default is 2 seconds as redis should be pretty fast
    fn connect_timeout(&self) -> Duration {
        Duration::from_secs(2)
    }
}
