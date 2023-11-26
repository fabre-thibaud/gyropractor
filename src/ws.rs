use serde_derive::{Deserialize, Serialize};
use tokio::sync::mpsc;
use warp::ws::Message;

#[derive(Debug, Clone)]
pub struct Client {
    pub user_id: usize,
    pub sender: Option<mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>,
}

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    pub user_id: usize
}

#[derive(Serialize, Debug)]
pub struct RegisterResponse {
    pub url: String,
}
