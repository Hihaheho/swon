use ahash::AHashMap;
use swon_value::value::{Identifier, PathSegment};
use thiserror::Error;

use crate::prelude::*;

pub struct ValueVisitor {
    ident_handles: AHashMap<IdentHandle, Identifier>,
    key_handles: AHashMap<KeyHandle, PathSegment>,
    keys_handles: AHashMap<KeysHandle, Vec<KeyHandle>>,
    current_keys: Vec<KeyHandle>,
}

#[derive(Debug, Error)]
pub enum ValueVisitorError {
    #[error(transparent)]
    CstError(#[from] CstConstructError),
}

impl CstVisitor for ValueVisitor {
    type Error = ValueVisitorError;

    fn visit_keys(
        &mut self,
        handle: KeysHandle,
        view: KeysView,
        tree: &crate::Cst,
    ) -> Result<(), Self::Error> {
        assert_eq!(self.current_keys.len(), 0);
        self.visit_keys_super(handle, view, tree)?;
        self.keys_handles
            .insert(handle, std::mem::take(&mut self.current_keys));
        Ok(())
    }

    fn visit_key(
        &mut self,
        handle: KeyHandle,
        view: KeyView,
        tree: &crate::Cst,
    ) -> Result<(), Self::Error> {
        self.visit_key_super(handle, view, tree)?;
        Ok(())
    }

    fn visit_key_base(
        &mut self,
        handle: KeyBaseHandle,
        view: KeyBaseView,
        tree: &crate::Cst,
    ) -> Result<(), Self::Error> {
        self.visit_key_base_super(handle, view, tree)?;
        Ok(())
    }

    fn visit_ident(
        &mut self,
        handle: IdentHandle,
        view: IdentView,
        tree: &crate::Cst,
    ) -> Result<(), Self::Error> {
        self.visit_ident_super(handle, view, tree)
    }

    fn visit_ident_terminal(
        &mut self,
        terminal: Ident,
        data: crate::tree::TerminalData,
        tree: &crate::Cst,
    ) -> Result<(), Self::Error> {
        self.visit_ident_terminal_super(terminal, data, tree)
    }

    fn visit_key_opt(
        &mut self,
        handle: KeyOptHandle,
        view: ArrayMarkerHandle,
        tree: &crate::Cst,
    ) -> Result<(), Self::Error> {
        self.visit_key_opt_super(handle, view, tree)?;
        Ok(())
    }
}
