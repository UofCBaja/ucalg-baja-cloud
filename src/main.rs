use std::sync::{Arc, Mutex};

use actix_cors::Cors;
use actix_web::{App, HttpServer, web};

use ucalg_baja_cloud::ApiDoc;
use ucalg_baja_cloud::database;
use ucalg_baja_cloud::merch_shop;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Running on port http://localhost:6526");

    let database = web::Data::new(Mutex::new(database::Database::new()));
    HttpServer::new(move || {
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
            .app_data(database.clone())
            .service(darkicewolf50_actix_setup::health_check_swagger)
            .service(
                web::scope("/shop")
                    .service(merch_shop::recieve_order)
                    .service(merch_shop::get_merch),
            )
            // accessable at /swagger-ui/
            // swagger/OpenAPI docs
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(("0.0.0.0", 6526))?
    .run()
    .await
}
