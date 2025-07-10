use actix_web::dev::Server;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};

async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().finish()
}

pub fn run() -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/health_check", web::head().to(health_check))
    })
    .bind("127.0.0.1:8091")?
    .run();
    Ok(server)
}
