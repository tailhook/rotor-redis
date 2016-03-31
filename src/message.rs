use std::str::from_utf8;
use substr::find_substr;


#[derive(Debug, PartialEq, Eq)]
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
            Some(_) => {
                unimplemented!();
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

    #[test]
    fn test_int() {
        assert_eq!(Message::parse(b":1\r\n"), Done(Int(1), 4));
    }
}
