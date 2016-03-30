use std::sync::{Arc, Mutex};

use rotor_tools::future::{FutureImpl};

use {Message};

trait PortImpl {
    fn put(&mut self, message: &Message);
}

pub struct Port(Arc<Mutex<PortImpl>>);

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
