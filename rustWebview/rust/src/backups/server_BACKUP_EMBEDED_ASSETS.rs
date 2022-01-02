use actix_cors::Cors;
use actix_files;
use actix_web::body::Body;
use actix_web::{get, post, http, web, App, HttpResponse, HttpServer, Responder};

use mime_guess::from_path;
use rust_embed::RustEmbed;
use std::borrow::Cow;

const PORT:&str = "8002";
const HOST:&str = "0.0.0.0";

#[derive(RustEmbed)]
#[folder = "react/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
  match Asset::get(path) {
    Some(content) => {
      let body: Body = match content {
        Cow::Borrowed(bytes) => bytes.into(),
        Cow::Owned(bytes) => bytes.into(),
      };
      HttpResponse::Ok().content_type(from_path(path).first_or_octet_stream().as_ref()).body(body)
    }
    None => HttpResponse::NotFound().body("404 Not Found"),
  }
}

fn index() -> HttpResponse {
  handle_embedded_file("index.html")
}

fn all(path: web::Path<String>) -> HttpResponse {
  handle_embedded_file(&path.0)
}

fn staticFiles(path: web::Path<String>) -> HttpResponse {
  handle_embedded_file(&path.0)
}

#[actix_web::main]
pub async fn run_app() -> std::io::Result<()> {
    error!("{}", "Servidor running ;)");
    println!(">>> Running on http://{}:{}...", HOST, PORT);
    HttpServer::new(|| {

        let _cors = Cors::default()
              .allowed_origin("https://www.rust-lang.org/")
              .allowed_origin_fn(|origin, _req_head| {
                  origin.as_bytes().ends_with(b".rust-lang.org")
              })
              .allowed_methods(vec!["GET", "POST"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);

        App::new()
            .service(web::resource("/app/dashboard").route(web::get().to(index)))
            .service(web::resource("/{_:.*}").route(web::get().to(all)))
            .service(web::resource("/static/{_:.*}").route(web::get().to(staticFiles)))

            //.service(actix_files::Files::new("/", "file:///android_asset/html").index_file("index.html"))
            //.service(actix_files::Files::new("/", "./src/html").show_files_listing())
            //.service(actix_files::Files::new("/", "././react").show_files_listing())
            //.service(index)
            //.service(echo)
            //.route("/hey", web::get().to(manual_hello))
    })
    .bind([HOST, PORT].join(":"))?
    .run()
    .await
}


/*
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok()
    .content_type("text/html; charset=utf-8")
    .body(include_str!("./html/index.html"))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}*/