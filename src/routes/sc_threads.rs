use crate::{
    core::location::{check_buffer, get_connected_users, update_lat_lng, update_location},
    models::message_model::LocationDto,
    utils::app_state::AppState,
};
use axum::extract::ws::{Message, WebSocket};
use futures_util::{
    sink::SinkExt,
    stream::{SplitSink, SplitStream, StreamExt},
};

pub async fn read_messages(
    mut sc: SplitSink<WebSocket, Message>,
    mut user_receiver: tokio::sync::mpsc::Receiver<Message>,
) {
    // println!("{}: Receiver Started");

    while let Some(mg) = user_receiver.recv().await {
        println!("Value received by external client");
        sc.send(mg).await.expect("Failed to send message");
    }
}

pub async fn incoming_req(
    mut sc: SplitStream<WebSocket>,
    _: tokio::sync::mpsc::Sender<Message>,
    client: AppState,
    user_id: String
) {
    // println!("Sender started");

    while let Some(msg) = sc.next().await {
        let msg = match msg {
            Ok(msg) => msg,
            Err(_) => return, // client disconnected
        };
        if let Message::Close(_) = &msg {
            println!("{} :Client sent close message",user_id);
            return;
        }

        // Deserialize the received JSON message into the DTO struct
        
        let message_dto: LocationDto = serde_json::from_slice(&msg.clone().into_data()).unwrap();
        println!("{} : Message: {:?}",user_id,message_dto);
        

        let update_location_task = tokio::spawn(update_location(
            client.clone(),
            message_dto.country.clone(),
            message_dto.state.clone(),
            message_dto.user_id.clone(),
        ));
        let update_lat_lng_task = tokio::spawn(update_lat_lng(client.clone(), message_dto.clone()));
        
        // Check the buffer only if the update_location and update_lat_lng tasks have completed
        if !check_buffer(client.clone(), message_dto.user_id.clone()).await {
            // Await the completion of both tasks
            let _ = tokio::try_join!(update_location_task, update_lat_lng_task);
        }

        let connections = get_connected_users(client.clone(), message_dto.user_id.clone()).await;
        println!("{}", connections.len());

        if message_dto.message != "".to_string() {
            {
                let data = client.connections.lock().await;
                println!("Connections: {}", data.len());
                for con in connections.iter() {
                    let y = data.get(con).cloned();
                    if let Some(valuess) = y {
                        let val = valuess
                            .send(Message::Text(message_dto.message.clone()))
                            .await;
                        println!("Value sent to other client{:?}", val);
                    };
                }
            };
        }
    }
}
