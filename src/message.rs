pub enum Message<'a>{
    Simple(&'a str),
    Error(&'a str, &'a str),
    Int(i64),
    Bytes(&'a [u8]),
    Null,
    Array(Vec<Message<'a>>),
}
