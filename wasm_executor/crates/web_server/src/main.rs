mod routes;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};

// it is patch as we would likely use it to change an existing flow
// that is persisted on server or network storage
const HELP: &str = r#"Routes:
PATCH /execute-flow
"#;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");

    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // enable CORS
            .wrap(cors)
            // routes
            .service(web::resource("/").route(web::get().to(|| async { HELP })))
            .service(web::resource("/execute-flow").route(web::patch().to(routes::execute_flow)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
