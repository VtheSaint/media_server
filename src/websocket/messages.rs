use actix::prelude::{Message, Recipient};
use serde::{Serialize, Deserialize};
use uuid::Uuid;



#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub self_id: Uuid,
    pub room_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub self_id: Uuid,
    pub room_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Serialize, Deserialize, Debug)]
pub struct AudioFrame {
    pub j_type: String,
    pub storage_id: Uuid,
    pub body: Vec<u8>,
    pub part: u128,
}

#[derive(Message)]
#[rtype(result = "()")]
#[derive(Serialize, Deserialize, Debug)]
pub struct VideoFrame {
    pub j_type: String,
    pub storage_id: Uuid,
    pub body: Vec<u8>,
    pub part: u128,
}
