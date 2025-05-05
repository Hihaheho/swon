// Comment out the module since it's empty
// mod semantic_tokens;

use std::collections::HashMap;

use lsp_types::request::SemanticTokensFullRequest;
use lsp_types::{
    InitializeParams, SemanticTokensFullOptions, SemanticTokensLegend, SemanticTokensOptions,
    SemanticTokensResult, ServerCapabilities,
};
use swon_editor_support::semantic_tokens;
use swon_parol::{parser, Cst};

use lsp_server::{
    Connection, ErrorCode, ExtractError, Message, Notification, Request, Response, ResponseError,
};

fn main() -> anyhow::Result<()> {
    let (connection, io_threads) = Connection::stdio();

    // Get the legend from the support crate
    let legend = semantic_tokens::get_legend();

    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        semantic_tokens_provider: Some(
            SemanticTokensOptions {
                work_done_progress_options: Default::default(),
                legend: legend.clone(), // Use the actual legend
                range: Some(false),     // Let's start with full document support only
                full: Some(SemanticTokensFullOptions::Delta { delta: Some(false) }), // No delta support yet
            }
            .into(),
        ),
        // Add textDocumentSync capability if not already present, needed for tracking documents
        text_document_sync: Some(lsp_types::TextDocumentSyncCapability::Kind(
            lsp_types::TextDocumentSyncKind::FULL, // Or INCREMENTAL if handled
        )),
        ..Default::default()
    })
    .unwrap();
    let params = match connection.initialize(server_capabilities) {
        Ok(it) => it,
        Err(e) => {
            if e.channel_is_disconnected() {
                io_threads.join()?;
            }
            return Err(e.into());
        }
    };
    let params: InitializeParams = serde_json::from_value(params)?;

    let mut context = ServerContext {
        connection,
        params,
        documents: HashMap::new(), // Initialize documents map
        legend,                    // Store legend in context
    };
    context.run()?;

    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("shutting down server");
    Ok(())
}

pub struct ServerContext {
    connection: Connection,
    #[allow(dead_code)]
    params: InitializeParams,
    documents: HashMap<String, (Option<Cst>, String)>, // Store (CST, Content) by document URI
    legend: SemanticTokensLegend,                      // Store the legend
}

pub enum Event {
    Request(Request),
    Response(Response),
    Notification(Notification),
}

impl ServerContext {
    fn run(&mut self) -> anyhow::Result<()> {
        loop {
            let msg = self.connection.receiver.recv()?;
            eprintln!("got msg: {msg:?}");
            match msg {
                Message::Request(req) => {
                    if self.connection.handle_shutdown(&req)? {
                        return Ok(());
                    }
                    // Handle Semantic Tokens request
                    if self
                        .handle_request::<SemanticTokensFullRequest>(
                            req.clone(), // Clone req as handle_request consumes it
                            Self::handle_semantic_tokens_full,
                        )?
                        .is_some()
                    {
                        continue; // Request was handled
                    }

                    // Placeholder for other request handlers
                    eprintln!("unhandled request: {:?}", req);
                    let resp = Response {
                        id: req.id,
                        result: None,
                        error: Some(ResponseError {
                            code: ErrorCode::MethodNotFound as i32,
                            message: format!("method not supported: {}", req.method),
                            data: None,
                        }),
                    };
                    self.send_response(resp)?;
                }
                Message::Response(resp) => {
                    eprintln!("got response: {resp:?}");
                }
                Message::Notification(not) => {
                    eprintln!("got notification: {not:?}");
                    // Handle notification for document updates
                    if not.method == "textDocument/didOpen" {
                        if let Ok(params) = serde_json::from_value::<
                            lsp_types::DidOpenTextDocumentParams,
                        >(not.params)
                        {
                            let uri = params.text_document.uri.to_string();
                            let text = params.text_document.text;

                            // Try to parse the document to get the CST
                            let cst = match parse_document(&text) {
                                Ok(cst) => Some(cst),
                                Err(e) => {
                                    eprintln!("Failed to parse document: {}", e);
                                    None
                                }
                            };

                            // Store document
                            self.documents.insert(uri, (cst, text));
                        }
                    } else if not.method == "textDocument/didChange" {
                        if let Ok(params) = serde_json::from_value::<
                            lsp_types::DidChangeTextDocumentParams,
                        >(not.params)
                        {
                            let uri = params.text_document.uri.to_string();

                            // For FULL sync, we just get the full content from the last change
                            if let Some(last_change) = params.content_changes.last() {
                                let text = last_change.text.clone();
                                // Try to parse the document to get the CST
                                let cst = match parse_document(&text) {
                                    Ok(cst) => Some(cst),
                                    Err(e) => {
                                        eprintln!("Failed to parse document: {}", e);
                                        None
                                    }
                                };

                                // Update document
                                self.documents.insert(uri, (cst, text));
                            }
                        }
                    }
                }
            }
        }
    }

