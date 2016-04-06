use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

extern crate memchr;
extern crate rotor;
extern crate rotor_stream;
extern crate rotor_tools;
#[macro_use] extern crate matches;
#[macro_use] extern crate log;

use rotor::{GenericScope, Response, Void, Notifier};
use rotor::mio::tcp::TcpStream;
use rotor_stream::{Persistent, ActiveStream};
use rotor_stream::sync::Mutexed;

mod proto;
mod context;
mod api;
mod message;
mod port;
mod substr;
pub mod conversion;

pub use proto::{RedisProto, Receiver};
pub use api::Promise;
pub use context::Context;
pub use message::Message;
pub use port::PortReceiver;

pub use rotor_tools::future::Future;

pub type Fsm<C, S> = Mutexed<Persistent<RedisProto<C, S, PortReceiver>>>;

pub struct Redis<C, S>(Arc<Mutex<Persistent<RedisProto<C, S, PortReceiver>>>>,
                       Notifier)
    where C: Context, S: ActiveStream;


/// Connect to the socket by IP address
///
/// The method is here while rotor-dns is not matured yet. The better way
/// would be to use dns resolving.
pub fn connect_ip<S, C>(scope: &mut S, addr: SocketAddr, db: u32)
    -> Response<(Fsm<C, TcpStream>, Redis<C, TcpStream>), Void>
    where S: GenericScope, C: Context
{
    Persistent::connect(scope, addr, (db, PortReceiver::new())).wrap(|fsm| {
        let arc = Arc::new(Mutex::new(fsm));
        (Mutexed(arc.clone()), Redis(arc, scope.notifier()))
    })
}
