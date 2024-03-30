use actix_web::{get, post, put, web, App, HttpResponse, HttpServer, Responder};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(get_index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[get("/")]
async fn get_index() -> impl Responder {
   HttpResponse::Ok().body("Hello, Actix!")
}

#[post("/")]
async fn post_index() -> impl Responder {
   HttpResponse::Ok().body("Hello, Actix!")
}

#[put("/")]
async fn put_index() -> impl Responder {
   HttpResponse::Ok().body("Hello, Actix!")
}