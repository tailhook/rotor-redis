use std::error::Error;
use std::marker::PhantomData;
use std::collections::VecDeque;

use rotor::{Scope};
use rotor_stream::{Protocol, ActiveStream, Intent, Transport, Exception};

use {Context};
use port::Port;
use message::Message;

pub struct RedisProto<C, S> {
    pub pipeline: VecDeque<Port>,
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
            pipeline: VecDeque::new(),
            phantom: PhantomData,
        }).expect_flush().deadline(scope.now() + scope.connect_timeout())
    }
    fn bytes_read(mut self, transport: &mut Transport<Self::Socket>,
        end: usize, scope: &mut Scope<Self::Context>)
        -> Intent<Self>
    {
        use message::ParseResult::*;
        use message::Expectation::*;
        let inp = transport.input();
        let bytes_read = inp.len();
        let bytes = match Message::parse(&inp[..]) {
            Done(msg, bytes) => {
                self.pipeline.pop_front().expect("request in a pipeline")
                    .put(&msg);
                bytes
            }
            Expect(x) => match x {
                Newline => {
                    // TODO(tailhook) set a deadline
                    return Intent::of(self).expect_delimiter_after(
                                bytes_read-1, b"\r\n",
                                scope.max_redis_message());
                }
                More(x) => {
                    // TODO(tailhook) set a deadline
                    return Intent::of(self).expect_bytes(x);
                }
            },
            InvalidData => {
                // TODO(tailhook) pass correct exception
                return Intent::done();
            }
        };
        inp.consume(bytes);
        return Intent::of(self).expect_bytes(1);
    }
    fn bytes_flushed(self, transport: &mut Transport<Self::Socket>,
        scope: &mut Scope<Self::Context>)
        -> Intent<Self>
    {
        // TODO(tailhook) this is a good time to fetch things from
        // out of the connection queue
        //
        // TODO(tailhook) implement pinging the connections
        Intent::of(self).sleep()
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
        // This is called by external code to flush the buffers
        //
        // TODO(tailhook) derive the real intent
        Intent::of(self).expect_bytes(1)
    }

    fn exception(self, _transport: &mut Transport<Self::Socket>,
        reason: Exception, _scope: &mut Scope<Self::Context>)
        -> Intent<Self>
    {
        // TODO(tailhook) fail all the requests
        unimplemented!();
    }
    fn fatal(self, reason: Exception, _scope: &mut Scope<Self::Context>)
        -> Option<Box<Error>>
    {
        // TODO(tailhook) fail all the requests
        unimplemented!();
    }
}
