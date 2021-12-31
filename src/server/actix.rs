use actix_web::{App, web, HttpServer, Responder};
use actix_web::web::Data;
use crate::logger::Logger;
use crate::server::auth::TestMiddleware;

//#[actix_rt::main]
pub async fn run_actix() -> Result<(), std::io::Error> {
    let logger = Logger::new(Some("Actix"));
    logger.info("Starting Actix Web Server!");
    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .wrap(TestMiddleware)
            .default_service(web::get().to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

async fn index() -> actix_web::HttpResponse {
    let data = serde_json::json!({
        "str": "Hello, middleware!"
    });
    actix_web::HttpResponse::Ok().json(data)
}