use actix_web::{get, Responder, HttpResponse};

#[get("/")]
pub async fn index() -> impl Responder {
    error!("{}", "Entrouuuuuu no index!!!");
    let json = r#"
    {"data":
        [{"id":"666"}]
    }
    "#;
    HttpResponse::Ok()
    .content_type("application/json; charset=utf-8")
    .body(json)
}