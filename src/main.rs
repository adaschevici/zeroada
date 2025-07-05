use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};

async fn index() -> impl Responder {
    format!("Hello, world!")
}
async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello, {}!", name)
}

async fn health_check(req: HttpRequest) -> impl Responder {
    HttpResponse::NotImplemented().body("Health check not implemented")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/health", web::get().to(health_check))
            .route("/{name}", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
