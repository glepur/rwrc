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

  let ws_host = window().location().unwrap().host().unwrap();
  let transmitter = Rc::new(RefCell::new(Transmitter::new(&ws_host)));

  window().add_event_listener(enclose!( (graphics, transmitter) move |event: TouchMove| {
    let touch = &event.touches()[0];
    let (x, y) = get_touch_coordinates(touch);
    graphics
      .borrow_mut()
      .set_touch_coordinates(x, y);
    let (dx, dy) = graphics.borrow().offset_from_center();
    let should_emit = !graphics.borrow().is_inside_center(x, y);
    transmitter.borrow_mut().update(dx, dy, should_emit);
  }));

  window().add_event_listener(enclose!( (graphics, transmitter) move |event: TouchStart| {
    let mut graphics_m = graphics.borrow_mut();
    let touch = &event.touches()[0];
    let (x, y) = get_touch_coordinates(touch);
    let is_inside_center = graphics_m.is_inside_center(x, y);
    if is_inside_center {
      transmitter.borrow_mut().update(x, y, !is_inside_center);
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
