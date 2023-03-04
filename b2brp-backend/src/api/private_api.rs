use actix_web::{get, HttpResponse, Responder, Scope};

pub fn scope() -> Scope {
    Scope::new("/api").service(test)
}

#[get("/test")]
pub async fn test() -> impl Responder {
    HttpResponse::Ok().body("test")
}
