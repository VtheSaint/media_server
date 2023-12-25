use std::collections::HashMap;

use actix::{Recipient, Actor, Context, Handler};
use uuid::Uuid;

use super::messages::{WsMessage, Disconnect, Connect, AudioFrame, VideoFrame};



type Socket = Recipient<WsMessage>;

pub struct Room {
    pub participants: HashMap<Uuid, Socket>
}

impl Default for Room {
    fn default() -> Self {
        Self {
            participants: HashMap::new(),
        }
    }
}

pub struct Lobby {
    pub sessions: HashMap<Uuid, Room>,
}

impl Default for Lobby {
    fn default() -> Self {
        Self { 
            sessions: HashMap::new(),
        }
    }
}

impl Lobby {
    fn send_connect(&self, message: &str, room_id: &Uuid, id_to: &Uuid) {
        if let Some(room) = self.sessions.get(room_id) {
            if let Some(socket_recipient) = room.participants.get(id_to) {
                let _ = socket_recipient
                .do_send(
                    WsMessage(message.to_string())
                );
            }

        } else {
            println!("attempting to send message but couldn't find user id.");
        }
    }
}


impl Actor for Lobby {
    type Context = Context<Self>;
}

impl Handler<Disconnect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) -> Self::Result {
        if  let Some(room) = self.sessions.get_mut(&msg.room_id) {
            room.participants.remove(&msg.self_id);
            if room.participants.is_empty() {
                self.sessions.remove(&msg.room_id);
            }
        }
    }
}


impl Handler<Connect> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) -> Self::Result {
        if let Some(room) = self.sessions.get_mut(&msg.room_id) {
            room.participants.insert(msg.self_id, msg.addr);
        }
        // TODO: ADD connect body struct 
        self.send_connect(&format!("id: {}, room_id: {}", msg.self_id, msg.room_id), &msg.room_id, &msg.self_id)
    }
}


impl Handler<AudioFrame> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: AudioFrame, ctx: &mut Self::Context) -> Self::Result {
        
    }
}

impl Handler<VideoFrame> for Lobby {
    type Result = ();

    fn handle(&mut self, msg: VideoFrame, ctx: &mut Self::Context) -> Self::Result {
        
    }
}


