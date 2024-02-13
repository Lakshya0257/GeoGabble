use crate::core::reset::reset_user;
use crate::routes::sc_threads::{incoming_req, read_messages};
use crate::utils::app_state::AppState;
use axum::extract::ws::Message;
use axum::extract::Query;
use axum::{
    extract::{
        ws::{WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures_util::stream::StreamExt;
use serde::Deserialize;
use tokio::select;
use tokio::sync::mpsc;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub user_id: String,
}

pub async fn handler(
    ws: WebSocketUpgrade,
    State(appstate): State<AppState>,
    Query(query): Query<QueryParams>,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, appstate, query.user_id.clone()))
}

pub async fn handle_socket(sc: WebSocket, client: AppState, user_id: String) {
    println!("Connected");
    let (sender, receiver) = sc.split();

    let (user_sender, user_receiver): (mpsc::Sender<Message>, mpsc::Receiver<Message>) =
        mpsc::channel(100);

    {
        let x = client
            .connections
            .lock()
            .await
            .insert(user_id.clone(), user_sender.clone());
        drop(x);
    }
    let read = tokio::spawn(read_messages(sender, user_receiver));
    let incom = tokio::spawn(incoming_req(receiver, user_sender, client.clone()));

    select! {
        _ = read => println!("read_task completed"),
        _ = incom => println!("incom_task completed"),
    };

    println!("Removing user: {}",user_id.clone());

    reset_user(user_id, client).await;
}
