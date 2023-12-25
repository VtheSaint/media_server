use actix::Addr;
use actix_web::{get, web::Data, web::Path, HttpResponse};
use uuid::Uuid;

use crate::websocket::lobby::Lobby;


#[get("/{room_id}")]
pub async fn create_room(
    path: Path<(Uuid, Uuid,)>,
    srv: Data<Addr<Lobby>>,
) -> HttpResponse {
}