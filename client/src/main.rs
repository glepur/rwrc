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
  let graphics_i = graphics.borrow();
  graphics_i.draw_center();
  graphics_i.draw_pointer();

  window().add_event_listener(enclose!( (graphics) move |event: TouchMove| {
    let touch = &event.touches()[0];
    graphics
      .borrow_mut()
      .set_touch_coordinates(touch.client_x() as f64, touch.client_y() as f64);
  }));

  window().add_event_listener(enclose!( (graphics) move |event: TouchStart| {
    let mut graphics_m = graphics.borrow_mut();
    let touch = &event.touches()[0];
    let x = touch.client_x() as f64;
    let y = touch.client_y() as f64;
    if graphics_m.should_animate(x, y) {
      graphics_m.set_touch_coordinates(x, y);
      graphics_m.animate(graphics.clone());
    }
  }));

  window().add_event_listener(enclose!( (graphics) move |_: TouchEnd| {
    graphics.borrow_mut().stop_animate();
  }));
}
