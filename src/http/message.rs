use crate::auth::AuthUser;
use crate::modles::message::Msg;
use crate::persistent::MessageManage;
use crate::ApiContext;
use axum::extract::ws::{Message, WebSocket};
use axum::extract::WebSocketUpgrade;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Router};
use eChat::err::{Error, Result};
use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use tokio::join;
use tokio::sync::mpsc::{channel, Receiver};
use tracing::debug;

const DEFAULT_MESSAGE_QUEUE_SIZE: usize = 100;

pub fn router(ctx: &ApiContext) -> Router {
    let message_manage = MessageManage::new(ctx.db.clone());
    Router::new()
        .route("/ws", get(ws_handler))
        .layer(Extension(message_manage))
}

async fn ws_handler(
    // 升级http请求到websocket中
    ws: WebSocketUpgrade,
    auth_user: AuthUser,
    Extension(message_manage): Extension<MessageManage>,
    Extension(ctx): Extension<ApiContext>,
) -> impl IntoResponse {
    ws.on_upgrade(|ws| handle_socket(ws, ctx, auth_user, message_manage))
}

async fn handle_socket(
    socket: WebSocket,
    ctx: ApiContext,
    auth_user: AuthUser,
    message_manage: MessageManage,
) {
    debug!("receiver a connect");
    // websocket sender and receiver
    let (sender, receiver) = socket.split();
    //sender.send(Message::Text("test".into())).await.unwrap();
    // a channel which community with other user
    let (tx, rx) = channel::<Msg>(DEFAULT_MESSAGE_QUEUE_SIZE);
    // save current user sender end for other user to send message
    ctx.sender_map.lock().await.insert(auth_user.uid, tx);

    // TODO: make a nicer name
    let receiver_task = receiver_message(receiver, &ctx, message_manage, auth_user);
    let sender_task = sender_message(sender, rx);
    let (r1, r2) = join!(receiver_task, sender_task);
    r1.unwrap();
    r2.unwrap();
}

async fn sender_message(
    mut sender: SplitSink<WebSocket, Message>,
    mut receiver: Receiver<Msg>,
) -> Result<()> {
    while let Some(msg) = receiver.recv().await {
        sender
            .send(Message::Text(serde_json::to_string(&msg)?))
            .await?;
    }
    Err(Error::unprocessable_entity([("msg", "websocket closed")]))
}

async fn receiver_message(
    mut receiver: SplitStream<WebSocket>,
    ctx: &ApiContext,
    message_manage: MessageManage,
    auth_user: AuthUser,
) -> Result<()> {
    // TODO: handler time out situation, use tokio::time::timeout
    while let Some(message) = receiver.next().await {
        let message = message?;
        debug!("receiver a message {:?}", message);
        match message {
            Message::Text(message) => {
                let map = &ctx.sender_map;
                if let Ok(msg) = serde_json::from_str::<Msg>(&message) {
                    // first save message
                    message_manage.create_message(msg.to_message(auth_user.uid)).await?;
                    debug!(
                        "receiver a message from {}({}): {}",
                        auth_user.uid, auth_user.username, msg.content
                    );
                    // TODO: consider the receiver type is group
                    if let Some(sender) = map.lock().await.get(&msg.receiver_id) {
                        debug!("send message {:?} to {}", msg, msg.receiver_id);
                        sender.send(msg).await.expect("send message error");
                    }
                }else {
                    // this unwrap is safe, because there are a sender of the user in the dash
                    let map = map.lock().await;
                    let sender = map.get(&auth_user.uid).unwrap(); 
                    sender.send(Msg::new("message format error")).await.expect("send message error");
                }
            }
            Message::Close(_) => {
                let map = &ctx.sender_map;
                map.lock().await.remove(&auth_user.uid);
            }
            _ => {
                // TODO: handle other message
                todo!()
            }
        }
    }
    Err(Error::unprocessable_entity([("msg", "websocket closed")]))
}
