use std::time::{Duration, Instant};
use actix::prelude::ContextFutureSpawner;
use actix::{fut, ActorContext, WrapFuture, ActorFutureExt, StreamHandler, Handler};
use actix::{Addr, Actor, AsyncContext};
use actix_web_actors::ws;
use serde_json::Value;
use uuid::Uuid;


use super::lobby::Lobby;
use super::messages::{Connect, Disconnect, WsMessage, AudioFrame, VideoFrame};


const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsConn {
    lobby_addr: Addr<Lobby>,
    heartbeat: Instant,
    session_id: Uuid,
    room_id: Uuid,
}


impl WsConn {
    pub fn new(lobby: Addr<Lobby>, session_id: Uuid, room_id: Uuid) -> WsConn {
        WsConn { 
            lobby_addr: lobby,
            heartbeat: Instant::now(),
            session_id: session_id,
            room_id: room_id
        }
    }

    pub fn heartbeat(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.heartbeat) > CLIENT_TIMEOUT {
                println!("Disconnsecting failed heartbeat");
                ctx.stop();
                return;
            }

            ctx.ping(b"hi");
        });
    }

}


impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;


    fn started(&mut self, ctx: &mut Self::Context) {
        self.heartbeat(ctx);

        let addr = ctx.address();
        self.lobby_addr
            .send(
                Connect {
                    addr: addr.recipient(),
                    self_id: self.session_id,
                    room_id: self.room_id
                }
            )
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }


    fn stopping(&mut self, _ctx: &mut Self::Context) -> actix::Running {
        self.lobby_addr.do_send(Disconnect {
            self_id: self.session_id,
            room_id: self.room_id
        });
        actix::Running::Stop
    }
}




impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, item: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Ping(msg)) => {
                self.heartbeat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.heartbeat= Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => {
                ctx.binary(bin)
            },
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(ws::Message::Text(s)) => {
                let x: Value = serde_json::from_str(&s).expect("Can't parse JSON");
                if let Some(j_type) = x.get("j_type") {
                    if let Some(j_type_str) = j_type.as_str() {
                        match j_type_str {
                            "VF" => {
                                let x: VideoFrame = serde_json::from_str(&s).unwrap();
                                self.lobby_addr.do_send(x);
                            },
                            "AF" => {
                                let x: AudioFrame = serde_json::from_str(&s).unwrap();
                                self.lobby_addr.do_send(x);
                            },
                            _ => {
                                // Обработка других случаев
                                println!("Unknown type");
                            },
                        }
                    } else {
                        // Обработка случая, когда "j_type" не является строкой
                        println!("j_type is not a string");
                    }
                }
                
            }
            Err(e) => std::panic::panic_any(e),
        }
    }
}



impl Handler<WsMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}


impl Handler<AudioFrame> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: AudioFrame, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}


impl Handler<VideoFrame> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: VideoFrame, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(serde_json::to_string(&msg).unwrap());
    }
}


// pub fn text(&mut self, text: impl Into<ByteString>) {
    // self.write_raw(Message::Text(text.into()));