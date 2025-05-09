use thisisplural::Plural;

use crate::{Cst, CstNode, tree::CstNodeId};

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
}

impl CstCommands {
    pub fn apply_to(&self, tree: &mut Cst) {
        let mut inserted = vec![];
        let to_id = |inserted: &[CstNodeId], target: &NodeTarget| -> CstNodeId {
            match target {
                NodeTarget::CstNodeId(id) => *id,
                NodeTarget::CommandNodeId(id) => inserted[id.0],
            }
        };
        for command in self.commands.iter() {
            match command {
                Command::Insert { parent, data } => {
                    let id = tree.add_node_with_parent(*data, to_id(&inserted, parent));
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
                    tree.update_node(to_id(&inserted, id), *data);
                }
            }
        }
    }
}
#[derive(Debug, Clone, Plural)]
pub struct CstActions(pub Vec<Command>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Command {
    DeleteNode(NodeTarget),
    DeleteRecursive(NodeTarget),
    ChangeParent { id: NodeTarget, parent: NodeTarget },
    Insert { parent: NodeTarget, data: CstNode },
    Update { id: NodeTarget, data: CstNode },
}
