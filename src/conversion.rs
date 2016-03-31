use std::io::Write;

use rotor_stream::Buf;

//struct RedisIter<T>(T);


pub trait ToRedisCommand {
    fn write_into(self, buf: &mut Buf);
}

pub trait ToRedisArg {
    fn write_into(self, buf: &mut Buf);
}

impl<T: AsRef<[u8]>> ToRedisArg for T {
    fn write_into(self, buf: &mut Buf) {
        let data = self.as_ref();
        write!(buf, "${}\r\n", data.len()).expect("buffer write");
        buf.write(data).expect("buffer write");
        buf.write(b"\r\n").expect("buffer write");
    }
}

/* TODO(tailhook) need to write correct array length in ToRedisCommand first
impl<I: AsRef<[u8]>, T> ToRedisArg for RedisIter<T>
    where T: ExactSizeIterator<Item=I>
{
    fn write_into(mut self, buf: &mut Buf) {
        write!(buf, "${}\r\n", self.0.len()).expect("buffer write");
        for item in self {
            buf.write(item).expect("buffer write");
            buf.write(b"\r\n").expect("buffer write");
        }
    }
}
*/

impl<A: ToRedisArg> ToRedisCommand for (A,) {
    fn write_into(self, buf: &mut Buf) {
        buf.write(b"*1\r\n").expect("buffer write");
        self.0.write_into(buf);
    }
}

impl<A, B> ToRedisCommand for (A, B)
    where A: ToRedisArg,
          B: ToRedisArg,
{
    fn write_into(self, buf: &mut Buf) {
        buf.write(b"*2\r\n").expect("buffer write");
        self.0.write_into(buf);
        self.1.write_into(buf);
    }
}

impl<A, B, C> ToRedisCommand for (A, B, C)
    where A: ToRedisArg,
          B: ToRedisArg,
          C: ToRedisArg,
{
    fn write_into(self, buf: &mut Buf) {
        buf.write(b"*3\r\n").expect("buffer write");
        self.0.write_into(buf);
        self.1.write_into(buf);
        self.2.write_into(buf);
    }
}
