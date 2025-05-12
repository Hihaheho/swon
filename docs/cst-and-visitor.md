# CST and Visitor

## Overview

The `swon-tree` crate provides a Concrete Syntax Tree (CST) representation for Swon documents, along with a powerful Visitor pattern API for traversing and analyzing the tree. This structure is generated automatically from the Swon grammar definition (`swon.parol`) using the `swon-parol-gen` tool, ensuring that the API always stays synchronized with the language grammar.

This document focuses on the design of the visitor API, the generated code structure, and what you can achieve using these tools.

## Generated Code (`swon-tree`)

The `swon-parol-gen` crate generates two key files within `swon-tree/src`:

1. `nodes.rs`: Defines the structural representation of the CST nodes.
2. `visitor.rs`: Defines the `CstVisitor` trait and related components.

### Node Handles and Views (`nodes.rs`)

For each element defined in the Swon grammar (`swon.parol`), `swon-parol-gen` generates corresponding types in `nodes.rs`:

* **Terminal Nodes:** For each terminal symbol (like `Ident`, `Integer`, `LBrace`), a simple struct wrapping a `CstNodeId` is generated (e.g., `struct Ident(CstNodeId)`). These structs implement the `TerminalHandle` trait, providing access to the node's ID and `TerminalKind`. The actual text content associated with the terminal can be retrieved using the `CstFacade` (like the `Cst` itself) and the node ID.
* **Non-Terminal Nodes:** For each non-terminal rule (like `Array`, `Binding`, `Value`), two types are generated:
  * **Handle:** A struct named `<RuleName>Handle` (e.g., `ArrayHandle(CstNodeId)`). This is a lightweight identifier for the specific node in the tree and implements the `NonTerminalHandle` trait.
  * **View:** A struct or enum named `<RuleName>View` (e.g., `ArrayView`, `ValueView`). This provides structured access to the children of the non-terminal node, typically containing Handles for those children. The structure of the View directly reflects the grammar rule:
    * **Sequences:** The View is a struct with fields for each element in the sequence (e.g., `BindingView { keys: KeysHandle, binding_rhs: BindingRhsHandle }`).
    * **Choices (Alternation):** The View is an enum with variants for each possible choice (e.g., `ValueView::Object(ObjectHandle)`).
    * **Repetitions/Lists (Recursion):** The View is often a struct containing a handle to the item and a handle to the next element in the list (e.g., `ArrayListView { value: ValueHandle, array_opt: ArrayOptHandle, array_list: ArrayListHandle }`). These views often implement the `RecursiveView` trait, providing helper methods like `get_all_with_visit` for easy iteration over list items.
    * **Options:** The corresponding `Handle`'s associated `View` type is an `Option` wrapping the handle of the optional child (e.g., `ArrayOptHandle` has `type View = Option<CommaHandle>`).

This generation ensures that the Rust types accurately represent the Swon grammar structure.

### Visitor Traits (`visitor.rs`)

The `swon-parol-gen` tool also generates the core visitor traits:

* **`CstVisitor<F: CstFacade>`:** This is the main trait users implement to traverse the CST. It contains methods for visiting each node type:
  * `visit_<RuleName>(&mut self, handle: <RuleName>Handle, view: <RuleName>View, tree: &F)`: Called when entering a non-terminal node. Receives the node's `Handle` and its `View` (providing access to children).
  * `visit_<TerminalName>_terminal(&mut self, terminal: <TerminalName>, data: TerminalData, tree: &F)`: Called when visiting a terminal node. Receives the terminal's `Handle` and associated data.
  * Generic methods like `visit_non_terminal`, `visit_terminal`, and `visit_non_terminal_close`.
* **`CstVisitorSuper<F: CstFacade, E>`:** This trait defines the default traversal logic. The default implementations of the `visit_*` methods in `CstVisitor` call corresponding `visit_*_super` methods defined in this trait. This allows users to override specific `visit_*` methods in their `CstVisitor` implementation without needing to manually implement the logic for descending into children.

## API Design and Capabilities

The visitor pattern implemented in `swon-tree` allows for robust and type-safe traversal of the Swon CST.

### How to Use

1. **Create a struct:** Define your custom visitor struct.
2. **Implement `CstVisitor<Cst>`:** Implement the trait for your struct. You only need to override the `visit_*` methods for the node types you are interested in. The default methods handle the tree traversal.
3. **Implement `CstVisitorSuper` (implicitly):** By implementing `CstVisitor`, your struct implicitly gains the default traversal logic defined in `CstVisitorSuper`.
4. **Run the visitor:** Instantiate your visitor and call the appropriate traversal method on the `Cst` object (e.g., using methods provided by `Cst` or helper functions that take a visitor).

```rust, ignore
use swon_tree::{Cst, CstVisitor, CstVisitorSuper, nodes::*, tree::{CstFacade, TerminalData, NonTerminalData, CstNodeId}, node_kind::{TerminalKind, NonTerminalKind}, CstConstructError};

struct MyVisitor {
    // Visitor state, e.g., collected data
    ident_count: usize,
}

impl CstVisitor<Cst> for MyVisitor {
    type Error = std::convert::Infallible; // Or your custom error type

    // Override visit method for Ident terminal
    fn visit_ident_terminal(
        &mut self,
        terminal: Ident, // Handle for the Ident terminal
        data: TerminalData,
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        self.ident_count += 1;
        println!("Found identifier: {}", tree.text(terminal.node_id()));
        // Call super to maintain default behavior (if any)
        self.visit_ident_terminal_super(terminal, data, tree)
    }

    // Override visit method for Array non-terminal
    fn visit_array(
        &mut self,
        handle: ArrayHandle, // Handle for the Array node
        view: ArrayView,     // View providing access to children (ArrayBegin, ArrayList, ArrayEnd)
        tree: &Cst,
    ) -> Result<(), Self::Error> {
        println!("Entering an array");
        // Default behavior: visit children. Call the _super method.
        self.visit_array_super(handle, view, tree)
        // If you wanted to *stop* traversal into arrays, you would *not* call visit_array_super.
    }

    // You don't need to implement methods for nodes you don't care about.
    // The default implementation from CstVisitorSuper will handle traversal.
}

fn visit(cst: &Cst) -> Result<(), CstConstructError> {
    let mut visitor = MyVisitor { ident_count: 0 };
    cst.root().visit(&cst, &mut visitor).unwrap(); // Start traversal from the root
    println!("Total identifiers found: {}", visitor.ident_count);
    Ok(())
}
```

### What Can Be Achieved?

* **Code Analysis:** Count occurrences of specific nodes, check for specific patterns, validate semantic rules.
* **Data Extraction:** Extract all key-value pairs, string literals, or specific sections from a Swon document.
* **Transformation:** Convert the CST into an Abstract Syntax Tree (AST), JSON, or any other format.
* **Code Generation:** Generate code or configuration based on the Swon input.
* **Linting/Formatting:** Implement custom linting rules or code formatting logic by analyzing node positions and content.

The combination of auto-generated, type-safe node access (`Handle` and `View`) and the flexible `CstVisitor` trait provides a powerful foundation for interacting with Swon CSTs.
