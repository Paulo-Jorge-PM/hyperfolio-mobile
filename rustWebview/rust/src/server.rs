use actix_cors::Cors;
//use actix_files;
use actix_web::body::Body;
use actix_web::{get, post, http, web, App, HttpResponse, HttpServer, Responder};

use crate::controllers::index::index;

const PORT:&str = "8002";
const HOST:&str = "0.0.0.0";

#[actix_web::main]
pub async fn run_app() -> std::io::Result<()> {
    error!("{}", "Servidor running ;)");
    println!(">>> Running on http://{}:{}...", HOST, PORT);
    HttpServer::new(|| {

        /*let _cors = Cors::default()
              .allowed_origin("https://www.rust-lang.org/")
              .allowed_origin_fn(|origin, _req_head| {
                  origin.as_bytes().ends_with(b".rust-lang.org")
              })
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);*/

        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .service(index)
            //.route("/", web::get().to(index::index))
            //.route("/", web::get().to(controllers::index))
            //.service(actix_files::Files::new("/", "file:///android_asset/html").index_file("index.html"))
            //.service(actix_files::Files::new("/", "./src/html").show_files_listing())
            //.service(actix_files::Files::new("/", "././react").show_files_listing())
            //.service(index)
            //.service(controllers::index)
            //.service(hi)
            //.service(echo)
            //.route("/hey", web::get().to(manual_hello))
    })
    .bind([HOST, PORT].join(":"))?
    .run()
    .await
}

/*#[get("/")]
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
}*/

/*
#[get("/hi")]
async fn hi() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}*/

/*#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}*/