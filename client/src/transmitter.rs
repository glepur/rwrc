use json::*;
use std::cell::RefCell;
use std::rc::Rc;
use stdweb::console;
use stdweb::traits::*;
use stdweb::web::set_timeout;
use stdweb::web::{SocketReadyState, WebSocket};

use stdweb::web::event::{SocketCloseEvent, SocketErrorEvent, SocketMessageEvent, SocketOpenEvent};

const THROTTLE_TIME_MILIS: u32 = 5;
const SENSITIVITY: u32 = 10;

pub struct Transmitter {
  ws_url: String,
  ws: WebSocket,
  activated: bool,
  should_emit: bool,
  coordinates: Coordinates,
}

struct Coordinates {
  x: i32,
  y: i32,
}

impl Transmitter {
  pub fn new(ws: &str) -> Self {
    let ws_url = format!("ws://{}/ws/", ws);
    let transmitter = Self {
      ws_url: ws_url.clone(),
      ws: Transmitter::new_socket(&ws_url),
      activated: false,
      should_emit: false,
      coordinates: Coordinates { x: 0, y: 0 },
    };
    Transmitter::attach_ws_callbacks(&transmitter.ws);
    transmitter
  }

  fn new_socket(ws: &str) -> WebSocket {
    WebSocket::new(ws).expect("Could not establish connection.")
  }

  fn reconnect(&mut self) {
    self.ws = Transmitter::new_socket(&self.ws_url);
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

    ws.add_event_listener(|event: SocketMessageEvent| {
      console!(log, "{}", event.data().into_text().unwrap());
    });
  }

  pub fn activate(&mut self) {
    self.activated = true;
  }

  pub fn deactivate(&mut self) {
    self.activated = false;
    if self.ws.ready_state() == SocketReadyState::Closed {
      self.reconnect();
    }
  }

  pub fn start_emit(&self, rc: Rc<RefCell<Transmitter>>) {
    let Coordinates { x, y } = rc.borrow().get_adapted_coordinates();
    if self.ws.ready_state() == SocketReadyState::Open && self.should_emit {
      let message = object! {
        x: x,
        y: y
      };
      self.ws.send_text(&message.dump()).unwrap();
    }
    if self.activated {
      set_timeout(
        move || rc.borrow().start_emit(rc.clone()),
        THROTTLE_TIME_MILIS as u32,
      );
    }
  }

  fn get_adapted_coordinates(&self) -> Coordinates {
    let x: f64 =
      self.coordinates.x as f64 / 100.0 / THROTTLE_TIME_MILIS as f64 * SENSITIVITY as f64;
    let y: f64 =
      self.coordinates.y as f64 / 100.0 / THROTTLE_TIME_MILIS as f64 * SENSITIVITY as f64;
    Coordinates {
      x: x as i32,
      y: y as i32,
    }
  }

  pub fn update(&mut self, x: i32, y: i32, should_emit: bool) {
    self.coordinates = Coordinates { x, y };
    self.should_emit = should_emit;
  }
}
