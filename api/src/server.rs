use actix_web::{App, HttpServer, web};
use crate::routes::{score_password, health};

pub async fn run() -> std::io::Result<()> {
    println!("🚀 Starting API on http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .service(score_password)
            .service(health)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
