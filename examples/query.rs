extern crate argparse;
extern crate rotor;
extern crate rotor_redis;
extern crate rotor_tools;

use argparse::{ArgumentParser, Store, List};
use rotor_redis::{connect_ip, Redis};
use rotor_tools::loop_ext::LoopExt;


struct Context;

impl rotor_redis::Context for Context {
    // all defaults
}


fn main() {
    let mut host = "127.0.0.1".to_string();
    let mut port = 2003u16;
    let mut db = 0u32;
    let mut commands = Vec::<String>::new();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description("Execute redis commands. It's similar to
        redis-cli but has no interactive mode and othervise very simple");
        ap.refer(&mut host).add_option(&["--host"], Store, "
            Host to connect to. Name resolution is done on start only.");
        ap.refer(&mut port).add_option(&["--port"], Store, "
            Port to connect to. Default is 6379 which is the default port
            for redis.");
        ap.refer(&mut db).add_option(&["--db"], Store, "
            The database number (default `0`)");
        ap.refer(&mut commands).add_argument("commmand", List, "
            Commands with arguments to execute on redis server");
        ap.parse_args_or_exit();
    }

    let mut loop_creator = rotor::Loop::new(
        &rotor::Config::new()).unwrap();
    let redis: Redis<Context, _> = loop_creator.add_and_fetch(|x| x, |scope| {
        connect_ip(scope,
            format!("{}:{}", host, port).parse().unwrap(), db)
    }).unwrap();

}
