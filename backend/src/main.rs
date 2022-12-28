use actix_web::{
    get,
    middleware::Logger,
    post,
    web::{self, scope},
    App, HttpResponse, HttpServer, Responder,
};
use actix_web_lab::web::spa;

#[get("")]
async fn hello() -> impl Responder {
    println!("Hello");
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    println!("Hey");
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    HttpServer::new(|| {
        let logger = Logger::default();

        App::new()
            .wrap(logger)
            .service(
                scope("/api")
                    .service(hello)
                    .service(echo)
                    .route("/hey", web::get().to(manual_hello)),
            )
            .service(
                spa()
                    .index_file("./frontend/dist/index.html")
                    .static_resources_mount("/")
                    .static_resources_location("./frontend/dist")
                    .finish(),
            )
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}
