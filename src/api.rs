use std::fmt::Debug;

use rotor_tools::future::{new as future, Future, GetNotifier, MakeFuture};
use rotor_stream::ActiveStream;

use {Redis, Context, Message};
use port::Port;

trait ToRedisCommand { }

pub struct Promise<'a, X: 'a, S: 'a, C>(&'a Redis<X, S>, C)
    where X: Context, S: ActiveStream, S::Address: Clone + Debug;

impl<'a, X: 'a, S: 'a, C: ToRedisCommand> Promise<'a, X, S, C>
    where X: Context, S: ActiveStream, S::Address: Clone + Debug
{
    pub fn then<O, F, N>(self, notifier: N, fun: F) -> Future<O>
        where O: Sized + 'static,
              F: FnOnce(&Message) -> O + 'static,
              N: GetNotifier
    {
        let mut lock = (self.0).0.lock().expect("lock redis connection");
        // TODO(tailhook) write data to the buffer
        let imp = future(notifier, fun);
        lock.protocol().expect("valid redis proto")
            .pipeline.push_back(Port(imp.clone()));
        imp.make_future()
    }
}

impl<'a, K: AsRef<[u8]> + Sized> ToRedisCommand for (&'a str, K) {
}

// Should be moved to traits
impl<C, S> Redis<C, S>
    where C: Context, S: ActiveStream, S::Address: Clone + Debug
{
    pub fn incr<K: AsRef<[u8]> + Sized>(&self, key: K)
        -> Promise<C, S, (&str, K)>
    {
        Promise(self, ("INCR", key))
    }
}
