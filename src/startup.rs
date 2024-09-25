use actix_web::{web, App, HttpServer};
use actix_web::dev::Server;
use sqlx::PgPool;
use actix_web::middleware::Logger;
use std::net::TcpListener;
use crate::routes::health_checker::health_checker;
use crate::routes::subscriptions::subscribe;
use tracing_actix_web::TracingLogger;

// async fn greet(req: HttpRequest) -> impl Responder {
//     let name: &str = req.match_info().get("name").unwrap_or("World");
//     HttpResponse::Ok().body(format!("Hello {}!\n", &name))

// }

pub fn run(listener: TcpListener, db_pool: PgPool ) -> Result<Server, std::io::Error>{
    let connection = web::Data::new(db_pool);
    let server = HttpServer::new( move || {
        App::new() 
        .wrap(TracingLogger::default())
        .route("/", web::get().to(health_checker))
        // .route("/{name}", web::get().to(greet))
        .route("/health_checker", web::get().to(health_checker))
        .route("/subscriptions", web::post().to(subscribe))
        .app_data(connection.clone())

    })
    .listen(listener)?
    .run();

    Ok(server)
}