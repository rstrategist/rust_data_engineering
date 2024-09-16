fn main() {
    use actix_web::{web, App, HttpServer, Responder};

    async fn index() -> impl Responder {
        "Hello, World!"
    }

    #[actix_web::main]
    async fn main() -> std::io::Result<()> {
        HttpServer::new(|| App::new().service(web::resource("/").to(index)))
            .bind("127.0.0.1:16634")?
            .run()
            .await
    }
}
