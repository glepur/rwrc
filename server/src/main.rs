extern crate dotenv;

use actix::prelude::*;
use actix_web::body::Body;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use dotenv::dotenv;
use enigo::*;
use json::parse;
use mime_guess::from_path;
use rust_embed::RustEmbed;
use std::env;
use std::time::{Duration, Instant};

use std::borrow::Cow;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(300);

async fn ws_index(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
  println!("{:?}", r);
  let res = ws::start(MyWebSocket::new(), &r, stream);
  println!("{:?}", res);
  res
}

#[derive(RustEmbed)]
#[folder = "../client/target/deploy/"]
struct Asset;

struct MyWebSocket {
  hb: Instant,
  enigo: Enigo,
}

impl Actor for MyWebSocket {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, ctx: &mut Self::Context) {
    self.hb(ctx);
  }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
  fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
    match msg {
      Ok(ws::Message::Ping(msg)) => {
        self.hb = Instant::now();
        ctx.pong(&msg);
      }
      Ok(ws::Message::Pong(_)) => {
        self.hb = Instant::now();
      }
      Ok(ws::Message::Text(text)) => {
        let message = parse(&text).unwrap();
        println!("Received message: {}", message);
        let msg_type: &str = &message["type"].to_string();
        match msg_type {
          "move" => self.enigo.mouse_move_relative(
            message["x"].as_i32().unwrap(),
            message["y"].as_i32().unwrap(),
          ),
          "click_left" => self.enigo.mouse_click(MouseButton::Left),
          "click_right" => self.enigo.mouse_click(MouseButton::Right),
          _ => (),
        };
      }
      Ok(ws::Message::Binary(_)) => (),
      Ok(ws::Message::Close(_)) => {
        ctx.stop();
      }
      _ => ctx.stop(),
    }
  }
}

impl MyWebSocket {
  fn new() -> Self {
    Self {
      hb: Instant::now(),
      enigo: Enigo::new(),
    }
  }

  fn hb(&self, ctx: &mut <Self as Actor>::Context) {
    ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
      if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
        println!("Websocket Client heartbeat failed, disconnecting!");
        ctx.stop();
        return;
      }
    });
  }
}

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
  std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
  env_logger::init();
  dotenv().ok();
  let port = env::var("PORT").unwrap();
  HttpServer::new(|| {
    App::new()
      .wrap(middleware::Logger::default())
      .service(web::resource("/ws/").route(web::get().to(ws_index)))
      .service(web::resource("/").route(web::get().to(index)))
      .service(web::resource("/{_:.*}").route(web::get().to(dist)))
  })
  .bind(format!("0.0.0.0:{}", port))?
  .run()
  .await
}
