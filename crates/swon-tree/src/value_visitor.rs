use ahash::AHashMap;
use swon_value::value::PathSegment;
use thiserror::Error;

use crate::{
    CstConstructError,
    nodes::{KeyHandle, KeysHandle},
    tree::NonTerminalHandle as _,
    visitor::{CstHandleSuper as _, CstVisitor},
};

pub struct ValueVisitor {
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
        view: crate::nodes::KeysView,
        tree: &crate::Cst,
    ) -> Result<(), Self::Error> {
        assert_eq!(self.current_keys.len(), 0);
        self.visit_keys_super(view, tree);
        self.keys_handles
            .insert(handle, std::mem::take(&mut self.current_keys));
        Ok(())
    }

    fn visit_key(
        &mut self,
        handle: KeyHandle,
        view: crate::nodes::KeyView,
        tree: &crate::Cst,
    ) -> Result<(), Self::Error> {
        let key_base = view.key_base.get_view(tree)?;
        // key_
        Ok(())
    }
}
