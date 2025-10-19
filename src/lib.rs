// use actix_web::HttpResponse;
use actix_web::{HttpRequest, Responder, get, web};
// use serde::{Deserialize, Serialize};
use serde_json::json;

pub mod merch_shop;
pub mod sponsors;

/// Logs out the request to the application, with method, and path it took to get there
///
/// # Params
///
/// - method - One of the HTTP request mothds.
/// - path_source - the path you need to get to the function
///
/// # Returns
///
/// - Nothing, prints to terminal the method used and path it is going to
///
/// # Example
/// ```rust
/// // this is how a public but internal module would be used by an outside user (ex_crate needs to be changed)
/// use ucalg_baja_cloud::log_incoming;
/// let result = log_incoming("GET", "/");
/// // unit value and should only be printed to the terminal
/// assert_eq!(result, ())
/// ```
/// # Author (s)
///
/// - Brock <brock@darkicewolf50.dev>
/// semi-permanent email, do not need to respond but try to be a good alumni
pub fn log_incoming(method: &'static str, path_source: &str) {
    println!("{} request, path: {}", method, path_source);
}

/// A quick method to check if the server is alive and running
/// This also keeps out scrapers from getting useful data
///
/// # Params
///
/// - nothing - needs nothing to check health of server
///
/// # Returns
///
/// - Json response with a alive message
///
/// # Example
/// ```rust
/// use ucalg_baja_cloud::hello;
/// use actix_web::{web, test, App};
/// use serde_json::json;
///
/// #[actix_web::test]
/// async fn test_hello() {
///     let app = test::init_service(App::new().service(hello)).await;
///     let req = test::TestRequest::get().uri("/").to_request();
///     let resp: serde_json::Value = test::call_and_read_body_json(&app, req).await;
///
///     assert_eq!(resp, json!({
///         "body": {
///             "message": "Hello I am alive, this does nothing"
///         }
///     }));
/// }
/// ```
/// # Author (s)
///
/// - Brock <brock@darkicewolf50.dev>
/// semi-permanent email, do not need to respond but try to be a good alumni

#[get("/")]
pub async fn hello() -> impl Responder {
    log_incoming("GET", "/");
    web::Json(json!({
    "body": {
            "message": "Hello I am alive, this does nothing"
        }
    }))
}
