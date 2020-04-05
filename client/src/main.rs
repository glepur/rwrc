mod graphics;

use graphics::Graphics;
use std::cell::RefCell;
use std::rc::Rc;
use stdweb::traits::*;
use stdweb::web::event::{MouseDownEvent, MouseUpEvent, TouchMove};
use stdweb::web::window;

fn main() {
  let graphics_ref = Rc::new(RefCell::new(Graphics::new()));
  let graphics_ref_clone = graphics_ref.clone();
  graphics_ref.borrow().draw_center();
  graphics_ref.borrow().draw_pointer();

  window().add_event_listener(move |event: TouchMove| {
    let touch = &event.touches()[0];
    graphics_ref_clone.borrow().clear();
    graphics_ref_clone
      .borrow_mut()
      .set_touch_coordinates(touch.client_x() as f64, touch.client_y() as f64);
  });

  graphics_ref.borrow_mut().animate(graphics_ref.clone());

  stdweb::event_loop();
}
