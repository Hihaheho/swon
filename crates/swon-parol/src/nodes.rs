use parol_runtime::parser::parse_tree_type::{
    ChildAttribute, ChildKind, ExpectedChildren, ExpectedChildrenKinds, Node, NodeKind,
    NonTerminalEnum, TerminalEnum,
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
    BindingRhs,
    Boolean,
    Code,
    CodeBlock,
    CodeBlockDelimiter,
    CodeBlockLine,
    CodeBlockTailCommon,
    CodeBlockTailCommonList,
    CodeBlockTailCommonOpt,
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
    NamedCode,
    NamedCodeBlock,
    NamedCodeBlockBegin,
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
    NamedCode,
    Code,
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
    NamedCodeBlockBegin,
    CodeBlockDelimiter,
    CodeBlockLine,
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
            14 => Self::NamedCode,
            15 => Self::Code,
            16 => Self::Newline,
            17 => Self::Ws,
            18 => Self::At,
            19 => Self::Dollar,
            20 => Self::Dot,
            21 => Self::LBrace,
            22 => Self::RBrace,
            23 => Self::LBracket,
            24 => Self::RBracket,
            25 => Self::Bind,
            26 => Self::Comma,
            27 => Self::Esc,
            28 => Self::TextStart,
            29 => Self::Ident,
            30 => Self::NamedCodeBlockBegin,
            31 => Self::CodeBlockDelimiter,
            32 => Self::CodeBlockLine,
            _ => panic!("Invalid terminal index: {}", index),
        }
    }
    fn is_builtin_terminal(&self) -> bool {
        matches!(
            self,
            TerminalKind::NewLine
                | TerminalKind::Whitespace
                | TerminalKind::LineComment
                | TerminalKind::BlockComment
        )
    }
    fn is_builtin_new_line(&self) -> bool {
        matches!(self, TerminalKind::NewLine)
    }
    fn is_builtin_whitespace(&self) -> bool {
        matches!(self, TerminalKind::Whitespace)
    }
    fn is_builtin_line_comment(&self) -> bool {
        matches!(self, TerminalKind::LineComment)
    }
    fn is_builtin_block_comment(&self) -> bool {
        matches!(self, TerminalKind::BlockComment)
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
            "BindingRhs" => Self::BindingRhs,
            "Boolean" => Self::Boolean,
            "Code" => Self::Code,
            "CodeBlock" => Self::CodeBlock,
            "CodeBlockDelimiter" => Self::CodeBlockDelimiter,
            "CodeBlockLine" => Self::CodeBlockLine,
            "CodeBlockTailCommon" => Self::CodeBlockTailCommon,
            "CodeBlockTailCommonList" => Self::CodeBlockTailCommonList,
            "CodeBlockTailCommonOpt" => Self::CodeBlockTailCommonOpt,
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
            "NamedCode" => Self::NamedCode,
            "NamedCodeBlock" => Self::NamedCodeBlock,
            "NamedCodeBlockBegin" => Self::NamedCodeBlockBegin,
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
            Self::NamedCode => write!(f, stringify!(NamedCode)),
            Self::Code => write!(f, stringify!(Code)),
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
            Self::NamedCodeBlockBegin => write!(f, stringify!(NamedCodeBlockBegin)),
            Self::CodeBlockDelimiter => write!(f, stringify!(CodeBlockDelimiter)),
            Self::CodeBlockLine => write!(f, stringify!(CodeBlockLine)),
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
            Self::BindingRhs => write!(f, stringify!(BindingRhs)),
            Self::Boolean => write!(f, stringify!(Boolean)),
            Self::Code => write!(f, stringify!(Code)),
            Self::CodeBlock => write!(f, stringify!(CodeBlock)),
            Self::CodeBlockDelimiter => write!(f, stringify!(CodeBlockDelimiter)),
            Self::CodeBlockLine => write!(f, stringify!(CodeBlockLine)),
            Self::CodeBlockTailCommon => write!(f, stringify!(CodeBlockTailCommon)),
            Self::CodeBlockTailCommonList => write!(f, stringify!(CodeBlockTailCommonList)),
            Self::CodeBlockTailCommonOpt => write!(f, stringify!(CodeBlockTailCommonOpt)),
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
            Self::NamedCode => write!(f, stringify!(NamedCode)),
            Self::NamedCodeBlock => write!(f, stringify!(NamedCodeBlock)),
            Self::NamedCodeBlockBegin => write!(f, stringify!(NamedCodeBlockBegin)),
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
            Self::Root => write!(f, stringify!(Root)),
        }
    }
}
impl ExpectedChildren<TerminalKind, NonTerminalKind> for NonTerminalKind {
    fn expected_children(&self) -> ExpectedChildrenKinds<TerminalKind, NonTerminalKind> {
        match self {
            Self::Array => ExpectedChildrenKinds::Sequence(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::ArrayBegin),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::ArrayList),
                    attribute: ChildAttribute::Vec,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::ArrayEnd),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::ArrayBegin => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::LBracket),
                attribute: ChildAttribute::Normal,
            }]),
            Self::ArrayEnd => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::RBracket),
                attribute: ChildAttribute::Normal,
            }]),
            Self::ArrayList => ExpectedChildrenKinds::Recursion(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Value),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::ArrayOpt),
                    attribute: ChildAttribute::Optional,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::ArrayList),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::ArrayMarker => ExpectedChildrenKinds::Sequence(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::ArrayBegin),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::ArrayMarkerOpt),
                    attribute: ChildAttribute::Optional,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::ArrayEnd),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::ArrayMarkerOpt => ExpectedChildrenKinds::Option(&[ChildKind {
                kind: NodeKind::NonTerminal(NonTerminalKind::Integer),
                attribute: ChildAttribute::Normal,
            }]),
            Self::ArrayOpt => ExpectedChildrenKinds::Option(&[ChildKind {
                kind: NodeKind::NonTerminal(NonTerminalKind::Comma),
                attribute: ChildAttribute::Normal,
            }]),
            Self::At => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::At),
                attribute: ChildAttribute::Normal,
            }]),
            Self::Begin => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::LBrace),
                attribute: ChildAttribute::Normal,
            }]),
            Self::Bind => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::Bind),
                attribute: ChildAttribute::Normal,
            }]),
            Self::Binding => ExpectedChildrenKinds::Sequence(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Keys),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::BindingRhs),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::BindingRhs => ExpectedChildrenKinds::OneOf(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::ValueBinding),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::SectionBinding),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::TextBinding),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::Boolean => ExpectedChildrenKinds::OneOf(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::True),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::False),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::Code => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::Code),
                attribute: ChildAttribute::Normal,
            }]),
            Self::CodeBlock => ExpectedChildrenKinds::Sequence(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::CodeBlockDelimiter),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommon),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::CodeBlockDelimiter => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::CodeBlockDelimiter),
                attribute: ChildAttribute::Normal,
            }]),
            Self::CodeBlockLine => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::CodeBlockLine),
                attribute: ChildAttribute::Normal,
            }]),
            Self::CodeBlockTailCommon => ExpectedChildrenKinds::Sequence(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Newline),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommonList),
                    attribute: ChildAttribute::Vec,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommonOpt),
                    attribute: ChildAttribute::Optional,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::CodeBlockDelimiter),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::CodeBlockTailCommonList => ExpectedChildrenKinds::Recursion(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::CodeBlockLine),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Newline),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommonList),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::CodeBlockTailCommonOpt => ExpectedChildrenKinds::Option(&[ChildKind {
                kind: NodeKind::NonTerminal(NonTerminalKind::Ws),
                attribute: ChildAttribute::Normal,
            }]),
            Self::Comma => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::Comma),
                attribute: ChildAttribute::Normal,
            }]),
            Self::Continue => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::Esc),
                attribute: ChildAttribute::Normal,
            }]),
            Self::Dot => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::Dot),
                attribute: ChildAttribute::Normal,
            }]),
            Self::End => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::RBrace),
                attribute: ChildAttribute::Normal,
            }]),
            Self::Ext => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::Dollar),
                attribute: ChildAttribute::Normal,
            }]),
            Self::ExtensionNameSpace => ExpectedChildrenKinds::Sequence(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Ext),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Ident),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::False => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::False),
                attribute: ChildAttribute::Normal,
            }]),
            Self::Hole => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::Hole),
                attribute: ChildAttribute::Normal,
            }]),
            Self::Ident => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::Ident),
                attribute: ChildAttribute::Normal,
            }]),
            Self::InStr => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::InStr),
                attribute: ChildAttribute::Normal,
            }]),
            Self::Integer => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::Integer),
                attribute: ChildAttribute::Normal,
            }]),
            Self::Key => ExpectedChildrenKinds::Sequence(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::KeyBase),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::KeyOpt),
                    attribute: ChildAttribute::Optional,
                },
            ]),
            Self::KeyBase => ExpectedChildrenKinds::OneOf(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Ident),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::ExtensionNameSpace),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Str),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Integer),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::KeyOpt => ExpectedChildrenKinds::Option(&[ChildKind {
                kind: NodeKind::NonTerminal(NonTerminalKind::ArrayMarker),
                attribute: ChildAttribute::Normal,
            }]),
            Self::Keys => ExpectedChildrenKinds::Sequence(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Key),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::KeysList),
                    attribute: ChildAttribute::Vec,
                },
            ]),
            Self::KeysList => ExpectedChildrenKinds::Recursion(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Dot),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Key),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::KeysList),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::NamedCode => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::NamedCode),
                attribute: ChildAttribute::Normal,
            }]),
            Self::NamedCodeBlock => ExpectedChildrenKinds::Sequence(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::NamedCodeBlockBegin),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommon),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::NamedCodeBlockBegin => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::NamedCodeBlockBegin),
                attribute: ChildAttribute::Normal,
            }]),
            Self::Newline => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::Newline),
                attribute: ChildAttribute::Normal,
            }]),
            Self::Null => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::Null),
                attribute: ChildAttribute::Normal,
            }]),
            Self::Object => ExpectedChildrenKinds::Sequence(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Begin),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::ObjectList),
                    attribute: ChildAttribute::Vec,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::End),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::ObjectList => ExpectedChildrenKinds::Recursion(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Key),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Bind),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Value),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::ObjectOpt),
                    attribute: ChildAttribute::Optional,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::ObjectList),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::ObjectOpt => ExpectedChildrenKinds::Option(&[ChildKind {
                kind: NodeKind::NonTerminal(NonTerminalKind::Comma),
                attribute: ChildAttribute::Normal,
            }]),
            Self::Quote => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::Quote),
                attribute: ChildAttribute::Normal,
            }]),
            Self::Section => ExpectedChildrenKinds::Sequence(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::At),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Keys),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::SectionList),
                    attribute: ChildAttribute::Vec,
                },
            ]),
            Self::SectionBinding => ExpectedChildrenKinds::Sequence(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Begin),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Swon),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::End),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::SectionList => ExpectedChildrenKinds::Recursion(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Binding),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::SectionList),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::Str => ExpectedChildrenKinds::Sequence(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Quote),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::InStr),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Quote),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::StrContinues => ExpectedChildrenKinds::Sequence(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Str),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::StrContinuesList),
                    attribute: ChildAttribute::Vec,
                },
            ]),
            Self::StrContinuesList => ExpectedChildrenKinds::Recursion(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Continue),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Str),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::StrContinuesList),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::Swon => ExpectedChildrenKinds::Sequence(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::SwonList),
                    attribute: ChildAttribute::Vec,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::SwonList0),
                    attribute: ChildAttribute::Vec,
                },
            ]),
            Self::SwonList => ExpectedChildrenKinds::Recursion(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Binding),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::SwonList),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::SwonList0 => ExpectedChildrenKinds::Recursion(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Section),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::SwonList0),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::Text => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::Text),
                attribute: ChildAttribute::Normal,
            }]),
            Self::TextBinding => ExpectedChildrenKinds::Sequence(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::TextStart),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::TextBindingOpt),
                    attribute: ChildAttribute::Optional,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Text),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Newline),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::TextBindingOpt => ExpectedChildrenKinds::Option(&[ChildKind {
                kind: NodeKind::NonTerminal(NonTerminalKind::Ws),
                attribute: ChildAttribute::Clipped,
            }]),
            Self::TextStart => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::TextStart),
                attribute: ChildAttribute::Normal,
            }]),
            Self::True => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::True),
                attribute: ChildAttribute::Normal,
            }]),
            Self::TypedQuote => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::TypedQuote),
                attribute: ChildAttribute::Normal,
            }]),
            Self::TypedStr => ExpectedChildrenKinds::Sequence(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::TypedQuote),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::InStr),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Quote),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::Value => ExpectedChildrenKinds::OneOf(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Object),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Array),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Integer),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Boolean),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Null),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::StrContinues),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::TypedStr),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Hole),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::NamedCodeBlock),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::CodeBlock),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::NamedCode),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Code),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::ValueBinding => ExpectedChildrenKinds::Sequence(&[
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Bind),
                    attribute: ChildAttribute::Normal,
                },
                ChildKind {
                    kind: NodeKind::NonTerminal(NonTerminalKind::Value),
                    attribute: ChildAttribute::Normal,
                },
            ]),
            Self::Ws => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::Terminal(TerminalKind::Ws),
                attribute: ChildAttribute::Normal,
            }]),
            Self::Root => ExpectedChildrenKinds::Sequence(&[ChildKind {
                kind: NodeKind::NonTerminal(NonTerminalKind::Swon),
                attribute: ChildAttribute::Normal,
            }]),
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Array<T>(T);
#[allow(dead_code)]
impl<'a, N> Array<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Array(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_array_begin(&self, cursor: usize) -> Result<Option<(usize, Array<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::ArrayBegin))
            .map(|option| option.map(|(i, node)| (i, Array::new(node))))
    }
    pub fn find_array_list(&self, cursor: usize) -> Result<Option<(usize, Array<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::ArrayList))
            .map(|option| option.map(|(i, node)| (i, Array::new(node))))
    }
    pub fn find_array_end(&self, cursor: usize) -> Result<Option<(usize, Array<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::ArrayEnd))
            .map(|option| option.map(|(i, node)| (i, Array::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArrayBegin<T>(T);
#[allow(dead_code)]
impl<'a, N> ArrayBegin<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        ArrayBegin(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_l_bracket(&self, cursor: usize) -> Result<Option<(usize, ArrayBegin<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::LBracket))
            .map(|option| option.map(|(i, node)| (i, ArrayBegin::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArrayEnd<T>(T);
#[allow(dead_code)]
impl<'a, N> ArrayEnd<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        ArrayEnd(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_r_bracket(&self, cursor: usize) -> Result<Option<(usize, ArrayEnd<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::RBracket))
            .map(|option| option.map(|(i, node)| (i, ArrayEnd::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArrayList<T>(T);
#[allow(dead_code)]
impl<'a, N> ArrayList<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        ArrayList(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_value(&self, cursor: usize) -> Result<Option<(usize, ArrayList<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Value))
            .map(|option| option.map(|(i, node)| (i, ArrayList::new(node))))
    }
    pub fn find_array_opt(&self, cursor: usize) -> Result<Option<(usize, ArrayList<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::ArrayOpt))
            .map(|option| option.map(|(i, node)| (i, ArrayList::new(node))))
    }
    pub fn find_array_list(&self, cursor: usize) -> Result<Option<(usize, ArrayList<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::ArrayList))
            .map(|option| option.map(|(i, node)| (i, ArrayList::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArrayMarker<T>(T);
#[allow(dead_code)]
impl<'a, N> ArrayMarker<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        ArrayMarker(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_array_begin(&self, cursor: usize) -> Result<Option<(usize, ArrayMarker<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::ArrayBegin))
            .map(|option| option.map(|(i, node)| (i, ArrayMarker::new(node))))
    }
    pub fn find_array_marker_opt(
        &self,
        cursor: usize,
    ) -> Result<Option<(usize, ArrayMarker<N>)>, N> {
        self.0
            .find_child(
                cursor,
                NodeKind::NonTerminal(NonTerminalKind::ArrayMarkerOpt),
            )
            .map(|option| option.map(|(i, node)| (i, ArrayMarker::new(node))))
    }
    pub fn find_array_end(&self, cursor: usize) -> Result<Option<(usize, ArrayMarker<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::ArrayEnd))
            .map(|option| option.map(|(i, node)| (i, ArrayMarker::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArrayMarkerOpt<T>(T);
#[allow(dead_code)]
impl<'a, N> ArrayMarkerOpt<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        ArrayMarkerOpt(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_integer(&self, cursor: usize) -> Result<Option<(usize, ArrayMarkerOpt<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Integer))
            .map(|option| option.map(|(i, node)| (i, ArrayMarkerOpt::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ArrayOpt<T>(T);
#[allow(dead_code)]
impl<'a, N> ArrayOpt<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        ArrayOpt(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_comma(&self, cursor: usize) -> Result<Option<(usize, ArrayOpt<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Comma))
            .map(|option| option.map(|(i, node)| (i, ArrayOpt::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct At<T>(T);
#[allow(dead_code)]
impl<'a, N> At<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        At(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_at(&self, cursor: usize) -> Result<Option<(usize, At<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::At))
            .map(|option| option.map(|(i, node)| (i, At::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Begin<T>(T);
#[allow(dead_code)]
impl<'a, N> Begin<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Begin(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_l_brace(&self, cursor: usize) -> Result<Option<(usize, Begin<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::LBrace))
            .map(|option| option.map(|(i, node)| (i, Begin::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bind<T>(T);
#[allow(dead_code)]
impl<'a, N> Bind<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Bind(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_bind(&self, cursor: usize) -> Result<Option<(usize, Bind<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::Bind))
            .map(|option| option.map(|(i, node)| (i, Bind::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Binding<T>(T);
#[allow(dead_code)]
impl<'a, N> Binding<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Binding(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_keys(&self, cursor: usize) -> Result<Option<(usize, Binding<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Keys))
            .map(|option| option.map(|(i, node)| (i, Binding::new(node))))
    }
    pub fn find_binding_rhs(&self, cursor: usize) -> Result<Option<(usize, Binding<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::BindingRhs))
            .map(|option| option.map(|(i, node)| (i, Binding::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BindingRhs<T> {
    ValueBinding(ValueBinding<T>),
    SectionBinding(SectionBinding<T>),
    TextBinding(TextBinding<T>),
    Invalid(T),
}
#[allow(dead_code)]
impl<'a, N> BindingRhs<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        match node.kind() {
            NodeKind::NonTerminal(NonTerminalKind::ValueBinding) => {
                Self::ValueBinding(ValueBinding::new(node))
            }
            NodeKind::NonTerminal(NonTerminalKind::SectionBinding) => {
                Self::SectionBinding(SectionBinding::new(node))
            }
            NodeKind::NonTerminal(NonTerminalKind::TextBinding) => {
                Self::TextBinding(TextBinding::new(node))
            }
            _ => BindingRhs::Invalid(node),
        }
    }
    pub fn node(&self) -> &N {
        match self {
            Self::ValueBinding(node) => node.node(),
            Self::SectionBinding(node) => node.node(),
            Self::TextBinding(node) => node.node(),
            Self::Invalid(node) => node,
        }
    }
    pub fn node_mut(&mut self) -> &mut N {
        match self {
            Self::ValueBinding(node) => node.node_mut(),
            Self::SectionBinding(node) => node.node_mut(),
            Self::TextBinding(node) => node.node_mut(),
            Self::Invalid(node) => node,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Boolean<T> {
    True(True<T>),
    False(False<T>),
    Invalid(T),
}
#[allow(dead_code)]
impl<'a, N> Boolean<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        match node.kind() {
            NodeKind::NonTerminal(NonTerminalKind::True) => Self::True(True::new(node)),
            NodeKind::NonTerminal(NonTerminalKind::False) => Self::False(False::new(node)),
            _ => Boolean::Invalid(node),
        }
    }
    pub fn node(&self) -> &N {
        match self {
            Self::True(node) => node.node(),
            Self::False(node) => node.node(),
            Self::Invalid(node) => node,
        }
    }
    pub fn node_mut(&mut self) -> &mut N {
        match self {
            Self::True(node) => node.node_mut(),
            Self::False(node) => node.node_mut(),
            Self::Invalid(node) => node,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Code<T>(T);
#[allow(dead_code)]
impl<'a, N> Code<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Code(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_code(&self, cursor: usize) -> Result<Option<(usize, Code<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::Code))
            .map(|option| option.map(|(i, node)| (i, Code::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CodeBlock<T>(T);
#[allow(dead_code)]
impl<'a, N> CodeBlock<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        CodeBlock(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_code_block_delimiter(
        &self,
        cursor: usize,
    ) -> Result<Option<(usize, CodeBlock<N>)>, N> {
        self.0
            .find_child(
                cursor,
                NodeKind::NonTerminal(NonTerminalKind::CodeBlockDelimiter),
            )
            .map(|option| option.map(|(i, node)| (i, CodeBlock::new(node))))
    }
    pub fn find_code_block_tail_common(
        &self,
        cursor: usize,
    ) -> Result<Option<(usize, CodeBlock<N>)>, N> {
        self.0
            .find_child(
                cursor,
                NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommon),
            )
            .map(|option| option.map(|(i, node)| (i, CodeBlock::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CodeBlockDelimiter<T>(T);
#[allow(dead_code)]
impl<'a, N> CodeBlockDelimiter<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        CodeBlockDelimiter(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_code_block_delimiter(
        &self,
        cursor: usize,
    ) -> Result<Option<(usize, CodeBlockDelimiter<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::CodeBlockDelimiter))
            .map(|option| option.map(|(i, node)| (i, CodeBlockDelimiter::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CodeBlockLine<T>(T);
#[allow(dead_code)]
impl<'a, N> CodeBlockLine<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        CodeBlockLine(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_code_block_line(
        &self,
        cursor: usize,
    ) -> Result<Option<(usize, CodeBlockLine<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::CodeBlockLine))
            .map(|option| option.map(|(i, node)| (i, CodeBlockLine::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CodeBlockTailCommon<T>(T);
#[allow(dead_code)]
impl<'a, N> CodeBlockTailCommon<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        CodeBlockTailCommon(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_newline(
        &self,
        cursor: usize,
    ) -> Result<Option<(usize, CodeBlockTailCommon<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Newline))
            .map(|option| option.map(|(i, node)| (i, CodeBlockTailCommon::new(node))))
    }
    pub fn find_code_block_tail_common_list(
        &self,
        cursor: usize,
    ) -> Result<Option<(usize, CodeBlockTailCommon<N>)>, N> {
        self.0
            .find_child(
                cursor,
                NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommonList),
            )
            .map(|option| option.map(|(i, node)| (i, CodeBlockTailCommon::new(node))))
    }
    pub fn find_code_block_tail_common_opt(
        &self,
        cursor: usize,
    ) -> Result<Option<(usize, CodeBlockTailCommon<N>)>, N> {
        self.0
            .find_child(
                cursor,
                NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommonOpt),
            )
            .map(|option| option.map(|(i, node)| (i, CodeBlockTailCommon::new(node))))
    }
    pub fn find_code_block_delimiter(
        &self,
        cursor: usize,
    ) -> Result<Option<(usize, CodeBlockTailCommon<N>)>, N> {
        self.0
            .find_child(
                cursor,
                NodeKind::NonTerminal(NonTerminalKind::CodeBlockDelimiter),
            )
            .map(|option| option.map(|(i, node)| (i, CodeBlockTailCommon::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CodeBlockTailCommonList<T>(T);
#[allow(dead_code)]
impl<'a, N> CodeBlockTailCommonList<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        CodeBlockTailCommonList(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_code_block_line(
        &self,
        cursor: usize,
    ) -> Result<Option<(usize, CodeBlockTailCommonList<N>)>, N> {
        self.0
            .find_child(
                cursor,
                NodeKind::NonTerminal(NonTerminalKind::CodeBlockLine),
            )
            .map(|option| option.map(|(i, node)| (i, CodeBlockTailCommonList::new(node))))
    }
    pub fn find_newline(
        &self,
        cursor: usize,
    ) -> Result<Option<(usize, CodeBlockTailCommonList<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Newline))
            .map(|option| option.map(|(i, node)| (i, CodeBlockTailCommonList::new(node))))
    }
    pub fn find_code_block_tail_common_list(
        &self,
        cursor: usize,
    ) -> Result<Option<(usize, CodeBlockTailCommonList<N>)>, N> {
        self.0
            .find_child(
                cursor,
                NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommonList),
            )
            .map(|option| option.map(|(i, node)| (i, CodeBlockTailCommonList::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CodeBlockTailCommonOpt<T>(T);
#[allow(dead_code)]
impl<'a, N> CodeBlockTailCommonOpt<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        CodeBlockTailCommonOpt(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_ws(&self, cursor: usize) -> Result<Option<(usize, CodeBlockTailCommonOpt<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Ws))
            .map(|option| option.map(|(i, node)| (i, CodeBlockTailCommonOpt::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Comma<T>(T);
#[allow(dead_code)]
impl<'a, N> Comma<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Comma(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_comma(&self, cursor: usize) -> Result<Option<(usize, Comma<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::Comma))
            .map(|option| option.map(|(i, node)| (i, Comma::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Continue<T>(T);
#[allow(dead_code)]
impl<'a, N> Continue<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Continue(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_esc(&self, cursor: usize) -> Result<Option<(usize, Continue<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::Esc))
            .map(|option| option.map(|(i, node)| (i, Continue::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Dot<T>(T);
#[allow(dead_code)]
impl<'a, N> Dot<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Dot(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_dot(&self, cursor: usize) -> Result<Option<(usize, Dot<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::Dot))
            .map(|option| option.map(|(i, node)| (i, Dot::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct End<T>(T);
#[allow(dead_code)]
impl<'a, N> End<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        End(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_r_brace(&self, cursor: usize) -> Result<Option<(usize, End<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::RBrace))
            .map(|option| option.map(|(i, node)| (i, End::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ext<T>(T);
#[allow(dead_code)]
impl<'a, N> Ext<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Ext(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_dollar(&self, cursor: usize) -> Result<Option<(usize, Ext<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::Dollar))
            .map(|option| option.map(|(i, node)| (i, Ext::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ExtensionNameSpace<T>(T);
#[allow(dead_code)]
impl<'a, N> ExtensionNameSpace<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        ExtensionNameSpace(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_ext(&self, cursor: usize) -> Result<Option<(usize, ExtensionNameSpace<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Ext))
            .map(|option| option.map(|(i, node)| (i, ExtensionNameSpace::new(node))))
    }
    pub fn find_ident(&self, cursor: usize) -> Result<Option<(usize, ExtensionNameSpace<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Ident))
            .map(|option| option.map(|(i, node)| (i, ExtensionNameSpace::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct False<T>(T);
#[allow(dead_code)]
impl<'a, N> False<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        False(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_false(&self, cursor: usize) -> Result<Option<(usize, False<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::False))
            .map(|option| option.map(|(i, node)| (i, False::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Hole<T>(T);
#[allow(dead_code)]
impl<'a, N> Hole<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Hole(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_hole(&self, cursor: usize) -> Result<Option<(usize, Hole<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::Hole))
            .map(|option| option.map(|(i, node)| (i, Hole::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ident<T>(T);
#[allow(dead_code)]
impl<'a, N> Ident<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Ident(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_ident(&self, cursor: usize) -> Result<Option<(usize, Ident<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::Ident))
            .map(|option| option.map(|(i, node)| (i, Ident::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InStr<T>(T);
#[allow(dead_code)]
impl<'a, N> InStr<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        InStr(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_in_str(&self, cursor: usize) -> Result<Option<(usize, InStr<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::InStr))
            .map(|option| option.map(|(i, node)| (i, InStr::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Integer<T>(T);
#[allow(dead_code)]
impl<'a, N> Integer<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Integer(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_integer(&self, cursor: usize) -> Result<Option<(usize, Integer<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::Integer))
            .map(|option| option.map(|(i, node)| (i, Integer::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Key<T>(T);
#[allow(dead_code)]
impl<'a, N> Key<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Key(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_key_base(&self, cursor: usize) -> Result<Option<(usize, Key<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::KeyBase))
            .map(|option| option.map(|(i, node)| (i, Key::new(node))))
    }
    pub fn find_key_opt(&self, cursor: usize) -> Result<Option<(usize, Key<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::KeyOpt))
            .map(|option| option.map(|(i, node)| (i, Key::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyBase<T> {
    Ident(Ident<T>),
    ExtensionNameSpace(ExtensionNameSpace<T>),
    Str(Str<T>),
    Integer(Integer<T>),
    Invalid(T),
}
#[allow(dead_code)]
impl<'a, N> KeyBase<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        match node.kind() {
            NodeKind::NonTerminal(NonTerminalKind::Ident) => Self::Ident(Ident::new(node)),
            NodeKind::NonTerminal(NonTerminalKind::ExtensionNameSpace) => {
                Self::ExtensionNameSpace(ExtensionNameSpace::new(node))
            }
            NodeKind::NonTerminal(NonTerminalKind::Str) => Self::Str(Str::new(node)),
            NodeKind::NonTerminal(NonTerminalKind::Integer) => Self::Integer(Integer::new(node)),
            _ => KeyBase::Invalid(node),
        }
    }
    pub fn node(&self) -> &N {
        match self {
            Self::Ident(node) => node.node(),
            Self::ExtensionNameSpace(node) => node.node(),
            Self::Str(node) => node.node(),
            Self::Integer(node) => node.node(),
            Self::Invalid(node) => node,
        }
    }
    pub fn node_mut(&mut self) -> &mut N {
        match self {
            Self::Ident(node) => node.node_mut(),
            Self::ExtensionNameSpace(node) => node.node_mut(),
            Self::Str(node) => node.node_mut(),
            Self::Integer(node) => node.node_mut(),
            Self::Invalid(node) => node,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeyOpt<T>(T);
#[allow(dead_code)]
impl<'a, N> KeyOpt<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        KeyOpt(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_array_marker(&self, cursor: usize) -> Result<Option<(usize, KeyOpt<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::ArrayMarker))
            .map(|option| option.map(|(i, node)| (i, KeyOpt::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Keys<T>(T);
#[allow(dead_code)]
impl<'a, N> Keys<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Keys(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_key(&self, cursor: usize) -> Result<Option<(usize, Keys<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Key))
            .map(|option| option.map(|(i, node)| (i, Keys::new(node))))
    }
    pub fn find_keys_list(&self, cursor: usize) -> Result<Option<(usize, Keys<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::KeysList))
            .map(|option| option.map(|(i, node)| (i, Keys::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeysList<T>(T);
#[allow(dead_code)]
impl<'a, N> KeysList<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        KeysList(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_dot(&self, cursor: usize) -> Result<Option<(usize, KeysList<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Dot))
            .map(|option| option.map(|(i, node)| (i, KeysList::new(node))))
    }
    pub fn find_key(&self, cursor: usize) -> Result<Option<(usize, KeysList<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Key))
            .map(|option| option.map(|(i, node)| (i, KeysList::new(node))))
    }
    pub fn find_keys_list(&self, cursor: usize) -> Result<Option<(usize, KeysList<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::KeysList))
            .map(|option| option.map(|(i, node)| (i, KeysList::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NamedCode<T>(T);
#[allow(dead_code)]
impl<'a, N> NamedCode<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        NamedCode(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_named_code(&self, cursor: usize) -> Result<Option<(usize, NamedCode<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::NamedCode))
            .map(|option| option.map(|(i, node)| (i, NamedCode::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NamedCodeBlock<T>(T);
#[allow(dead_code)]
impl<'a, N> NamedCodeBlock<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        NamedCodeBlock(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_named_code_block_begin(
        &self,
        cursor: usize,
    ) -> Result<Option<(usize, NamedCodeBlock<N>)>, N> {
        self.0
            .find_child(
                cursor,
                NodeKind::NonTerminal(NonTerminalKind::NamedCodeBlockBegin),
            )
            .map(|option| option.map(|(i, node)| (i, NamedCodeBlock::new(node))))
    }
    pub fn find_code_block_tail_common(
        &self,
        cursor: usize,
    ) -> Result<Option<(usize, NamedCodeBlock<N>)>, N> {
        self.0
            .find_child(
                cursor,
                NodeKind::NonTerminal(NonTerminalKind::CodeBlockTailCommon),
            )
            .map(|option| option.map(|(i, node)| (i, NamedCodeBlock::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NamedCodeBlockBegin<T>(T);
#[allow(dead_code)]
impl<'a, N> NamedCodeBlockBegin<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        NamedCodeBlockBegin(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_named_code_block_begin(
        &self,
        cursor: usize,
    ) -> Result<Option<(usize, NamedCodeBlockBegin<N>)>, N> {
        self.0
            .find_child(
                cursor,
                NodeKind::Terminal(TerminalKind::NamedCodeBlockBegin),
            )
            .map(|option| option.map(|(i, node)| (i, NamedCodeBlockBegin::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Newline<T>(T);
#[allow(dead_code)]
impl<'a, N> Newline<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Newline(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_newline(&self, cursor: usize) -> Result<Option<(usize, Newline<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::Newline))
            .map(|option| option.map(|(i, node)| (i, Newline::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Null<T>(T);
#[allow(dead_code)]
impl<'a, N> Null<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Null(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_null(&self, cursor: usize) -> Result<Option<(usize, Null<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::Null))
            .map(|option| option.map(|(i, node)| (i, Null::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Object<T>(T);
#[allow(dead_code)]
impl<'a, N> Object<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Object(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_begin(&self, cursor: usize) -> Result<Option<(usize, Object<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Begin))
            .map(|option| option.map(|(i, node)| (i, Object::new(node))))
    }
    pub fn find_object_list(&self, cursor: usize) -> Result<Option<(usize, Object<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::ObjectList))
            .map(|option| option.map(|(i, node)| (i, Object::new(node))))
    }
    pub fn find_end(&self, cursor: usize) -> Result<Option<(usize, Object<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::End))
            .map(|option| option.map(|(i, node)| (i, Object::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectList<T>(T);
#[allow(dead_code)]
impl<'a, N> ObjectList<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        ObjectList(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_key(&self, cursor: usize) -> Result<Option<(usize, ObjectList<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Key))
            .map(|option| option.map(|(i, node)| (i, ObjectList::new(node))))
    }
    pub fn find_bind(&self, cursor: usize) -> Result<Option<(usize, ObjectList<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Bind))
            .map(|option| option.map(|(i, node)| (i, ObjectList::new(node))))
    }
    pub fn find_value(&self, cursor: usize) -> Result<Option<(usize, ObjectList<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Value))
            .map(|option| option.map(|(i, node)| (i, ObjectList::new(node))))
    }
    pub fn find_object_opt(&self, cursor: usize) -> Result<Option<(usize, ObjectList<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::ObjectOpt))
            .map(|option| option.map(|(i, node)| (i, ObjectList::new(node))))
    }
    pub fn find_object_list(&self, cursor: usize) -> Result<Option<(usize, ObjectList<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::ObjectList))
            .map(|option| option.map(|(i, node)| (i, ObjectList::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObjectOpt<T>(T);
#[allow(dead_code)]
impl<'a, N> ObjectOpt<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        ObjectOpt(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_comma(&self, cursor: usize) -> Result<Option<(usize, ObjectOpt<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Comma))
            .map(|option| option.map(|(i, node)| (i, ObjectOpt::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quote<T>(T);
#[allow(dead_code)]
impl<'a, N> Quote<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Quote(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_quote(&self, cursor: usize) -> Result<Option<(usize, Quote<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::Quote))
            .map(|option| option.map(|(i, node)| (i, Quote::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Section<T>(T);
#[allow(dead_code)]
impl<'a, N> Section<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Section(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_at(&self, cursor: usize) -> Result<Option<(usize, Section<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::At))
            .map(|option| option.map(|(i, node)| (i, Section::new(node))))
    }
    pub fn find_keys(&self, cursor: usize) -> Result<Option<(usize, Section<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Keys))
            .map(|option| option.map(|(i, node)| (i, Section::new(node))))
    }
    pub fn find_section_list(&self, cursor: usize) -> Result<Option<(usize, Section<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::SectionList))
            .map(|option| option.map(|(i, node)| (i, Section::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SectionBinding<T>(T);
#[allow(dead_code)]
impl<'a, N> SectionBinding<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        SectionBinding(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_begin(&self, cursor: usize) -> Result<Option<(usize, SectionBinding<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Begin))
            .map(|option| option.map(|(i, node)| (i, SectionBinding::new(node))))
    }
    pub fn find_swon(&self, cursor: usize) -> Result<Option<(usize, SectionBinding<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Swon))
            .map(|option| option.map(|(i, node)| (i, SectionBinding::new(node))))
    }
    pub fn find_end(&self, cursor: usize) -> Result<Option<(usize, SectionBinding<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::End))
            .map(|option| option.map(|(i, node)| (i, SectionBinding::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SectionList<T>(T);
#[allow(dead_code)]
impl<'a, N> SectionList<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        SectionList(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_binding(&self, cursor: usize) -> Result<Option<(usize, SectionList<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Binding))
            .map(|option| option.map(|(i, node)| (i, SectionList::new(node))))
    }
    pub fn find_section_list(&self, cursor: usize) -> Result<Option<(usize, SectionList<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::SectionList))
            .map(|option| option.map(|(i, node)| (i, SectionList::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Str<T>(T);
#[allow(dead_code)]
impl<'a, N> Str<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Str(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_quote(&self, cursor: usize) -> Result<Option<(usize, Str<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Quote))
            .map(|option| option.map(|(i, node)| (i, Str::new(node))))
    }
    pub fn find_in_str(&self, cursor: usize) -> Result<Option<(usize, Str<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::InStr))
            .map(|option| option.map(|(i, node)| (i, Str::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrContinues<T>(T);
#[allow(dead_code)]
impl<'a, N> StrContinues<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        StrContinues(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_str(&self, cursor: usize) -> Result<Option<(usize, StrContinues<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Str))
            .map(|option| option.map(|(i, node)| (i, StrContinues::new(node))))
    }
    pub fn find_str_continues_list(
        &self,
        cursor: usize,
    ) -> Result<Option<(usize, StrContinues<N>)>, N> {
        self.0
            .find_child(
                cursor,
                NodeKind::NonTerminal(NonTerminalKind::StrContinuesList),
            )
            .map(|option| option.map(|(i, node)| (i, StrContinues::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StrContinuesList<T>(T);
#[allow(dead_code)]
impl<'a, N> StrContinuesList<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        StrContinuesList(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_continue(&self, cursor: usize) -> Result<Option<(usize, StrContinuesList<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Continue))
            .map(|option| option.map(|(i, node)| (i, StrContinuesList::new(node))))
    }
    pub fn find_str(&self, cursor: usize) -> Result<Option<(usize, StrContinuesList<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Str))
            .map(|option| option.map(|(i, node)| (i, StrContinuesList::new(node))))
    }
    pub fn find_str_continues_list(
        &self,
        cursor: usize,
    ) -> Result<Option<(usize, StrContinuesList<N>)>, N> {
        self.0
            .find_child(
                cursor,
                NodeKind::NonTerminal(NonTerminalKind::StrContinuesList),
            )
            .map(|option| option.map(|(i, node)| (i, StrContinuesList::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Swon<T>(T);
#[allow(dead_code)]
impl<'a, N> Swon<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Swon(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_swon_list(&self, cursor: usize) -> Result<Option<(usize, Swon<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::SwonList))
            .map(|option| option.map(|(i, node)| (i, Swon::new(node))))
    }
    pub fn find_swon_list0(&self, cursor: usize) -> Result<Option<(usize, Swon<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::SwonList0))
            .map(|option| option.map(|(i, node)| (i, Swon::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SwonList<T>(T);
#[allow(dead_code)]
impl<'a, N> SwonList<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        SwonList(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_binding(&self, cursor: usize) -> Result<Option<(usize, SwonList<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Binding))
            .map(|option| option.map(|(i, node)| (i, SwonList::new(node))))
    }
    pub fn find_swon_list(&self, cursor: usize) -> Result<Option<(usize, SwonList<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::SwonList))
            .map(|option| option.map(|(i, node)| (i, SwonList::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SwonList0<T>(T);
#[allow(dead_code)]
impl<'a, N> SwonList0<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        SwonList0(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_section(&self, cursor: usize) -> Result<Option<(usize, SwonList0<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Section))
            .map(|option| option.map(|(i, node)| (i, SwonList0::new(node))))
    }
    pub fn find_swon_list0(&self, cursor: usize) -> Result<Option<(usize, SwonList0<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::SwonList0))
            .map(|option| option.map(|(i, node)| (i, SwonList0::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Text<T>(T);
#[allow(dead_code)]
impl<'a, N> Text<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Text(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_text(&self, cursor: usize) -> Result<Option<(usize, Text<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::Text))
            .map(|option| option.map(|(i, node)| (i, Text::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextBinding<T>(T);
#[allow(dead_code)]
impl<'a, N> TextBinding<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        TextBinding(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_text_start(&self, cursor: usize) -> Result<Option<(usize, TextBinding<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::TextStart))
            .map(|option| option.map(|(i, node)| (i, TextBinding::new(node))))
    }
    pub fn find_text_binding_opt(
        &self,
        cursor: usize,
    ) -> Result<Option<(usize, TextBinding<N>)>, N> {
        self.0
            .find_child(
                cursor,
                NodeKind::NonTerminal(NonTerminalKind::TextBindingOpt),
            )
            .map(|option| option.map(|(i, node)| (i, TextBinding::new(node))))
    }
    pub fn find_text(&self, cursor: usize) -> Result<Option<(usize, TextBinding<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Text))
            .map(|option| option.map(|(i, node)| (i, TextBinding::new(node))))
    }
    pub fn find_newline(&self, cursor: usize) -> Result<Option<(usize, TextBinding<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Newline))
            .map(|option| option.map(|(i, node)| (i, TextBinding::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextBindingOpt<T>(T);
#[allow(dead_code)]
impl<'a, N> TextBindingOpt<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        TextBindingOpt(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextStart<T>(T);
#[allow(dead_code)]
impl<'a, N> TextStart<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        TextStart(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_text_start(&self, cursor: usize) -> Result<Option<(usize, TextStart<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::TextStart))
            .map(|option| option.map(|(i, node)| (i, TextStart::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct True<T>(T);
#[allow(dead_code)]
impl<'a, N> True<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        True(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_true(&self, cursor: usize) -> Result<Option<(usize, True<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::True))
            .map(|option| option.map(|(i, node)| (i, True::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypedQuote<T>(T);
#[allow(dead_code)]
impl<'a, N> TypedQuote<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        TypedQuote(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_typed_quote(&self, cursor: usize) -> Result<Option<(usize, TypedQuote<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::TypedQuote))
            .map(|option| option.map(|(i, node)| (i, TypedQuote::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TypedStr<T>(T);
#[allow(dead_code)]
impl<'a, N> TypedStr<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        TypedStr(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_typed_quote(&self, cursor: usize) -> Result<Option<(usize, TypedStr<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::TypedQuote))
            .map(|option| option.map(|(i, node)| (i, TypedStr::new(node))))
    }
    pub fn find_in_str(&self, cursor: usize) -> Result<Option<(usize, TypedStr<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::InStr))
            .map(|option| option.map(|(i, node)| (i, TypedStr::new(node))))
    }
    pub fn find_quote(&self, cursor: usize) -> Result<Option<(usize, TypedStr<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Quote))
            .map(|option| option.map(|(i, node)| (i, TypedStr::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value<T> {
    Object(Object<T>),
    Array(Array<T>),
    Integer(Integer<T>),
    Boolean(Boolean<T>),
    Null(Null<T>),
    StrContinues(StrContinues<T>),
    TypedStr(TypedStr<T>),
    Hole(Hole<T>),
    NamedCodeBlock(NamedCodeBlock<T>),
    CodeBlock(CodeBlock<T>),
    NamedCode(NamedCode<T>),
    Code(Code<T>),
    Invalid(T),
}
#[allow(dead_code)]
impl<'a, N> Value<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        match node.kind() {
            NodeKind::NonTerminal(NonTerminalKind::Object) => Self::Object(Object::new(node)),
            NodeKind::NonTerminal(NonTerminalKind::Array) => Self::Array(Array::new(node)),
            NodeKind::NonTerminal(NonTerminalKind::Integer) => Self::Integer(Integer::new(node)),
            NodeKind::NonTerminal(NonTerminalKind::Boolean) => Self::Boolean(Boolean::new(node)),
            NodeKind::NonTerminal(NonTerminalKind::Null) => Self::Null(Null::new(node)),
            NodeKind::NonTerminal(NonTerminalKind::StrContinues) => {
                Self::StrContinues(StrContinues::new(node))
            }
            NodeKind::NonTerminal(NonTerminalKind::TypedStr) => Self::TypedStr(TypedStr::new(node)),
            NodeKind::NonTerminal(NonTerminalKind::Hole) => Self::Hole(Hole::new(node)),
            NodeKind::NonTerminal(NonTerminalKind::NamedCodeBlock) => {
                Self::NamedCodeBlock(NamedCodeBlock::new(node))
            }
            NodeKind::NonTerminal(NonTerminalKind::CodeBlock) => {
                Self::CodeBlock(CodeBlock::new(node))
            }
            NodeKind::NonTerminal(NonTerminalKind::NamedCode) => {
                Self::NamedCode(NamedCode::new(node))
            }
            NodeKind::NonTerminal(NonTerminalKind::Code) => Self::Code(Code::new(node)),
            _ => Value::Invalid(node),
        }
    }
    pub fn node(&self) -> &N {
        match self {
            Self::Object(node) => node.node(),
            Self::Array(node) => node.node(),
            Self::Integer(node) => node.node(),
            Self::Boolean(node) => node.node(),
            Self::Null(node) => node.node(),
            Self::StrContinues(node) => node.node(),
            Self::TypedStr(node) => node.node(),
            Self::Hole(node) => node.node(),
            Self::NamedCodeBlock(node) => node.node(),
            Self::CodeBlock(node) => node.node(),
            Self::NamedCode(node) => node.node(),
            Self::Code(node) => node.node(),
            Self::Invalid(node) => node,
        }
    }
    pub fn node_mut(&mut self) -> &mut N {
        match self {
            Self::Object(node) => node.node_mut(),
            Self::Array(node) => node.node_mut(),
            Self::Integer(node) => node.node_mut(),
            Self::Boolean(node) => node.node_mut(),
            Self::Null(node) => node.node_mut(),
            Self::StrContinues(node) => node.node_mut(),
            Self::TypedStr(node) => node.node_mut(),
            Self::Hole(node) => node.node_mut(),
            Self::NamedCodeBlock(node) => node.node_mut(),
            Self::CodeBlock(node) => node.node_mut(),
            Self::NamedCode(node) => node.node_mut(),
            Self::Code(node) => node.node_mut(),
            Self::Invalid(node) => node,
        }
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ValueBinding<T>(T);
#[allow(dead_code)]
impl<'a, N> ValueBinding<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        ValueBinding(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_bind(&self, cursor: usize) -> Result<Option<(usize, ValueBinding<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Bind))
            .map(|option| option.map(|(i, node)| (i, ValueBinding::new(node))))
    }
    pub fn find_value(&self, cursor: usize) -> Result<Option<(usize, ValueBinding<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::NonTerminal(NonTerminalKind::Value))
            .map(|option| option.map(|(i, node)| (i, ValueBinding::new(node))))
    }
}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ws<T>(T);
#[allow(dead_code)]
impl<'a, N> Ws<N>
where
    N: Node<'a, TerminalKind, NonTerminalKind>,
{
    pub fn new(node: N) -> Self {
        Ws(node)
    }
    pub fn node(&self) -> &N {
        &self.0
    }
    pub fn node_mut(&mut self) -> &mut N {
        &mut self.0
    }
    pub fn find_ws(&self, cursor: usize) -> Result<Option<(usize, Ws<N>)>, N> {
        self.0
            .find_child(cursor, NodeKind::Terminal(TerminalKind::Ws))
            .map(|option| option.map(|(i, node)| (i, Ws::new(node))))
    }
}
