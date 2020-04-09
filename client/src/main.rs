mod graphics;
mod transmitter;

use graphics::Graphics;
use std::cell::RefCell;
use std::rc::Rc;
use stdweb::traits::*;
use stdweb::web::event::{TouchEnd, TouchMove, TouchStart};
use stdweb::web::{window, Touch};
use transmitter::Transmitter;

macro_rules! enclose {
  ( ($( $x:ident ),*) $y:expr ) => {
    {
      $(let $x = $x.clone();)*
      $y
    }
  };
}

fn main() {
  let graphics = Rc::new(RefCell::new(Graphics::new()));
  let graphics_i = graphics.borrow();
  graphics_i.draw_center();
  graphics_i.draw_pointer();

  let transmitter = Rc::new(RefCell::new(Transmitter::new()));

  window().add_event_listener(enclose!( (graphics, transmitter) move |event: TouchMove| {
    let touch = &event.touches()[0];
    let (x, y) = get_touch_coordinates(touch);
    graphics
      .borrow_mut()
      .set_touch_coordinates(x, y);
    let (x, y) = graphics.borrow().offset_from_center();
    transmitter.borrow_mut().update(x, y);
  }));

  window().add_event_listener(enclose!( (graphics, transmitter) move |event: TouchStart| {
    let mut graphics_m = graphics.borrow_mut();
    let touch = &event.touches()[0];
    let (x, y) = get_touch_coordinates(touch);
    if graphics_m.should_animate(x, y) {
      transmitter.borrow_mut().update(x, y);
      transmitter.borrow_mut().activate();
      transmitter.borrow().start_emit(transmitter.clone());
      graphics_m.set_touch_coordinates(x, y);
      graphics_m.animate(graphics.clone());
    }
  }));

  window().add_event_listener(enclose!( (graphics, transmitter) move |_: TouchEnd| {
    transmitter.borrow_mut().deactivate();
    graphics.borrow_mut().stop_animate();
  }));
}

fn get_touch_coordinates(touch: &Touch) -> (f64, f64) {
  (touch.client_x() as f64, touch.client_y() as f64)
}
