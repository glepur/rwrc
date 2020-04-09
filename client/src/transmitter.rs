use json::*;
use std::cell::RefCell;
use std::rc::Rc;
use stdweb::console;
use stdweb::traits::*;
use stdweb::web::set_timeout;
use stdweb::web::WebSocket;

use stdweb::web::event::{SocketCloseEvent, SocketErrorEvent, SocketOpenEvent};

const THROTTLE_TIME_MILIS: u32 = 100;

pub struct Transmitter {
  ws: WebSocket,
  activated: bool,
  coordinates: Coordinates,
}

struct Coordinates {
  x: f64,
  y: f64,
}

impl Transmitter {
  pub fn new() -> Self {
    let ws = WebSocket::new("wss://echo.websocket.org").expect("Could not establish connection.");

    Transmitter::attach_ws_callbacks(&ws);

    Transmitter {
      ws,
      activated: false,
      coordinates: Coordinates { x: 0.0, y: 0.0 },
    }
  }

  fn attach_ws_callbacks(ws: &WebSocket) {
    ws.add_event_listener(|_: SocketOpenEvent| {
      console!(log, "Connection established");
    });

    ws.add_event_listener(|_: SocketErrorEvent| {
      console!(error, "Connection errored");
    });

    ws.add_event_listener(|event: SocketCloseEvent| {
      console!(error, "Connection closed: {}", event.reason());
    });
  }

  pub fn activate(&mut self) {
    self.activated = true;
  }

  pub fn deactivate(&mut self) {
    self.activated = false;
  }

  pub fn start_emit(&self, rc: Rc<RefCell<Transmitter>>) {
    let Coordinates { x, y } = rc.borrow().coordinates;
    let message = object! {
      x: x,
      y: y
    };
    self.ws.send_text(&message.dump()).unwrap();
    if self.activated {
      set_timeout(
        move || rc.borrow().start_emit(rc.clone()),
        THROTTLE_TIME_MILIS,
      );
    }
  }

  pub fn update(&mut self, x: f64, y: f64) {
    self.coordinates = Coordinates { x, y };
  }
}
