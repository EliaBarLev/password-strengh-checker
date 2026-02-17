mod routes;
mod models;
mod server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    server::run().await
}
