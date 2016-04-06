use std::sync::{Arc, Mutex};
use std::collections::VecDeque;

use rotor_tools::future::{FutureImpl};

use {Message, Receiver};

// TODO(tailhook) make private
pub trait PortImpl {
    fn put(&mut self, message: &Message);
}

// TODO(tailhook) make internals private
pub struct Port(pub Arc<Mutex<PortImpl>>);

pub struct PortReceiver {
    pipeline: VecDeque<Port>,
}

impl Port {
    pub fn put(self, val: &Message) {
        self.0.lock().expect("port locked").put(val)
    }
}

impl<'a, O, F> PortImpl for FutureImpl<&'a Message<'a>, O, F>
    where F: FnOnce(&Message) -> O
{
    fn put(&mut self, val: &Message) {
        let converted = self.convert()(val);
        self.put(converted);
    }
}

impl PortReceiver {
    pub fn new() -> PortReceiver {
        PortReceiver {
            pipeline: VecDeque::new(),
        }
    }
    pub fn add_port(&mut self, port: Port) {
        self.pipeline.push_back(port)
    }
}

impl Clone for PortReceiver {
    fn clone(&self) -> PortReceiver {
        PortReceiver {
            pipeline: VecDeque::new(),
        }
    }
}

impl Receiver for PortReceiver {
    fn receive(&mut self, msg: &Message) {
        self.pipeline.pop_front()
        .expect("pipeline is not empty")
        .put(msg)
    }
}
