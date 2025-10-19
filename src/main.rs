use actix_cors::Cors;
use actix_web::{App, HttpServer, web};

use ucalg_baja_cloud::hello;
use ucalg_baja_cloud::merch_shop;
use ucalg_baja_cloud::sponsors::get_sponors;

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
            .service(get_sponors)
            .service(
                web::scope("/shop")
                    .service(merch_shop::recieve_order)
                    .service(merch_shop::get_merch),
            )
    })
    .bind(("0.0.0.0", 6526))?
    .run()
    .await
}
