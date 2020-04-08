use std::cell::RefCell;
use std::rc::Rc;
use stdweb::console;
use stdweb::web::set_timeout;

const THROTTLE_TIME_MILIS: u32 = 100;

pub struct Transmitter {
  activated: bool,
  coordinates: Coordinates,
}

struct Coordinates {
  x: f64,
  y: f64,
}

impl Transmitter {
  pub fn new() -> Self {
    Transmitter {
      activated: false,
      coordinates: Coordinates { x: 0.0, y: 0.0 },
    }
  }

  pub fn activate(&mut self) {
    self.activated = true;
  }

  pub fn deactivate(&mut self) {
    self.activated = false;
  }

  pub fn start_emit(&self, rc: Rc<RefCell<Transmitter>>) {
    let Coordinates { x, y } = rc.borrow().coordinates;
    console!(log, "{},{}", x, y);
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
