use std::io;

use actix_files::Files;
use actix_session::CookieSession;
use actix_web::{middleware, App, HttpServer};

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            // enable automatic response compression - usually register this first
            .wrap(middleware::Compress::default())
            // cookie session middleware
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            // enable logger - always register Actix Web Logger middleware last
            .wrap(middleware::Logger::default())
            .service(Files::new("/pkg", "./pine-client/pkg"))
            .service(Files::new("/{path:.*}", "./pine-client").index_file("index.html"))
    })
    .bind(("127.0.0.1", 9500))?
    .workers(2)
    .run()
    .await
}
