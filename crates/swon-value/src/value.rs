use ahash::HashMap;
use ahash::HashMapExt;
use thisisplural::Plural;

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    I64(i64),
    U64(u64),
    F32(f32),
    F64(f64),
    String(String),
    TypedString(TypedString),
    Code(Code),
    Array(Array),
    Tuple(Tuple),
    Map(Map),
    Variant(Variant),
    Unit,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypedString {
    pub type_name: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Code {
    pub language: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Plural, Default)]
pub struct Array(pub Vec<Value>);

#[derive(Debug, Clone, PartialEq, Plural, Default)]
pub struct Tuple(pub Vec<Value>);

#[derive(Debug, Clone, PartialEq, Plural, Default)]
pub struct Map(pub HashMap<String, Value>);

#[derive(Debug, Clone, PartialEq)]
pub struct Variant {
    pub tag: String,
    pub content: Box<Value>,
}

pub struct Unit;
