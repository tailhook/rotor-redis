use std::str::from_utf8;
use substr::find_substr;


#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Message<'a>{
    Simple(&'a str),
    Error(&'a str, &'a str),
    Int(i64),
    Bytes(&'a [u8]),
    Null,
    Array(Vec<Message<'a>>),  // TODO(tailhook) eliminate memory allocation
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expectation {
    Newline,
    More(usize),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseResult<'a> {
    Done(Message<'a>, usize),
    Expect(Expectation),
    InvalidData, // TODO(tailhook) add error description
}

impl<'a> Message<'a> {
    pub fn parse(bytes: &[u8]) -> ParseResult {
        use self::Message::*;
        use self::ParseResult::*;
        use self::Expectation::*;
        match bytes.get(0) {
            Some(&b':') => {
                match find_substr(&bytes[1..], b"\r\n") {
                    Some(end) => {
                        match from_utf8(&bytes[1..end+1]).ok()
                              .and_then(|x| x.parse().ok())
                        {
                            Some(x) => return Done(Int(x), end+1+2),
                            None => return InvalidData,
                        }
                    }
                    None => {
                        return Expect(Newline);
                    }
                }
            }
            Some(&b'+') => {
                match find_substr(&bytes[1..], b"\r\n") {
                    Some(end) => {
                        match from_utf8(&bytes[1..end+1]).ok() {
                            Some(x) => return Done(Simple(x), end+1+2),
                            None => return InvalidData,
                        }
                    }
                    None => {
                        return Expect(Newline);
                    }
                }
            }
            Some(_) => {
                panic!("Unimplemented data {:?}", from_utf8(bytes));
            }
            None => Expect(More(1)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Message;
    use super::Message::*;
    use super::ParseResult::*;

    fn partial_compare(src: &[u8], tgt: Message) {
        assert_eq!(Message::parse(src), Done(tgt.clone(), src.len()));
        for end in 0..src.len() {
            match Message::parse(&src[..end]) {
                Done(value, bytes) => {
                    panic!("premature parser exit for {:?} -> {:?} / {}",
                        String::from_utf8_lossy(&src[..end]), value, bytes);
                }
                Expect(_) => {},
                InvalidData => {
                    panic!("Can't parse partial {:?}",
                        String::from_utf8_lossy(&src[..end]));
                }
            }
        }
        let garbage = b"$-1\r\n$:102\r\n*12\r\n$$$$";
        let mut vec = src.to_vec();
        vec.extend(garbage);
        assert_eq!(Message::parse(&vec), Done(tgt, src.len()));
    }

    #[test]
    fn test_int() {
        assert_eq!(Message::parse(b":1\r\n"), Done(Int(1), 4));
    }
    #[test]
    fn test_ok() {
        assert_eq!(Message::parse(b"+OK\r\n"), Done(Simple("OK"), 5));
    }
}
