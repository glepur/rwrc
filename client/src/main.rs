mod graphics;

use graphics::Graphics;
use stdweb::traits::*;
use stdweb::web::event::{MouseDownEvent, MouseUpEvent, TouchMove};
use stdweb::web::window;

fn main() {
  let mut graphics = Graphics::new();
  graphics.draw_center();
  graphics.draw_pointer();

  window().add_event_listener(move |event: TouchMove| {
    let touch = &event.touches()[0];
    graphics.clear();
    graphics.set_touch_coordinates(touch.client_x() as f64, touch.client_y() as f64);
    graphics.draw_pointer();
    graphics.draw_center();
  });

  stdweb::event_loop();
}
