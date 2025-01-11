use syntree::Span;

pub enum NodeData {
    NonTerminal(NonTerminal),
    Terminal { terminal: Terminal, token: Token },
}

pub enum Token {
    /// A token from the original input.
    Input(Span<u32>),
    /// A token that was added after parsing.
    Added(String),
}

pub enum Terminal {
    Ws,
    Newline,
    Ident,
    Extension,
    BlockComment,
    LineComment,
    Text,
    Code,
    Number,
    Boolean,
    Unit,
    Null,
}

pub enum NonTerminal {
    Swon,
    Sections,
    Bindings,
    Section,
    Binding,
    Keys,
    Key,
    Array,
    ArrayIndex,
    Tuple,
    List,
    Map,
}

pub struct Swon {
    pub sections: Vec<SwonSection>,
    pub bindings: Vec<SwonBinding>,
}

pub struct SwonSection {
    pub keys: Vec<SwonKey>,
    pub bindings: Vec<SwonBinding>,
}

pub enum SwonKey {
    Ident(String),
    Extension(String),
    Array,
    ArrayIndex(usize),
}

pub enum SwonBinding {
    Value(SwonValue),
    Block(Swon),
}

pub enum SwonValue {
    String(String),
    Text(String),
    Code { language: String, code: String },
    Number(f64),
    Boolean(bool),
    Unit,
    Tuple(Vec<SwonValue>),
    List(Vec<SwonValue>),
    Map(ahash::HashMap<String, SwonValue>),
    Null,
}
