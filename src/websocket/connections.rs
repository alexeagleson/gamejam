use crate::api::UserId;
use std::{
    collections::HashMap,
    sync::{atomic::AtomicI32, Arc},
};
use tokio::sync::{mpsc, RwLock};
use warp::ws::Message;

pub static USER_ID_COUNTER: AtomicI32 = AtomicI32::new(1);

#[derive(Debug)]
pub struct Connections(pub HashMap<i32, mpsc::UnboundedSender<Message>>);

impl Default for Connections {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl Connections {
    pub fn new_connection(&mut self, id: UserId, sender: mpsc::UnboundedSender<Message>) {
        self.0.insert(id, sender);
    }
}

pub type ConnectionsLock = Arc<RwLock<Connections>>;
