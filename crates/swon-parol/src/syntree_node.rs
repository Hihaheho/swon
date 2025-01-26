use parol_runtime::parser::parse_tree_type::{
    ExpectedChildren, ExpectedChildrenKinds, NodeKind, NonTerminalEnum, TerminalEnum,
};
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NonTerminalKind {
    Array,
    ArrayBegin,
    ArrayEnd,
    ArrayList,
    ArrayMarker,
    ArrayMarkerOpt,
    ArrayOpt,
    At,
    Begin,
    Bind,
    Binding,
    Bindings,
    Boolean,
    Comma,
    Continue,
    Dot,
    End,
    Ext,
    ExtensionNameSpace,
    False,
    Hole,
    Ident,
    InStr,
    Integer,
    Key,
    KeyBase,
    KeyOpt,
    Keys,
    KeysList,
    Newline,
    Null,
    Object,
    ObjectList,
    ObjectOpt,
    Quote,
    Section,
    SectionBinding,
    SectionList,
    Str,
    StrContinues,
    StrContinuesList,
    Swon,
    SwonList,
    SwonList0,
    Text,
    TextBinding,
    TextBindingOpt,
    TextStart,
    True,
    TypedQuote,
    TypedStr,
    Value,
    ValueBinding,
    Ws,
    Root,
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TerminalKind {
    NewLine,
    Whitespace,
    LineComment,
    BlockComment,
    Integer,
    True,
    False,
    Null,
    Hole,
    Quote,
    TypedQuote,
    InStr,
    Text,
    Newline,
    Ws,
    At,
    Dollar,
    Dot,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Bind,
    Comma,
    Esc,
    TextStart,
    Ident,
}
impl TerminalEnum for TerminalKind {
    fn from_terminal_index(index: u16) -> Self {
        match index {
            1 => Self::NewLine,
            2 => Self::Whitespace,
            3 => Self::LineComment,
            4 => Self::BlockComment,
            5 => Self::Integer,
            6 => Self::True,
            7 => Self::False,
            8 => Self::Null,
            9 => Self::Hole,
            10 => Self::Quote,
            11 => Self::TypedQuote,
            12 => Self::InStr,
            13 => Self::Text,
            14 => Self::Newline,
            15 => Self::Ws,
            16 => Self::At,
            17 => Self::Dollar,
            18 => Self::Dot,
            19 => Self::LBrace,
            20 => Self::RBrace,
            21 => Self::LBracket,
            22 => Self::RBracket,
            23 => Self::Bind,
            24 => Self::Comma,
            25 => Self::Esc,
            26 => Self::TextStart,
            27 => Self::Ident,
            _ => panic!("Invalid terminal index: {}", index),
        }
    }
    fn is_builtin_new_line(&self) -> bool {
        matches!(self, TerminalKind::NewLine)
    }
    fn is_builtin_whitespace(&self) -> bool {
        matches!(self, TerminalKind::Whitespace)
    }
}

impl NonTerminalEnum for NonTerminalKind {
    fn from_non_terminal_name(name: &str) -> Self {
        match name {
            "Array" => Self::Array,
            "ArrayBegin" => Self::ArrayBegin,
            "ArrayEnd" => Self::ArrayEnd,
            "ArrayList" => Self::ArrayList,
            "ArrayMarker" => Self::ArrayMarker,
            "ArrayMarkerOpt" => Self::ArrayMarkerOpt,
            "ArrayOpt" => Self::ArrayOpt,
            "At" => Self::At,
            "Begin" => Self::Begin,
            "Bind" => Self::Bind,
            "Binding" => Self::Binding,
            "Bindings" => Self::Bindings,
            "Boolean" => Self::Boolean,
            "Comma" => Self::Comma,
            "Continue" => Self::Continue,
            "Dot" => Self::Dot,
            "End" => Self::End,
            "Ext" => Self::Ext,
            "ExtensionNameSpace" => Self::ExtensionNameSpace,
            "False" => Self::False,
            "Hole" => Self::Hole,
            "Ident" => Self::Ident,
            "InStr" => Self::InStr,
            "Integer" => Self::Integer,
            "Key" => Self::Key,
            "KeyBase" => Self::KeyBase,
            "KeyOpt" => Self::KeyOpt,
            "Keys" => Self::Keys,
            "KeysList" => Self::KeysList,
            "Newline" => Self::Newline,
            "Null" => Self::Null,
            "Object" => Self::Object,
            "ObjectList" => Self::ObjectList,
            "ObjectOpt" => Self::ObjectOpt,
            "Quote" => Self::Quote,
            "Section" => Self::Section,
            "SectionBinding" => Self::SectionBinding,
            "SectionList" => Self::SectionList,
            "Str" => Self::Str,
            "StrContinues" => Self::StrContinues,
            "StrContinuesList" => Self::StrContinuesList,
            "Swon" => Self::Swon,
            "SwonList" => Self::SwonList,
            "SwonList0" => Self::SwonList0,
            "Text" => Self::Text,
            "TextBinding" => Self::TextBinding,
            "TextBindingOpt" => Self::TextBindingOpt,
            "TextStart" => Self::TextStart,
            "True" => Self::True,
            "TypedQuote" => Self::TypedQuote,
            "TypedStr" => Self::TypedStr,
            "Value" => Self::Value,
            "ValueBinding" => Self::ValueBinding,
            "Ws" => Self::Ws,
            "" => Self::Root,
            _ => panic!("Invalid non-terminal name: {}", name),
        }
    }
}
impl std::fmt::Display for TerminalKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NewLine => write!(f, stringify!(NewLine)),
            Self::Whitespace => write!(f, stringify!(Whitespace)),
            Self::LineComment => write!(f, stringify!(LineComment)),
            Self::BlockComment => write!(f, stringify!(BlockComment)),
            Self::Integer => write!(f, stringify!(Integer)),
            Self::True => write!(f, stringify!(True)),
            Self::False => write!(f, stringify!(False)),
            Self::Null => write!(f, stringify!(Null)),
            Self::Hole => write!(f, stringify!(Hole)),
            Self::Quote => write!(f, stringify!(Quote)),
            Self::TypedQuote => write!(f, stringify!(TypedQuote)),
            Self::InStr => write!(f, stringify!(InStr)),
            Self::Text => write!(f, stringify!(Text)),
            Self::Newline => write!(f, stringify!(Newline)),
            Self::Ws => write!(f, stringify!(Ws)),
            Self::At => write!(f, stringify!(At)),
            Self::Dollar => write!(f, stringify!(Dollar)),
            Self::Dot => write!(f, stringify!(Dot)),
            Self::LBrace => write!(f, stringify!(LBrace)),
            Self::RBrace => write!(f, stringify!(RBrace)),
            Self::LBracket => write!(f, stringify!(LBracket)),
            Self::RBracket => write!(f, stringify!(RBracket)),
            Self::Bind => write!(f, stringify!(Bind)),
            Self::Comma => write!(f, stringify!(Comma)),
            Self::Esc => write!(f, stringify!(Esc)),
            Self::TextStart => write!(f, stringify!(TextStart)),
            Self::Ident => write!(f, stringify!(Ident)),
        }
    }
}

