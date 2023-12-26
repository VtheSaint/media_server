use std::env;
use actix::Actor;
use actix_web::{Result, HttpServer, App, web::Data, middleware::Logger};
use dotenvy::dotenv;

use crate::{websocket::lobby::Lobby, routes::routes_factory};


pub mod websocket;
pub mod routes;
pub mod handlers;

#[actix_rt::main] 
async fn main() -> Result<(), std::io::Error> {
    
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env::set_var("RUST_BACKTRACE", "RUST_BACKTRACE=1");
    env_logger::init();

    let server_url = env::var("SERVER_URL")
    .expect("SERVER_URL must be set");

    let lobby = Lobby::default().start(); //create and spin up a lobby


    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(lobby.clone()))
            .wrap(Logger::default())
            .configure(routes_factory)
    })
    .bind(server_url)?
    .run()
    .await
}
