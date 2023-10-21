use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

#[get("/api/health_check")]
pub async fn health_check() -> impl Responder {
    const MESSAGE: &str = "Build a simple CRUD";
    HttpResponse::Ok().json(json!({"status": "success", "message": MESSAGE}))
}