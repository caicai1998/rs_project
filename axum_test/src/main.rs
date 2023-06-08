mod handle;

use axum::{
    routing::{get, post},
    Extension, Json, Router,
};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc::UnboundedSender, oneshot::Sender};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct User {
    name: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Method {
    Create,
    Query,
    Del,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Body {
    user: User,
    method: Method,
}
#[derive(Debug)]
pub(crate) struct SenderBody {
    user: User,
    method: Method,
    sender: Sender<Reponse>,
}

#[derive(Debug, Serialize)]
pub(crate) enum Reponse {
    Ok,
    Err(String),
    User(User),
}

#[tokio::main]
async fn main() {
    let (collect_tx, collect_rx) = tokio::sync::mpsc::unbounded_channel::<SenderBody>();
    let collect_rx = tokio_stream::wrappers::UnboundedReceiverStream::new(collect_rx);

    tokio::spawn(async move { handle::handle(collect_rx).await });

    let app = Router::new()
        .route("/", post(root))
        .layer(Extension(collect_tx.clone()));

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root(
    Extension(collect_tx): Extension<UnboundedSender<SenderBody>>,
    Json(params): Json<Body>,
) -> Json<Reponse> {
    let (sender, recv) = tokio::sync::oneshot::channel::<Reponse>();
    collect_tx
        .send(SenderBody {
            sender,
            user: params.user,
            method: params.method,
        })
        .unwrap();
    match recv.await {
        Ok(res) => Json(res),
        Err(_) => Json(Reponse::Err("出错了".to_string())),
    }
}
