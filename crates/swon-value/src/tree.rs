use thisisplural::Plural;

/// A data structure for representing a Swon document without any span information.
pub struct SwonDocument {
    sections: Vec<SwonSection>,
    bindings: Vec<SwonBinding>,
}

pub struct SwonSection {
    /// Whether the section has `{` and `}`
    nested: bool,
    keys: Vec<SwonKey>,
}

pub struct SwonBinding {
    keys: Vec<SwonKey>,
    value: SwonValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Plural)]
pub struct SwonKeys(Vec<SwonKey>);

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum SwonKey {
    Ident(String),
    String(String),
    Extension(String),
    ArrayIndex(u32),
    Array,
    TupleIndex(u8),
}

pub enum SwonValue {
    String(String),
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Array(Vec<SwonValue>),
    Tuple(Vec<SwonValue>),
    Map(ahash::HashMap<SwonValue, SwonValue>),
    Swon(SwonDocument),
}
