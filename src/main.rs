use actix_cors::Cors;
use actix_web::{web, App, HttpServer};

use ucalg_baja_cloud::hello;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running on port 6526");
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    // Allow any origin — or use `.allowed_origin("http://localhost:8080")` to restrict
                    .allow_any_origin()
                    // Allow common methods
                    .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
                    // Allow any headers
                    .allow_any_header(), // Optionally enable sending cookies, etc.
                                         //.supports_credentials()
            )
            .service(hello)
    })
    .bind(("0.0.0.0", 6526))?
    .run()
    .await
}
