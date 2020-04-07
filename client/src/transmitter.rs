use instant::Instant;
use stdweb::console;

const THROTTLE_TIME_MILIS: u128 = 100;

pub struct Transmitter {
  activated: bool,
  last_called: Instant,
}

impl Transmitter {
  pub fn new() -> Self {
    Transmitter {
      activated: false,
      last_called: Instant::now(),
    }
  }

  pub fn activate(&mut self) {
    self.activated = true;
  }

  pub fn deactivate(&mut self) {
    self.activated = false;
  }

  pub fn send(&mut self, x: f64, y: f64) {
    if self.activated && self.last_called.elapsed().as_millis() > THROTTLE_TIME_MILIS {
      self.last_called = Instant::now();
      console!(log, "{},{}", x, y);
    }
  }
}
