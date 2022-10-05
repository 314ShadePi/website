use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};

#[get("/ping")]
async fn ping() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();

    let dist_dir = std::env::var("DIST_DIR").unwrap_or_else(|_| "./dist".to_string());
    let addr = std::env::var("ACTIX_ADDR").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("ACTIX_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .unwrap_or(8080);

    let front_service =
        move || actix_files::Files::new("/", dist_dir.clone()).index_file("index.html");

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .service(ping)
            .service(front_service())
    })
    .bind((addr, port))?
    .run()
    .await
}
