use actix_web::{HttpResponse, Responder};

pub async fn redirect(location: &str) -> impl Responder {
    HttpResponse::PermanentRedirect().append_header(("Location", location)).finish()
 }