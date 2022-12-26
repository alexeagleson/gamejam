use crate::database::DatabaseLock;
use crate::websocket::connections::USER_ID_COUNTER;
use crate::websocket::message::handle_message;
use crate::{game::world::WorldLock, websocket::disconnect::handle_disconnect};
use futures_util::{SinkExt, StreamExt, TryFutureExt};
use std::sync::atomic::Ordering;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::WebSocket;

use super::connections::ConnectionsLock;

pub async fn handle_new_connection(
    ws: WebSocket,
    connections: ConnectionsLock,
    db: DatabaseLock,
    mut world: WorldLock,
) {
    let new_id = USER_ID_COUNTER.fetch_add(1, Ordering::Relaxed);

    eprintln!("new chat user: {}", new_id);

    // Split the socket into a sender and receive of messages.
    let (mut user_ws_tx, mut user_ws_rx) = ws.split();

    // Use an unbounded channel to handle buffering and flushing of messages to the websocket
    let (tx, rx) = mpsc::unbounded_channel();
    let mut rx = UnboundedReceiverStream::new(rx);

    tokio::task::spawn(async move {
        while let Some(message) = rx.next().await {
            user_ws_tx
                .send(message)
                .unwrap_or_else(|e| {
                    eprintln!("websocket send error: {}", e);
                })
                .await;
        }
    });

    // Add user to the list of active connections
    connections.write().await.new_connection(new_id, tx);

    // Every time the user sends a message, broadcast it to all other users
    while let Some(result) = user_ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(uid={}): {}", new_id, e);
                break;
            }
        };
        handle_message(new_id, msg, &connections, &db, &mut world).await;
    }

    // user_ws_rx stream will keep processing as long as the user stays
    // connected. Once they disconnect, then...
    handle_disconnect(new_id, &connections, &world).await;
}
