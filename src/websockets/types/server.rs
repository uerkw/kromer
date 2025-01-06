use surrealdb::Uuid;
use tokio::sync::{mpsc, oneshot};

#[derive(Debug)]
pub enum Command {
    Connect {
        conn_tx: mpsc::UnboundedSender<String>,
        res_tx: oneshot::Sender<Uuid>,
    },

    Disconnect {
        conn: Uuid,
    },

    List {
        res_tx: oneshot::Sender<Vec<Uuid>>,
    },

    Message {
        msg: String,
        conn: Uuid,
        res_tx: oneshot::Sender<()>,
    },
}
