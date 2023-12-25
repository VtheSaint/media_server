use actix_web::web::{ServiceConfig, scope};

use crate::handlers::start_connection::start_connection;

pub fn routes_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1")
            .service(
                scope("/ws")
                            .service(start_connection)
            )
    );
}   