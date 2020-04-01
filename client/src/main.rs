use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{MouseMoveEvent, ResizeEvent};
use stdweb::web::html_element::CanvasElement;
use stdweb::web::FillRule;
use stdweb::web::{document, window, CanvasRenderingContext2d};

const CENTER_RADIUS: f64 = 50.0;
const CENTER_COLOR: &'static str = "#333";

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = $x.clone();)*
            $y
        }
    };
}

fn main() {
  stdweb::initialize();

  let canvas: CanvasElement = document()
    .query_selector("#canvas")
    .unwrap()
    .unwrap()
    .try_into()
    .unwrap();

  canvas.set_width(canvas.offset_width() as u32);
  canvas.set_height(canvas.offset_height() as u32);
  draw_center(&canvas);

  window().add_event_listener(enclose!( (canvas) move |_: ResizeEvent| {
      canvas.set_width(canvas.offset_width() as u32);
      canvas.set_height(canvas.offset_height() as u32);
      draw_center(&canvas);
  }));

  stdweb::event_loop();
}

fn draw_center(canvas: &CanvasElement) {
  let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
  context.begin_path();
  context.arc(
    canvas.width() as f64 / 2.0,
    canvas.height() as f64 / 2.0,
    CENTER_RADIUS,
    0.0,
    2.0 * std::f64::consts::PI,
    false,
  );
  context.set_fill_style_color(CENTER_COLOR);
  context.fill(FillRule::default());
  context.stroke();
}
