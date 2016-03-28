use std::marker::PhantomData;

use rotor::{Scope};
use rotor_stream::{Protocol, ActiveStream, Intent, Transport, Exception};
use {Context};

pub struct RedisProto<C, S> {
    phantom: PhantomData<*const (C, S)>,
}

impl<C: Context, S: ActiveStream> Protocol for RedisProto<C, S> {
    type Context = C;
    type Socket = S;
    /// Start with database number
    type Seed = u32;

    fn create(seed: Self::Seed, sock: &mut Self::Socket,
        scope: &mut Scope<Self::Context>) -> Intent<Self>
    {
        Intent::of(RedisProto {
            phantom: PhantomData,
        }).expect_flush().deadline(scope.now() + scope.connect_timeout())
    }
    fn bytes_read(self, transport: &mut Transport<Self::Socket>,
        end: usize, scope: &mut Scope<Self::Context>)
        -> Intent<Self>
    {
        unimplemented!();
    }
    fn bytes_flushed(self, transport: &mut Transport<Self::Socket>,
        scope: &mut Scope<Self::Context>)
        -> Intent<Self>
    {
        unimplemented!();
    }
    fn timeout(self, transport: &mut Transport<Self::Socket>,
        scope: &mut Scope<Self::Context>) -> Intent<Self>
    {
        // TODO(tailhook) fail all the requests
        unimplemented!();
    }
    fn wakeup(self, transport: &mut Transport<Self::Socket>,
        scope: &mut Scope<Self::Context>)
        -> Intent<Self>
    {
        unimplemented!();
    }

    fn exception(self, _transport: &mut Transport<Self::Socket>,
        reason: Exception, _scope: &mut Scope<Self::Context>)
        -> Intent<Self>
    {
        // TODO(tailhook) fail all the requests
        unimplemented!();
    }
}