    // Handler for textDocument/semanticTokens/full
    fn handle_semantic_tokens_full(
        &mut self,
        params: lsp_types::SemanticTokensParams,
    ) -> anyhow::Result<Option<Option<SemanticTokensResult>>> {
        let uri = params.text_document.uri.to_string();

        eprintln!("Handling semantic tokens full request for: {}", uri);

        // Lookup document in our store
        if let Some((cst_opt, text)) = self.documents.get(&uri) {
            if let Some(cst) = cst_opt {
                // Generate tokens if we have a CST
                match semantic_tokens::semantic_tokens(text, cst, &self.legend) {
                    Some(tokens) => Ok(Some(Some(SemanticTokensResult::Tokens(tokens)))),
                    None => Ok(Some(None)),
                }
            } else {
                eprintln!("Document has no valid CST for {}", uri);
                Ok(Some(None))
            }
        } else {
            eprintln!("Document not found in store: {}", uri);
            Ok(Some(None))
        }
    }

    fn send_response(&self, resp: Response) -> anyhow::Result<()> {
        Ok(self.connection.sender.send(Message::Response(resp))?)
    }

    // Generic request handler
    fn handle_request<R>(
        &mut self,
        req: Request,
        f: impl FnOnce(&mut Self, R::Params) -> anyhow::Result<Option<R::Result>>,
    ) -> anyhow::Result<Option<()>>
    // Returns Some(()) if handled, None otherwise
    where
        R: lsp_types::request::Request,
        R::Params: serde::de::DeserializeOwned,
        R::Result: serde::ser::Serialize,
    {
        let (id, params) = match req.extract(R::METHOD) {
            Ok(it) => it,
            Err(ExtractError::MethodMismatch(_)) => return Ok(None), // Not this request type
            Err(ExtractError::JsonError { method, error }) => {
                return Err(anyhow::anyhow!(
                    "failed to parse request: {method}: {error}"
                ))
            }
        };

        let result_opt = f(self, params);

        let resp = match result_opt {
            Ok(Some(result)) => Response {
                id,
                result: Some(serde_json::to_value(&result)?),
                error: None,
            },
            Ok(None) => Response {
                id,
                result: Some(serde_json::Value::Null),
                error: None,
            },
            Err(e) => Response {
                id,
                result: None,
                error: Some(ResponseError {
                    code: ErrorCode::InternalError as i32,
                    message: e.to_string(),
                    data: None,
                }),
            },
        };
        self.send_response(resp)?;
        Ok(Some(())) // Signal that the request was handled
    }
}

// Helper function to parse a document and return a CST
fn parse_document(text: &str) -> anyhow::Result<Cst> {
    use swon_parol::nodes::{NonTerminalKind, TerminalKind};
    use swon_parol::tree::CstBuilder;
    use swon_parol::TreeConstruct;

    let mut actions = swon_parol::grammar::Grammar::new();
    let mut tree_builder = CstBuilder::<TerminalKind, NonTerminalKind>::new();
    let _ = parser::parse_into(text, &mut tree_builder, "document.swon", &mut actions);
    let cst = tree_builder.build()?;
    Ok(cst)
}
