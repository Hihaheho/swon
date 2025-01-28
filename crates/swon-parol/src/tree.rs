use parol_runtime::{
    parser::parse_tree_type::{
        AstNode, ExpectedChildren, ExpectedChildrenKinds, NodeKind, NonTerminalEnum, TerminalEnum,
    },
    Token,
};

#[derive(Debug, Clone, Copy)]
/// A parse tree that is associated with a grammar.
pub enum SynTree2<T, Nt> {
    /// A terminal node.
    Terminal(SynTreeTerminal<T>),
    /// A non-terminal node.
    NonTerminal(SynTreeNonTerminal<Nt>),
}

impl<T: Copy, Nt: Copy> SynTree2<T, Nt> {
    /// The kind of the node.
    pub fn kind(&self) -> NodeKind<T, Nt> {
        match self {
            SynTree2::Terminal(data) => NodeKind::Terminal(data.kind),
            SynTree2::NonTerminal(data) => NodeKind::NonTerminal(data.kind),
        }
    }
}

impl<T, Nt> std::fmt::Display for SynTree2<T, Nt>
where
    T: std::fmt::Display,
    Nt: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SynTree2::Terminal(t) => write!(f, "{}", t),
            SynTree2::NonTerminal(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Debug, Clone, Copy)]
/// A terminal node.
pub struct SynTreeTerminal<T> {
    /// The kind of the terminal.
    pub kind: T,
    /// The data of the terminal.
    pub data: TerminalData,
}

impl<T> std::fmt::Display for SynTreeTerminal<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

#[derive(Debug, Clone, Copy)]
/// A span that is only valid within the context of the input text.
pub struct InputSpan {
    /// The start of the span.
    pub start: u32,
    /// The end of the span.
    pub end: u32,
}

#[derive(Debug, Clone, Copy)]
/// A dynamic token id that provided by the user land.
pub struct DynamicTokenId(pub u32);

#[derive(Debug, Clone, Copy)]
/// The data of the terminal.
pub enum TerminalData {
    /// A terminal that is associated with an input span.
    Input(InputSpan),
    /// A terminal that is associated with a dynamic token id.
    Dynamic(DynamicTokenId),
}

#[derive(Debug, Clone, Copy)]
/// A non-terminal node.
pub struct SynTreeNonTerminal<Nt> {
    /// The kind of the non-terminal.
    pub kind: Nt,
}

impl<Nt> std::fmt::Display for SynTreeNonTerminal<Nt>
where
    Nt: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.kind)
    }
}

impl<'t, T, Nt> AstNode<'t> for SynTree2<T, Nt>
where
    T: TerminalEnum,
    Nt: NonTerminalEnum,
{
    fn from_token(token: &Token<'t>) -> Self {
        SynTree2::Terminal(SynTreeTerminal {
            kind: T::from_terminal_index(token.token_type),
            data: TerminalData::Input(InputSpan {
                start: token.location.start,
                end: token.location.end,
            }),
        })
    }
    fn from_non_terminal(name: &'static str) -> Self {
        SynTree2::NonTerminal(SynTreeNonTerminal {
            kind: Nt::from_non_terminal_name(name),
        })
    }
}

impl<T, Nt> ExpectedChildren<T, Nt> for SynTree2<T, Nt>
where
    Nt: ExpectedChildren<T, Nt>,
{
    fn expected_children(&self) -> ExpectedChildrenKinds<T, Nt> {
        match self {
            SynTree2::Terminal(_) => ExpectedChildrenKinds::Sequence(&[]),
            SynTree2::NonTerminal(nt) => nt.kind.expected_children(),
        }
    }
}
