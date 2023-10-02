pub struct Token {
    kind: TokenKind,
    offset: usize,
    line: usize,
    pos: usize
}

pub enum TokenKind {
    StartTag {
        name: String,
        self_closing: bool,
        attributes: Vec<Attribute>
    },
    EndTag { name: String },
    Comment(String),
    Text(String),
    DocType,
    Eof
}

pub struct Attribute {
    name: String,
    value: String
}