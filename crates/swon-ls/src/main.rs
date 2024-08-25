mod semantic_tokens;

use lsp_types::request::DocumentHighlightRequest;
use lsp_types::{
    InitializeParams, SemanticTokensFullOptions, SemanticTokensLegend, SemanticTokensOptions,
    ServerCapabilities,
};

use lsp_server::{
    Connection, ErrorCode, ExtractError, Message, Notification, Request, Response, ResponseError,
};

fn main() -> anyhow::Result<()> {
    let (connection, io_threads) = Connection::stdio();

    let server_capabilities = serde_json::to_value(&ServerCapabilities {
        semantic_tokens_provider: Some(
            SemanticTokensOptions {
                work_done_progress_options: Default::default(),
                legend: SemanticTokensLegend {
                    token_types: vec![],
                    token_modifiers: vec![],
                },
                range: Some(true),
                full: Some(SemanticTokensFullOptions::Delta { delta: Some(true) }),
            }
            .into(),
        ),
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

    let mut context = ServerContext { connection, params };
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
                    // self.handle_request::<DocumentHighlightRequest>(req, Self::document_highlight)?;
                }
                Message::Response(resp) => {
                    eprintln!("got response: {resp:?}");
                }
                Message::Notification(not) => {
                    eprintln!("got notification: {not:?}");
                }
            }
        }
    }

    fn send_response(&self, resp: Response) -> anyhow::Result<()> {
        Ok(self.connection.sender.send(Message::Response(resp))?)
    }

    fn handle_request<R>(
        &mut self,
        req: Request,
        f: impl FnOnce(&mut Self, R::Params) -> anyhow::Result<R::Result>,
    ) -> anyhow::Result<Option<()>>
    where
        R: lsp_types::request::Request,
        R::Params: serde::de::DeserializeOwned,
        R::Result: serde::ser::Serialize,
    {
        let (id, params) = match req.extract(R::METHOD) {
            Ok(it) => it,
            Err(e) => match e {
                ExtractError::MethodMismatch(_) => return Ok(None),
                ExtractError::JsonError { method, error } => {
                    return Err(anyhow::anyhow!(
                        "failed to parse request: {method}: {error}"
                    ))
                }
            },
        };
        let resp = match f(self, params) {
            Ok(result) => Response {
                id,
                result: Some(serde_json::to_value(&result)?),
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
        Ok(Some(()))
    }
}
