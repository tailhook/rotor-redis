use std::error::Error;
use std::marker::PhantomData;
use std::collections::VecDeque;

use rotor::{Scope};
use rotor_stream::{Protocol, ActiveStream, Intent, Transport, Exception};
use rotor_tools::future::{new as future, Future, MakeFuture};

use {Context};
use conversion::ToRedisCommand;
use port::Port;
use message::Message;


enum State {
    Connecting,
    SettingDb,
    Operating,
}

pub trait Receiver {
    fn receive(&mut self, &Message);
}

pub struct RedisProto<C, S, R> {
    pub receiver: R,
    db: u32,
    state: State,
    phantom: PhantomData<*const (C, S)>,
}

impl<C, S, R> Protocol for RedisProto<C, S, R>
    where C: Context, S: ActiveStream, R: Receiver
{
    type Context = C;
    type Socket = S;
    /// Start with database number
    type Seed = (u32, R);

    fn create((db, recv): Self::Seed, _sock: &mut Self::Socket,
        scope: &mut Scope<Self::Context>) -> Intent<Self>
    {
        Intent::of(RedisProto {
            receiver: recv,
            db: db,
            state: State::Connecting,
            phantom: PhantomData,
        }).expect_flush().deadline(scope.now() + scope.connect_timeout())
    }
    fn bytes_read(mut self, transport: &mut Transport<Self::Socket>,
        _end: usize, scope: &mut Scope<Self::Context>)
        -> Intent<Self>
    {
        use self::State::*;
        use message::ParseResult::*;
        use message::Expectation::*;
        let inp = transport.input();
        let bytes_read = inp.len();
        let bytes = match Message::parse(&inp[..]) {
            Done(msg, bytes) => {
                match self.state {
                    Connecting => {
                        debug!("Got response when no request");
                        // TODO(tailhook) make it Intent::error?
                        return Intent::done();
                    }
                    SettingDb => {
                        if !matches!(msg, Message::Simple("OK")) {
                            debug!("Got bad message to SettingDb");
                            // TODO(tailhook) make it Intent::error?
                            return Intent::done();
                        }
                        self.state = Operating;
                    }
                    Operating => {
                        self.receiver.receive(&msg);
                    }
                }
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
    fn bytes_flushed(mut self, transport: &mut Transport<Self::Socket>,
        scope: &mut Scope<Self::Context>)
        -> Intent<Self>
    {
        use self::State::*;
        match self.state {
            Connecting => {
                let imp = future(scope, |msg: &Message| {
                    msg == &Message::Simple("OK")
                });
                ("SELECT", format!("{}", self.db))
                    .write_into(transport.output());
                self.state = SettingDb;
            }
            _ => {
                // TODO(tailhook) implement pinging the connections
            }
        }
        Intent::of(self).expect_bytes(1)
    }
    fn timeout(self, _transport: &mut Transport<Self::Socket>,
        _scope: &mut Scope<Self::Context>) -> Intent<Self>
    {
        // TODO(tailhook) fail all the requests
        unimplemented!();
    }
    fn wakeup(mut self, _transport: &mut Transport<Self::Socket>,
        _scope: &mut Scope<Self::Context>)
        -> Intent<Self>
    {
        Intent::of(self).expect_bytes(1)
    }

    fn exception(self, _transport: &mut Transport<Self::Socket>,
        _reason: Exception, _scope: &mut Scope<Self::Context>)
        -> Intent<Self>
    {
        // TODO(tailhook) fail all the requests
        unimplemented!();
    }
    fn fatal(self, _reason: Exception, _scope: &mut Scope<Self::Context>)
        -> Option<Box<Error>>
    {
        // TODO(tailhook) fail all the requests
        unimplemented!();
    }
}
