use std::fmt::Debug;

use rotor_tools::future::{new as future, Future, GetNotifier, MakeFuture};
use rotor_stream::{ActiveStream, Buf};

use {Redis, Context, Message};
use port::Port;
use conversion::ToRedisCommand;

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
        // TODO(tailhook) write data to the buffer
        let imp = future(notifier, fun);

        let mut lock = (self.0).0.lock().expect("lock redis connection");
        self.1.write_into(
            lock.transport().expect("valid redis transport").output());
        lock.protocol().expect("valid redis proto")
            .pipeline.push_back(Port(imp.clone()));
        (self.0).1.wakeup().expect("redis notify");

        imp.make_future()
    }
}

// Should be moved to traits
impl<C, S> Redis<C, S>
    where C: Context, S: ActiveStream, S::Address: Clone + Debug
{
    pub fn get<K: AsRef<[u8]> + Sized>(&self, key: K)
        -> Promise<C, S, (&str, K)>
    {
        Promise(self, ("GET", key))
    }
    pub fn set<K: AsRef<[u8]> + Sized>(&self, key: K)
        -> Promise<C, S, (&str, K)>
    {
        Promise(self, ("SET", key))
    }
    pub fn incr<K: AsRef<[u8]> + Sized>(&self, key: K)
        -> Promise<C, S, (&str, K)>
    {
        Promise(self, ("INCR", key))
    }
}