impl std::fmt::Display for NonTerminalKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Array => write!(f, stringify!(Array)),
            Self::ArrayBegin => write!(f, stringify!(ArrayBegin)),
            Self::ArrayEnd => write!(f, stringify!(ArrayEnd)),
            Self::ArrayList => write!(f, stringify!(ArrayList)),
            Self::ArrayMarker => write!(f, stringify!(ArrayMarker)),
            Self::ArrayMarkerOpt => write!(f, stringify!(ArrayMarkerOpt)),
            Self::ArrayOpt => write!(f, stringify!(ArrayOpt)),
            Self::At => write!(f, stringify!(At)),
            Self::Begin => write!(f, stringify!(Begin)),
            Self::Bind => write!(f, stringify!(Bind)),
            Self::Binding => write!(f, stringify!(Binding)),
            Self::Bindings => write!(f, stringify!(Bindings)),
            Self::Boolean => write!(f, stringify!(Boolean)),
            Self::Comma => write!(f, stringify!(Comma)),
            Self::Continue => write!(f, stringify!(Continue)),
            Self::Dot => write!(f, stringify!(Dot)),
            Self::End => write!(f, stringify!(End)),
            Self::Ext => write!(f, stringify!(Ext)),
            Self::ExtensionNameSpace => write!(f, stringify!(ExtensionNameSpace)),
            Self::False => write!(f, stringify!(False)),
            Self::Hole => write!(f, stringify!(Hole)),
            Self::Ident => write!(f, stringify!(Ident)),
            Self::InStr => write!(f, stringify!(InStr)),
            Self::Integer => write!(f, stringify!(Integer)),
            Self::Key => write!(f, stringify!(Key)),
            Self::KeyBase => write!(f, stringify!(KeyBase)),
            Self::KeyOpt => write!(f, stringify!(KeyOpt)),
            Self::Keys => write!(f, stringify!(Keys)),
            Self::KeysList => write!(f, stringify!(KeysList)),
            Self::Newline => write!(f, stringify!(Newline)),
            Self::Null => write!(f, stringify!(Null)),
            Self::Object => write!(f, stringify!(Object)),
            Self::ObjectList => write!(f, stringify!(ObjectList)),
            Self::ObjectOpt => write!(f, stringify!(ObjectOpt)),
            Self::Quote => write!(f, stringify!(Quote)),
            Self::Section => write!(f, stringify!(Section)),
            Self::SectionBinding => write!(f, stringify!(SectionBinding)),
            Self::SectionList => write!(f, stringify!(SectionList)),
            Self::Str => write!(f, stringify!(Str)),
            Self::StrContinues => write!(f, stringify!(StrContinues)),
            Self::StrContinuesList => write!(f, stringify!(StrContinuesList)),
            Self::Swon => write!(f, stringify!(Swon)),
            Self::SwonList => write!(f, stringify!(SwonList)),
            Self::SwonList0 => write!(f, stringify!(SwonList0)),
            Self::Text => write!(f, stringify!(Text)),
            Self::TextBinding => write!(f, stringify!(TextBinding)),
            Self::TextBindingOpt => write!(f, stringify!(TextBindingOpt)),
            Self::TextStart => write!(f, stringify!(TextStart)),
            Self::True => write!(f, stringify!(True)),
            Self::TypedQuote => write!(f, stringify!(TypedQuote)),
            Self::TypedStr => write!(f, stringify!(TypedStr)),
            Self::Value => write!(f, stringify!(Value)),
            Self::ValueBinding => write!(f, stringify!(ValueBinding)),
            Self::Ws => write!(f, stringify!(Ws)),
            Self::Root => write!(f, stringify!()),
        }
    }
}
impl ExpectedChildren<TerminalKind, NonTerminalKind> for NonTerminalKind {
    fn expected_children(&self) -> ExpectedChildrenKinds<TerminalKind, NonTerminalKind> {
        match self {
            Self::Array => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::ArrayBegin),
                NodeKind::NonTerminal(NonTerminalKind::ArrayList /* Vec */),
                NodeKind::NonTerminal(NonTerminalKind::ArrayEnd),
            ]),
            Self::ArrayBegin => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::LBracket)])
            }
            Self::ArrayEnd => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::RBracket)])
            }
            Self::ArrayList => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::Value),
                NodeKind::NonTerminal(NonTerminalKind::ArrayOpt /* Option */),
                NodeKind::NonTerminal(NonTerminalKind::ArrayList),
            ]),
            Self::ArrayMarker => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::ArrayBegin),
                NodeKind::NonTerminal(NonTerminalKind::ArrayMarkerOpt /* Option */),
                NodeKind::NonTerminal(NonTerminalKind::ArrayEnd),
            ]),
            Self::ArrayMarkerOpt => ExpectedChildrenKinds::OneOf(&[]),
            Self::ArrayOpt => ExpectedChildrenKinds::OneOf(&[]),
            Self::At => ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::At)]),
            Self::Begin => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::LBrace)])
            }
            Self::Bind => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::Bind)])
            }
            Self::Binding => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::Keys),
                NodeKind::NonTerminal(NonTerminalKind::Bindings),
            ]),
            Self::Bindings => ExpectedChildrenKinds::OneOf(&[
                NodeKind::NonTerminal(NonTerminalKind::ValueBinding),
                NodeKind::NonTerminal(NonTerminalKind::SectionBinding),
                NodeKind::NonTerminal(NonTerminalKind::TextBinding),
            ]),
            Self::Boolean => ExpectedChildrenKinds::OneOf(&[]),
            Self::Comma => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::Comma)])
            }
            Self::Continue => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::Esc)])
            }
            Self::Dot => ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::Dot)]),
            Self::End => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::RBrace)])
            }
            Self::Ext => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::Dollar)])
            }
            Self::ExtensionNameSpace => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::Ext),
                NodeKind::NonTerminal(NonTerminalKind::Ident),
            ]),
            Self::False => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::False)])
            }
            Self::Hole => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::Hole)])
            }
            Self::Ident => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::Ident)])
            }
            Self::InStr => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::InStr)])
            }
            Self::Integer => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::Integer)])
            }
            Self::Key => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::KeyBase),
                NodeKind::NonTerminal(NonTerminalKind::KeyOpt /* Option */),
            ]),
            Self::KeyBase => ExpectedChildrenKinds::OneOf(&[
                NodeKind::NonTerminal(NonTerminalKind::Ident),
                NodeKind::NonTerminal(NonTerminalKind::ExtensionNameSpace),
                NodeKind::NonTerminal(NonTerminalKind::Str),
            ]),
            Self::KeyOpt => ExpectedChildrenKinds::OneOf(&[]),
            Self::Keys => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::Key),
                NodeKind::NonTerminal(NonTerminalKind::KeysList /* Vec */),
            ]),
            Self::KeysList => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::Dot),
                NodeKind::NonTerminal(NonTerminalKind::Key),
                NodeKind::NonTerminal(NonTerminalKind::KeysList),
            ]),
            Self::Newline => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::Newline)])
            }
            Self::Null => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::Null)])
            }
            Self::Object => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::Begin),
                NodeKind::NonTerminal(NonTerminalKind::ObjectList /* Vec */),
                NodeKind::NonTerminal(NonTerminalKind::End),
            ]),
            Self::ObjectList => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::Key),
                NodeKind::NonTerminal(NonTerminalKind::Bind),
                NodeKind::NonTerminal(NonTerminalKind::Value),
                NodeKind::NonTerminal(NonTerminalKind::ObjectOpt /* Option */),
                NodeKind::NonTerminal(NonTerminalKind::ObjectList),
            ]),
            Self::ObjectOpt => ExpectedChildrenKinds::OneOf(&[]),
            Self::Quote => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::Quote)])
            }
            Self::Section => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::At),
                NodeKind::NonTerminal(NonTerminalKind::Keys),
                NodeKind::NonTerminal(NonTerminalKind::SectionList /* Vec */),
            ]),
            Self::SectionBinding => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::Begin),
                NodeKind::NonTerminal(NonTerminalKind::Swon),
                NodeKind::NonTerminal(NonTerminalKind::End),
            ]),
            Self::SectionList => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::Binding),
                NodeKind::NonTerminal(NonTerminalKind::SectionList),
            ]),
            Self::Str => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::Quote),
                NodeKind::NonTerminal(NonTerminalKind::InStr),
                NodeKind::NonTerminal(NonTerminalKind::Quote),
            ]),
            Self::StrContinues => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::Str),
                NodeKind::NonTerminal(NonTerminalKind::StrContinuesList /* Vec */),
            ]),
            Self::StrContinuesList => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::Continue),
                NodeKind::NonTerminal(NonTerminalKind::Str),
                NodeKind::NonTerminal(NonTerminalKind::StrContinuesList),
            ]),
            Self::Swon => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::SwonList /* Vec */),
                NodeKind::NonTerminal(NonTerminalKind::SwonList0 /* Vec */),
            ]),
            Self::SwonList => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::Binding),
                NodeKind::NonTerminal(NonTerminalKind::SwonList),
            ]),
            Self::SwonList0 => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::Section),
                NodeKind::NonTerminal(NonTerminalKind::SwonList0),
            ]),
            Self::Text => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::Text)])
            }
            Self::TextBinding => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::TextStart),
                NodeKind::NonTerminal(NonTerminalKind::TextBindingOpt /* Option */),
                NodeKind::NonTerminal(NonTerminalKind::Text),
                NodeKind::NonTerminal(NonTerminalKind::Newline),
            ]),
            Self::TextBindingOpt => ExpectedChildrenKinds::OneOf(&[]),
            Self::TextStart => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::TextStart)])
            }
            Self::True => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::True)])
            }
            Self::TypedQuote => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::TypedQuote)])
            }
            Self::TypedStr => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::TypedQuote),
                NodeKind::NonTerminal(NonTerminalKind::InStr),
                NodeKind::NonTerminal(NonTerminalKind::Quote),
            ]),
            Self::Value => ExpectedChildrenKinds::OneOf(&[
                NodeKind::NonTerminal(NonTerminalKind::Object),
                NodeKind::NonTerminal(NonTerminalKind::Array),
                NodeKind::NonTerminal(NonTerminalKind::Integer),
                NodeKind::NonTerminal(NonTerminalKind::Boolean),
                NodeKind::NonTerminal(NonTerminalKind::Null),
                NodeKind::NonTerminal(NonTerminalKind::StrContinues),
                NodeKind::NonTerminal(NonTerminalKind::TypedStr),
                NodeKind::NonTerminal(NonTerminalKind::Hole),
            ]),
            Self::ValueBinding => ExpectedChildrenKinds::Sequence(&[
                NodeKind::NonTerminal(NonTerminalKind::Bind),
                NodeKind::NonTerminal(NonTerminalKind::Value),
            ]),
            Self::Ws => ExpectedChildrenKinds::Sequence(&[NodeKind::Terminal(TerminalKind::Ws)]),
            Self::Root => {
                ExpectedChildrenKinds::Sequence(&[NodeKind::NonTerminal(NonTerminalKind::Swon)])
            }
        }
    }
}
