use thiserror::Error;

use crate::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeTarget {
    CstNodeId(CstNodeId),
    CommandNodeId(CommandNodeId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CommandNodeId(usize);

impl From<CommandNodeId> for NodeTarget {
    fn from(value: CommandNodeId) -> Self {
        NodeTarget::CommandNodeId(value)
    }
}

impl From<CstNodeId> for NodeTarget {
    fn from(value: CstNodeId) -> Self {
        NodeTarget::CstNodeId(value)
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Default)]
pub struct CstCommands {
    insert_num: usize,
    commands: Vec<Command>,
}

impl CstCommands {
    pub fn delete_node(&mut self, id: impl Into<NodeTarget>) {
        self.commands.push(Command::DeleteNode(id.into()));
    }

    pub fn delete_recursive(&mut self, id: impl Into<NodeTarget>) {
        self.commands.push(Command::DeleteRecursive(id.into()));
    }

    pub fn insert_dynamic_terminal(
        &mut self,
        kind: TerminalKind,
        data: impl Into<String>,
    ) -> CommandNodeId {
        self.commands.push(Command::InsertDynamicTerminal {
            kind,
            data: data.into(),
        });
        let id = CommandNodeId(self.insert_num);
        self.insert_num += 1;
        id
    }

    pub fn insert_node(&mut self, parent: impl Into<NodeTarget>, data: CstNode) -> CommandNodeId {
        self.commands.push(Command::Insert {
            parent: parent.into(),
            data,
        });
        let id = CommandNodeId(self.insert_num);
        self.insert_num += 1;
        id
    }

    pub fn update_node(&mut self, id: impl Into<NodeTarget>, data: CstNode) {
        self.commands.push(Command::Update {
            id: id.into(),
            data,
        });
    }

    /// Add nodes before the target child node
    pub fn add_nodes_before(
        &mut self,
        id: impl Into<NodeTarget>,
        before: impl Into<NodeTarget>,
        data: impl IntoIterator<Item = impl Into<NodeTarget>>,
    ) {
        self.commands.push(Command::AddNodesBefore {
            id: id.into(),
            before: before.into(),
            data: data.into_iter().map(|d| d.into()).collect(),
        });
    }
}

#[derive(Debug, Error)]
pub enum CommandApplyError {
    #[error("before node not found")]
    BeforeNodeNotFound { id: CstNodeId, before: CstNodeId },
}

impl CstCommands {
    pub fn apply_to(self, tree: &mut Cst) -> Result<(), CommandApplyError> {
        let mut inserted = vec![];
        let to_id = |inserted: &[CstNodeId], target: NodeTarget| -> CstNodeId {
            match target {
                NodeTarget::CstNodeId(id) => id,
                NodeTarget::CommandNodeId(id) => inserted[id.0],
            }
        };
        for command in self.commands.into_iter() {
            match command {
                Command::Insert { parent, data } => {
                    let id = tree.add_node_with_parent(data, to_id(&inserted, parent));
                    inserted.push(id);
                }
                Command::DeleteNode(node_target) => {
                    tree.delete_node(to_id(&inserted, node_target));
                }
                Command::DeleteRecursive(node_target) => {
                    tree.delete_node(to_id(&inserted, node_target));
                }
                Command::ChangeParent { id, parent } => {
                    tree.change_parent(to_id(&inserted, id), to_id(&inserted, parent));
                }
                Command::Update { id, data } => {
                    tree.update_node(to_id(&inserted, id), data);
                }
                Command::AddNodesBefore { id, before, data } => {
                    let mut children = tree.children(to_id(&inserted, id)).collect::<Vec<_>>();
                    let Some(before_index) =
                        children.iter().position(|c| to_id(&inserted, before) == *c)
                    else {
                        return Err(CommandApplyError::BeforeNodeNotFound {
                            id: to_id(&inserted, id),
                            before: to_id(&inserted, before),
                        });
                    };
                    children.splice(
                        before_index..before_index,
                        data.into_iter().map(|d| to_id(&inserted, d)),
                    );
                    tree.update_children(to_id(&inserted, id), children);
                }
                Command::InsertDynamicTerminal { kind, data } => {
                    let token_id = tree.insert_dynamic_terminal(data);
                    let node_id = tree.add_node(CstNode::Terminal {
                        kind,
                        data: TerminalData::Dynamic(token_id),
                    });
                    inserted.push(node_id);
                }
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    DeleteNode(NodeTarget),
    DeleteRecursive(NodeTarget),
    ChangeParent {
        id: NodeTarget,
        parent: NodeTarget,
    },
    Insert {
        parent: NodeTarget,
        data: CstNode,
    },
    Update {
        id: NodeTarget,
        data: CstNode,
    },
    AddNodesBefore {
        id: NodeTarget,
        before: NodeTarget,
        data: Vec<NodeTarget>,
    },
    InsertDynamicTerminal {
        kind: TerminalKind,
        data: String,
    },
}
