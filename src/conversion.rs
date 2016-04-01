use std::io::{Write, Cursor};

use rotor_stream::Buf;

pub struct RedisIter<T>(T);


pub trait ToRedisCommand {
    fn write_into(self, buf: &mut Buf);
}

pub trait ToRedisArg {
    fn num_args(&self) -> usize { 1 }
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

impl<I: AsRef<[u8]>, T> ToRedisArg for RedisIter<T>
    where T: ExactSizeIterator<Item=I>
{
    fn num_args(&self) -> usize {
        return self.0.len();
    }
    fn write_into(self, buf: &mut Buf) {
        let num = self.0.len();
        let mut x = 0;
        for item in self.0 {
            x += 1;
            let data = item.as_ref();
            write!(buf, "${}\r\n", data.len()).expect("buffer write");
            buf.write(data).expect("buffer write");
            buf.write(b"\r\n").expect("buffer write");
        }
        // This check also assumes that `len()` can't change between calls
        assert_eq!(x, num);
    }
}

impl<A: ToRedisArg> ToRedisCommand for (A,) {
    fn write_into(self, buf: &mut Buf) {
        let n = self.0.num_args();
        write!(buf, "*{}\r\n", n).expect("buffer write");
        self.0.write_into(buf);
    }
}

impl<A, B> ToRedisCommand for (A, B)
    where A: ToRedisArg,
          B: ToRedisArg,
{
    fn write_into(self, buf: &mut Buf) {
        let n = self.0.num_args() +
                self.1.num_args();
        write!(buf, "*{}\r\n", n).expect("buffer write");
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
        let n = self.0.num_args() +
                self.1.num_args() +
                self.2.num_args();
        write!(buf, "*{}\r\n", n).expect("buffer write");
        self.0.write_into(buf);
        self.1.write_into(buf);
        self.2.write_into(buf);
    }
}

impl<A, B, C, D> ToRedisCommand for (A, B, C, D)
    where A: ToRedisArg,
          B: ToRedisArg,
          C: ToRedisArg,
          D: ToRedisArg,
{
    fn write_into(self, buf: &mut Buf) {
        let n = self.0.num_args() +
                self.1.num_args() +
                self.2.num_args() +
                self.3.num_args();
        write!(buf, "*{}\r\n", n).expect("buffer write");
        self.0.write_into(buf);
        self.1.write_into(buf);
        self.2.write_into(buf);
        self.3.write_into(buf);
    }
}

impl<A, B, C, D, E> ToRedisCommand for (A, B, C, D, E)
    where A: ToRedisArg,
          B: ToRedisArg,
          C: ToRedisArg,
          D: ToRedisArg,
          E: ToRedisArg,
{
    fn write_into(self, buf: &mut Buf) {
        let n = self.0.num_args() +
                self.1.num_args() +
                self.2.num_args() +
                self.3.num_args() +
                self.4.num_args();
        write!(buf, "*{}\r\n", n).expect("buffer write");
        self.0.write_into(buf);
        self.1.write_into(buf);
        self.2.write_into(buf);
        self.3.write_into(buf);
        self.4.write_into(buf);
    }
}

impl<A, B, C, D, E, F> ToRedisCommand for (A, B, C, D, E, F)
    where A: ToRedisArg,
          B: ToRedisArg,
          C: ToRedisArg,
          D: ToRedisArg,
          E: ToRedisArg,
          F: ToRedisArg,
{
    fn write_into(self, buf: &mut Buf) {
        let n = self.0.num_args() +
                self.1.num_args() +
                self.2.num_args() +
                self.3.num_args() +
                self.4.num_args() +
                self.5.num_args();
        write!(buf, "*{}\r\n", n).expect("buffer write");
        self.0.write_into(buf);
        self.1.write_into(buf);
        self.2.write_into(buf);
        self.3.write_into(buf);
        self.4.write_into(buf);
        self.5.write_into(buf);
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;
    use std::str::from_utf8;
    use rotor_stream::Buf;
    use super::{ToRedisArg, ToRedisCommand, RedisIter};

    #[test]
    fn str_arg() {
        let mut buf = Buf::new();
        "test".write_into(&mut buf);
        assert_eq!(&buf[..], b"$4\r\ntest\r\n");
    }

    #[test]
    fn longer_str_arg() {
        let mut buf = Buf::new();
        "123456789012".write_into(&mut buf);
        assert_eq!(&buf[..], b"$12\r\n123456789012\r\n");
    }

    #[test]
    fn single_arg() {
        let mut buf = Buf::new();
        ("hello",).write_into(&mut buf);
        assert_eq!(&buf[..], b"*1\r\n$5\r\nhello\r\n");
        buf.remove_range(..);
        (b"world",).write_into(&mut buf);
        assert_eq!(&buf[..], b"*1\r\n$5\r\nworld\r\n");
    }

    #[test]
    fn two_args() {
        let mut buf = Buf::new();
        ("GET", b"mykey").write_into(&mut buf);
        assert_eq!(&buf[..], b"*2\r\n$3\r\nGET\r\n$5\r\nmykey\r\n");
    }

    #[test]
    fn three_args() {
        let mut buf = Buf::new();
        ("SET", "mykey", String::from("my/value")).write_into(&mut buf);
        assert_eq!(&buf[..],
            "*3\r\n$3\r\nSET\r\n$5\r\nmykey\r\n$8\r\nmy/value\r\n".as_bytes());
    }

    #[test]
    fn iter_args() {
        let mut buf = Buf::new();
        ("BLPOP", RedisIter(["k1", "k2", "k3"].iter()), "128").write_into(&mut buf);
        assert_eq!(from_utf8(&buf[..]).unwrap(),
            "*5\r\n$5\r\nBLPOP\r\n$2\r\nk1\r\n$2\r\nk2\r\n$2\r\nk3\r\n\
             $3\r\n128\r\n");
    }
}

