use actix_web::body::Body;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use mime_guess::from_path;
use rust_embed::RustEmbed;

use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "../client/target/deploy/"]
struct Asset;

fn handle_embedded_file(path: &str) -> HttpResponse {
  match Asset::get(path) {
    Some(content) => {
      let body: Body = match content {
        Cow::Borrowed(bytes) => bytes.into(),
        Cow::Owned(bytes) => bytes.into(),
      };
      HttpResponse::Ok()
        .content_type(from_path(path).first_or_octet_stream().as_ref())
        .body(body)
    }
    None => HttpResponse::NotFound().body("404 Not Found"),
  }
}

fn index(_req: HttpRequest) -> HttpResponse {
  handle_embedded_file("index.html")
}

fn dist(req: HttpRequest) -> HttpResponse {
  handle_embedded_file(&req.path()[1..])
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(web::resource("/").route(web::get().to(index)))
      .service(web::resource("/{_:.*}").route(web::get().to(dist)))
  })
  // TODO: read port from mutual config
  .bind("0.0.0.0:1337")?
  .run()
  .await
}
