use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

mod utils;
use crate::utils::redirect::redirect;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   HttpServer::new(|| {
      App::new()
         .service(get_index)
         .service(get_redirect)
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
   create_response("You just sent a POST request!").await
}

#[get("/redirect")]
async fn get_redirect() -> impl Responder {
   redirect("/").await
}


async fn create_response(message: &str) -> impl Responder {
   HttpResponse::Ok().body(message.to_string())
}