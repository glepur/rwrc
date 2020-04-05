mod graphics;

use graphics::Graphics;
use std::cell::RefCell;
use std::rc::Rc;
use stdweb::traits::*;
use stdweb::web::event::{TouchEnd, TouchMove, TouchStart};
use stdweb::web::window;

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
  graphics.borrow().draw_center();
  graphics.borrow().draw_pointer();

  window().add_event_listener(enclose!( (graphics) move |event: TouchMove| {
    let touch = &event.touches()[0];
    graphics
      .borrow_mut()
      .set_touch_coordinates(touch.client_x() as f64, touch.client_y() as f64);
  }));

  window().add_event_listener(enclose!( (graphics) move |event: TouchStart| {
    let touch = &event.touches()[0];
    let distance_from_center = graphics.borrow().distance_from_center(touch.client_x() as f64, touch.client_y() as f64);
    if distance_from_center < graphics.borrow().center_radius() {
      graphics.borrow_mut().set_touch_coordinates(touch.client_x() as f64, touch.client_y() as f64);
      graphics.borrow_mut().animate(graphics.clone());
    }
  }));

  window().add_event_listener(enclose!( (graphics) move |_: TouchEnd| {
    graphics.borrow_mut().stop_animate();
  }));
}
