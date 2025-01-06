use super::types::server::Command;
use crate::errors::websocket::WebSocketError;
use crate::errors::KromerError;
use std::collections::HashMap;
use std::io;
use surrealdb::Uuid;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub struct WsServer {
    sessions: HashMap<Uuid, mpsc::UnboundedSender<String>>,
    cmd_rx: mpsc::UnboundedReceiver<Command>,
}

impl WsServer {
    pub fn new() -> (Self, WsServerHandle) {
        let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();

        (
            Self {
                sessions: HashMap::new(),
                cmd_rx,
            },
            WsServerHandle { cmd_tx },
        )
    }

    // Send message to a specific connection
    pub async fn send_single_client_message(&self, target: Uuid, msg: impl Into<String>) {
        let msg = msg.into();
        if let Some(session) = self.sessions.get(&target) {
            let _ = session.send(msg.clone());
        }
    }

    // Register new session and assign a Uuid to this session
    async fn connect(&mut self, tx: mpsc::UnboundedSender<String>) -> Uuid {
        tracing::info!("Registering a new connection");

        // TODO: Notify Tracing/Logging Services?

        // register session with Uuid
        let id = Uuid::new_v4();
        self.sessions.insert(id, tx);

        // TODO: Increment Prometheus connection count?
        id
    }

    async fn disconnect(&mut self, conn_id: Uuid) {
        tracing::info!("Disconnecting a client");

        if self.sessions.remove(&conn_id).is_some() {
            tracing::info!("Found Session in Cache, removing");
        } else {
            tracing::error!("Could not find session in cache to remove");
        }

        // TODO: Decrement Prometheus connection count?
    }

    fn list_sessions(&mut self) -> Vec<Uuid> {
        self.sessions.keys().cloned().collect()
    }

    pub async fn run(mut self) -> io::Result<()> {
        while let Some(cmd) = self.cmd_rx.recv().await {
            match cmd {
                Command::Connect { conn_tx, res_tx } => {
                    let conn_id = self.connect(conn_tx).await;
                    let _ = res_tx.send(conn_id);
                }

                Command::Disconnect { conn } => {
                    self.disconnect(conn).await;
                }

                Command::List { res_tx } => {
                    let _ = res_tx.send(self.list_sessions());
                }

                Command::Message { msg, conn, res_tx } => {
                    self.send_single_client_message(conn, msg).await;
                    let _ = res_tx.send(());
                }
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct WsServerHandle {
    cmd_tx: mpsc::UnboundedSender<Command>,
}

impl WsServerHandle {
    // Register client message send, obtain connection ID
    pub async fn connect(
        &self,
        conn_tx: mpsc::UnboundedSender<String>,
    ) -> Result<Uuid, KromerError> {
        let (res_tx, res_rx) = oneshot::channel();

        if self
            .cmd_tx
            .send(Command::Connect { conn_tx, res_tx })
            .is_err()
        {
            return Err(KromerError::WebSocket(WebSocketError::HandshakeError));
        }

        // Match to see if there's errors to return
        match res_rx.await {
            Ok(result) => Ok(result),
            Err(_) => Err(KromerError::WebSocket(WebSocketError::HandshakeError)),
        }
    }

    pub async fn list_sessions(&self) -> Result<Vec<Uuid>, KromerError> {
        let (res_tx, res_rx) = oneshot::channel();

        if self.cmd_tx.send(Command::List { res_tx }).is_err() {
            return Err(KromerError::WebSocket(WebSocketError::ListSessions));
        }
        match res_rx.await {
            Ok(result) => Ok(result),
            Err(_) => Err(KromerError::WebSocket(WebSocketError::ListSessions)),
        }
    }

    pub async fn send_message(
        &self,
        conn: Uuid,
        msg: impl Into<String>,
    ) -> Result<(), KromerError> {
        let (res_tx, res_rx) = oneshot::channel();

        if self
            .cmd_tx
            .send(Command::Message {
                msg: msg.into(),
                conn,
                res_tx,
            })
            .is_err()
        {
            return Err(KromerError::WebSocket(WebSocketError::MessageSend));
        }

        res_rx
            .await
            .map_err(|_| KromerError::WebSocket(WebSocketError::MessageSend))
    }

    pub fn disconnect(&self, conn: Uuid) -> Result<(), KromerError> {
        self.cmd_tx
            .send(Command::Disconnect { conn })
            .map_err(|_| KromerError::WebSocket(WebSocketError::Disconnect))
    }
}
