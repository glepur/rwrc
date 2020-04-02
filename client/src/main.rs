use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::event::{MouseDownEvent, MouseUpEvent, ResizeEvent, TouchMove};
use stdweb::web::html_element::CanvasElement;
use stdweb::web::FillRule;
use stdweb::web::{document, window, CanvasRenderingContext2d};

const CENTER_RADIUS_RATIO: f64 = 0.15;
const CENTER_COLOR: &'static str = "#333";

const POINTER_LINE_WIDTH: f64 = 8.0;
const POINTER_COLOR: &'static str = "green";

#[derive(Copy, Clone)]
struct Circle {
  x: f64,
  y: f64,
  radius: f64,
  fill_color: Option<&'static str>,
  stroke_width: Option<f64>,
  stroke_color: Option<&'static str>,
}

struct Coordinates {
  x: f64,
  y: f64,
}

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
  let context: CanvasRenderingContext2d = canvas.get_context().unwrap();

  canvas.set_width(canvas.offset_width() as u32);
  canvas.set_height(canvas.offset_height() as u32);
  let x = canvas.width() as f64 / 2.0;
  let y = canvas.height() as f64 / 2.0;

  let mut center = Circle {
    x,
    y,
    radius: canvas.width() as f64 * CENTER_RADIUS_RATIO,
    fill_color: Some(CENTER_COLOR),
    stroke_color: None,
    stroke_width: None,
  };

  let mut pointer = Circle {
    x,
    y,
    radius: 0.0,
    fill_color: None,
    stroke_color: Some(POINTER_COLOR),
    stroke_width: Some(POINTER_LINE_WIDTH),
  };

  draw_circle(&context, &center);
  draw_circle(&context, &pointer);

  window().add_event_listener(enclose!( (canvas) move |event: TouchMove| {
    context.clear_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);
    let touch = &event.touches()[0];
    let start = Coordinates {
      x: canvas.width() as f64 / 2.0,
      y: canvas.height() as f64 / 2.0
    };
    let end = Coordinates {
      x: touch.client_x() as f64,
      y: touch.client_y() as f64
    };
    pointer.radius = distance(&start, &end);
    draw_line(&context, &start, &end);
    draw_circle(&context, &pointer);
    draw_circle(&context, &center);
  }));

  stdweb::event_loop();
}

fn draw_circle(context: &CanvasRenderingContext2d, circle: &Circle) {
  context.save();
  context.begin_path();
  context.arc(
    circle.x,
    circle.y,
    circle.radius,
    0.0,
    2.0 * std::f64::consts::PI,
    false,
  );
  if let Some(color) = circle.fill_color {
    context.set_fill_style_color(color);
    context.fill(FillRule::default());
  }
  if let Some(stroke_width) = circle.stroke_width {
    context.set_line_width(stroke_width);
  }
  if let Some(stroke_color) = circle.stroke_color {
    context.set_stroke_style_color(stroke_color);
  }
  context.stroke();
  context.restore();
}

fn draw_line(context: &CanvasRenderingContext2d, start: &Coordinates, end: &Coordinates) {
  context.save();
  context.begin_path();
  context.set_line_width(POINTER_LINE_WIDTH);
  context.set_stroke_style_color(POINTER_COLOR);
  context.move_to(start.x, start.y);
  context.line_to(end.x, end.y);
  context.stroke();
  context.restore();
}

fn distance(start: &Coordinates, end: &Coordinates) -> f64 {
  let abs_x = if start.x < end.x {
    (end.x - start.x).abs()
  } else {
    (start.x - end.x).abs()
  };
  let abs_y = if start.y < end.y {
    end.y - start.y
  } else {
    start.y - end.y
  };
  (abs_x.powf(2.0) + abs_y.powf(2.0)).sqrt()
}
