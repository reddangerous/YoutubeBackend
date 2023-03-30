use actix_cors::Cors;
use actix_web::{ web, App, HttpServer,http};
mod services;
use services::{search_channel};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
         let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8080")
            .allowed_origin("http://127.0.0.1:8080")
            .allowed_methods(vec!["GET", "POST","PUT", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);
        App::new()
            .service(web::resource("/{channel}").to(search_channel))
    })
    .bind("127.0.0.1:45678")?
    .run()
    .await
}
