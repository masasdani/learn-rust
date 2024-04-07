use actix_web::{get, web, HttpResponse, Responder};

#[get("/")]
pub async fn ok_index() -> &'static str {
    "Ok!"
}

#[get("/{code}")]
pub async fn redirect_to_code(path: web::Path<String>) -> HttpResponse {
    if path.into_inner() == "404" {
        return HttpResponse::NotFound().body("Not Found");
    } else {
        return HttpResponse::PermanentRedirect()
            .append_header(("Location", "https://masasdani.com")).finish();
    }
}
