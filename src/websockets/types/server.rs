use surrealdb::Uuid;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub enum Command {
    Connect {
        conn_tx: mpsc::UnboundedSender<String>,
        res_tx: oneshot::Sender<Uuid>,
        session_uuid: surrealdb::Uuid,
    },

    Disconnect {
        conn: Uuid,
    },

    List {
        res_tx: oneshot::Sender<Vec<Uuid>>,
    },

    ChannelMessage {
        msg: String,
        conn: Uuid,
        res_tx: oneshot::Sender<()>,
    },

    SessionMessage {
        msg: String,
        session: surrealdb::Uuid,
        res_tx: oneshot::Sender<()>,
    },
}
