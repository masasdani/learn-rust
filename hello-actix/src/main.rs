use actix_web::{web, App, HttpResponse, HttpServer};

mod utils;
mod controllers;
mod api;
mod service;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   HttpServer::new(|| {
      App::new()
         .service(controllers::ok_index)
         .service(controllers::redirect_to_code)
         .default_service(
            // change default 404 for GET request
            web::route().to(|| async { 
               HttpResponse::NotFound().body("Not Found") 
            })
         )
   })
   .bind(("127.0.0.1", 8080))?
   .run()
   .await
}