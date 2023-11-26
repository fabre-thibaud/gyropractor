use crate::{
    ws::{Client, RegisterRequest, RegisterResponse},
    Clients, Result,
};
use futures::{FutureExt, StreamExt};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use uuid::Uuid;
use warp::{
    filters::ws::{Message, WebSocket},
    http::StatusCode,
    reply::{json, Reply},
};

async fn client_connection(ws: WebSocket, id: String, clients: Clients, mut client: Client) {
    let (client_ws_sender, mut client_ws_rcv) = ws.split();
    let (client_sender, client_rcv) = mpsc::unbounded_channel();

    let client_rcv = UnboundedReceiverStream::new(client_rcv);

    tokio::task::spawn(client_rcv.forward(client_ws_sender).map(|result| {
        if let Err(e) = result {
            eprintln!("error sending websocket msg: {}", e);
        }
    }));

    client.sender = Some(client_sender);
    clients.write().await.insert(id.clone(), client.clone());

    println!("{} connected", id);

    while let Some(result) = client_ws_rcv.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("error receiving ws message for id: {}): {}", id.clone(), e);
                break;
            }
        };

        client_msg(&id, msg, client.clone()).await;
    }

    clients.write().await.remove(&id);

    println!("{} disconnected", id);
}

async fn client_msg(id: &str, msg: Message, client: Client) {
    println!("received message from {}: {:?}", id, msg);
    let message = match msg.to_str() {
        Ok(v) => v,
        Err(_) => return,
    };

    if message == "ping" || message == "ping\n" {
        if let Some(sender) = &client.sender {
            let _ = sender.send(Ok(Message::text("pong\n")));
        }

        return;
    }
}

async fn register_client(id: String, user_id: usize, clients: Clients) {
    clients.write().await.insert(
        id,
        Client {
            user_id,
            sender: None,
        },
    );
}

pub async fn handler(ws: warp::ws::Ws, id: String, clients: Clients) -> Result<impl Reply> {
    let client = clients.read().await.get(&id).cloned();

    match client {
        Some(c) => Ok(ws.on_upgrade(move |socket| client_connection(socket, id, clients, c))),
        None => Err(warp::reject::not_found()),
    }
}

pub async fn register(body: RegisterRequest, clients: Clients) -> Result<impl Reply> {
    let user_id = body.user_id;
    let uuid = Uuid::new_v4().as_simple().to_string();

    register_client(uuid.clone(), user_id, clients).await;

    Ok(json(&RegisterResponse {
        url: format!("ws://gyro-test:8000/ws/{}", uuid),
        id: format!("{}", uuid),
    }))
}

pub async fn unregister(id: String, clients: Clients) -> Result<impl Reply> {
    clients.write().await.remove(&id);

    Ok(StatusCode::NO_CONTENT)
}
