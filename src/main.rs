use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};

async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Ok")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/health_check", web::head().to(health_check))
    })
    .bind("127.0.0.1:8091")?
    .run()
    .await
}
