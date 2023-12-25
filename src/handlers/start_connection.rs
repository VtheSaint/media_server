use actix::Addr;
use actix_web::{get, web::Data, web::Path, web::Payload, Error, HttpResponse, HttpRequest};
use actix_web_actors::ws;
use uuid::Uuid;

use crate::websocket::{connection::WsConn, lobby::Lobby};


#[get("/{room_id}/{session_id}")]
pub async fn start_connection(
    req: HttpRequest,
    stream: Payload,
    path: Path<(Uuid, Uuid,)>,
    srv: Data<Addr<Lobby>>,
) -> Result<HttpResponse, Error> {
    let query_params = path.into_inner();
    let ws = WsConn::new(
        srv.get_ref().clone(),
        query_params.1,
        query_params.0
    );
    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}