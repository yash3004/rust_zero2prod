pub mod configurations;
pub mod schemas;
pub mod operations;

use actix_web::dev::Server;
use actix_web::{App, HttpResponse, HttpServer, web};
use std::net::TcpListener;

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({"status": "ok"}))
}

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

async fn subscribe(_form: web::Form<FormData>) -> HttpResponse {
    if _form.name.is_empty() && _form.email.is_empty() {
        return HttpResponse::BadRequest().finish();
    }
    HttpResponse::Ok().json(serde_json::json!({"message": format!("Welcome {} to the newsletter!", _form.name)}))
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}

// pub fn run(address: &str) -> Result<Server, std::io::Error> {
//     let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
//         .bind(address)?
//         .run();
//     Ok(server)
// }
