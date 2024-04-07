use actix_web::{get, post};

#[post("/api/signin")]
pub async fn api_index() -> &'static str {
    "Hello world!"
}

#[post("/api/shorten")]
pub async fn shorten_url() -> &'static str {
    "Shorten URL!"
}