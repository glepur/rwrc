use stdweb::console;

pub struct Transmitter {
  activated: bool,
}

impl Transmitter {
  pub fn new() -> Self {
    Transmitter { activated: false }
  }
  pub fn activate(&mut self) {
    self.activated = true;
  }

  pub fn deactivate(&mut self) {
    self.activated = false;
  }

  pub fn send(&self, x: f64, y: f64) {
    if self.activated {
      console!(log, "{},{}", x, y);
    }
  }
}
