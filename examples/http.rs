extern crate argparse;
#[macro_use] extern crate rotor;
extern crate rotor_http;
extern crate rotor_redis;
extern crate rotor_tools;

use std::time::Duration;

use argparse::{ArgumentParser, Store};
use rotor::{Scope, Time};
use rotor::mio::tcp::TcpStream;
use rotor_http::server::{RecvMode, Server, Head, Response};
use rotor::mio::tcp::TcpListener;
use rotor_tools::loop_ext::LoopExt;

use rotor_redis::{connect_ip, Redis, Future};
use rotor_redis::Message::Int;

struct Context {
    redis: Redis<Context, TcpStream>,
}

impl rotor_redis::Context for Context {}

rotor_compose! {
    enum Fsm/Seed<Context> {
        Redis(rotor_redis::Fsm<Context, TcpStream>),
        Http(rotor_http::server::Fsm<HelloWorld, TcpListener>),
    }
}

struct HelloWorld(Future<i64>);

fn send_string<B: AsRef<[u8]>>(res: &mut Response, data: B) {
    let data = data.as_ref();
    res.status(200, "OK");
    res.add_length(data.len() as u64).unwrap();
    res.done_headers().unwrap();
    res.write_body(data);
    res.done();
}

impl Server for HelloWorld {
    type Seed = ();
    type Context = Context;
    fn headers_received(_seed: (), _head: Head, _res: &mut Response,
        scope: &mut Scope<Context>)
        -> Option<(Self, RecvMode, Time)>
    {
        let notifier = scope.notifier();
        let future = scope.redis.incr("hello-world").then(notifier, |msg| {
            match *msg {
                Int(x) => x,
                _ => unreachable!(),
            }
        });
        Some((HelloWorld(future), RecvMode::Buffered(1024),
            scope.now() + Duration::new(10, 0)))
    }
    fn request_received(self, _data: &[u8], _res: &mut Response,
        _scope: &mut Scope<Context>)
        -> Option<Self>
    {
        Some(self)
    }
    fn request_chunk(self, _chunk: &[u8], _response: &mut Response,
        _scope: &mut Scope<Context>)
        -> Option<Self>
    {
        unreachable!();
    }

    /// End of request body, only for Progressive requests
    fn request_end(self, _response: &mut Response, _scope: &mut Scope<Context>)
        -> Option<Self>
    {
        unreachable!();
    }

    fn timeout(self, _response: &mut Response, _scope: &mut Scope<Context>)
        -> Option<(Self, Time)>
    {
        panic!("TIMEOUT: not impelmented");
    }
    fn wakeup(self, res: &mut Response, _scope: &mut Scope<Context>)
        -> Option<Self>
    {
        match self.0.consume() {
            Ok(n) => {
                send_string(res, format!("Hello, {}th visitor", n));
                None
            }
            Err(future) => {
                Some(HelloWorld(future))
            }
        }
    }
}

fn main() {

    let mut redis_host = "127.0.0.1".to_string();
    let mut redis_port = 2003u16;
    let mut redis_db = 0u32;
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Execute redis commands. It's similar to
        redis-cli but has no interactive mode and othervise very simple");
        ap.refer(&mut redis_host).add_option(&["--redis-host"], Store, "
            Redis host to connect to. Name resolution is done on start only.");
        ap.refer(&mut redis_port).add_option(&["--redis-port"], Store, "
            Redis port to connect to. Default is 6379 which is the default port
            for redis.");
        ap.refer(&mut redis_db).add_option(&["--redis-db"], Store, "
            The database number (default `0`)");
        ap.parse_args_or_exit();
    }

    println!("Starting http server on http://127.0.0.1:3000/");
    println!("Expecting redis at redis://{}:{}/{}",
        redis_host, redis_port, redis_db);
    let mut loop_creator = rotor::Loop::new(&rotor::Config::new()).unwrap();
    let redis: Redis<Context, _> = loop_creator.add_and_fetch(Fsm::Redis,
        |scope| {
            connect_ip(scope, format!("{}:{}", redis_host, redis_port)
                              .parse().unwrap(), redis_db)
        }).unwrap();
    let mut loop_inst = loop_creator.instantiate(Context {
        redis: redis,
    });
    let lst = TcpListener::bind(&"127.0.0.1:3000".parse().unwrap()).unwrap();
    loop_inst.add_machine_with(|scope| {
        rotor_http::server::Fsm::<HelloWorld, _>::new(lst, (), scope)
        .wrap(Fsm::Http)
    }).unwrap();
    loop_inst.run().unwrap();
}
