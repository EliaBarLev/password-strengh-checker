use actix_web::{post, get, HttpResponse, Responder};
use crate::models::{ScoreRequest, ApiResponse};
use password_engine::analyze;

#[post("/score")]
pub async fn score_password(body: actix_web::web::Json<ScoreRequest>) -> impl Responder {
    let result = analyze(&body.password);

    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: result,
    })
}

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: "OK",
    })
}
