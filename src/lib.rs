use std::sync::Arc;

// use actix_web::HttpResponse;
// use actix_web::{Responder, get, web};
// use serde::{Deserialize, Serialize};
// use serde_json::json;
use utoipa::OpenApi;

pub mod merch_shop;

pub mod database;

#[derive(OpenApi)]
#[openapi(paths(darkicewolf50_actix_setup::swagger_docs::health_check_swagger))]
pub struct ApiDoc;

pub type ArcString = Arc<str>;
pub type ArcVec<T> = Arc<[T]>;
