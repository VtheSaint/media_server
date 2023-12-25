use actix::prelude::{Message, Recipient};
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
pub struct AudioFrame {
    pub storage_id: Uuid,
    pub body: Vec<u8>,
    pub part: u128,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct VideoFrame {
    pub storage_id: Uuid,
    pub body: Vec<u8>,
    pub part: u128,
}
