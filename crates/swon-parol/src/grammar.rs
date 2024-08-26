use crate::grammar_trait::{GrammarTrait, Swon};
#[allow(unused_imports)]
use parol_runtime::{Result, Token};
use std::fmt::{Debug, Display, Error, Formatter};

///
/// Data structure that implements the semantic actions for our Swon grammar
/// !Change this type as needed!
///
#[derive(Debug, Default)]
pub struct Grammar<'t> {
    pub swon: Option<Swon<'t>>,
}

impl Grammar<'_> {
    pub fn new() -> Self {
        Grammar::default()
    }
}

impl Display for Swon<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        write!(f, "{:?}", self)
    }
}

impl Display for Grammar<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::result::Result<(), Error> {
        match &self.swon {
            Some(swon) => writeln!(f, "{}", swon),
            None => write!(f, "No parse result"),
        }
    }
}

impl<'t> GrammarTrait<'t> for Grammar<'t> {
    // !Adjust your implementation as needed!

    /// Semantic action for non-terminal 'Swon'
    fn swon(&mut self, arg: &Swon<'t>) -> Result<()> {
        self.swon = Some(arg.clone());
        Ok(())
    }

    fn on_comment_parsed(&mut self, _token: Token<'t>) {
        // TODO
    }
}
