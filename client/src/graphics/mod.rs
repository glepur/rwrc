use std::cell::RefCell;
use std::rc::Rc;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::FillRule;
use stdweb::web::{document, window, CanvasRenderingContext2d, RequestAnimationFrameHandle};

const CENTER_RADIUS_RATIO: f64 = 0.11;
const CENTER_COLOR: &'static str = "#333";

const POINTER_LINE_WIDTH: f64 = 8.0;
const POINTER_COLOR: &'static str = "green";

pub struct Graphics {
  canvas: CanvasElement,
  center: Circle,
  pointer_circle: Circle,
  pointer_line: Line,
  request_animation_frame_handle: Option<RequestAnimationFrameHandle>,
}

struct Circle {
  coordinates: Coordinates,
  radius: f64,
  fill_color: Option<&'static str>,
  stroke_width: Option<f64>,
  stroke_color: Option<&'static str>,
}

struct Line {
  start: Coordinates,
  end: Coordinates,
}

#[derive(Copy, Clone)]
struct Coordinates {
  x: f64,
  y: f64,
}

impl Graphics {
  pub fn new() -> Self {
    stdweb::initialize();

    let canvas: CanvasElement = document()
      .query_selector("#canvas")
      .unwrap()
      .unwrap()
      .try_into()
      .unwrap();

    canvas.set_width(canvas.offset_width() as u32);
    canvas.set_height(canvas.offset_height() as u32);
    let center_radius = canvas.width() as f64 * CENTER_RADIUS_RATIO;
    let center = Coordinates {
      x: canvas.width() as f64 / 2.0,
      y: canvas.height() as f64 / 2.0 - center_radius,
    };

    Self {
      center: Circle {
        coordinates: center,
        radius: center_radius,
        fill_color: Some(CENTER_COLOR),
        stroke_color: None,
        stroke_width: None,
      },
      pointer_circle: Circle {
        coordinates: center,
        radius: 0.0,
        fill_color: None,
        stroke_color: Some(POINTER_COLOR),
        stroke_width: Some(POINTER_LINE_WIDTH),
      },
      pointer_line: Line {
        start: center,
        end: center,
      },
      canvas: canvas,
      request_animation_frame_handle: None,
    }
  }

  pub fn draw_center(&self) {
    self.draw_circle(&self.center);
  }

  pub fn draw_pointer(&self) {
    self.draw_circle(&self.pointer_circle);
    self.draw_line(self.pointer_line.start, self.pointer_line.end);
  }

  fn clear(&self) {
    let context: CanvasRenderingContext2d = self.canvas.get_context().unwrap();
    context.clear_rect(
      0.0,
      0.0,
      self.canvas.width() as f64,
      self.canvas.height() as f64,
    );
  }

  pub fn set_touch_coordinates(&mut self, x: f64, y: f64) {
    self.pointer_line.end = Coordinates { x, y };
    self.pointer_circle.radius = distance(&self.pointer_line.start, &self.pointer_line.end);
  }

  pub fn is_inside_center(&self, x: f64, y: f64) -> bool {
    distance(&self.center.coordinates, &Coordinates { x, y }) < self.center.radius
  }

  pub fn animate(&mut self, rc: Rc<RefCell<Self>>) {
    self.clear();
    self.draw_pointer();
    self.draw_center();
    self.request_animation_frame_handle =
      Some(window().request_animation_frame(move |_| rc.borrow_mut().animate(rc.clone())));
  }

  pub fn stop_animate(&mut self) {
    self.clear();
    self.draw_center();
    if self.request_animation_frame_handle.is_some() {
      self.request_animation_frame_handle.take().unwrap().cancel();
    };
  }

  fn draw_circle(&self, circle: &Circle) {
    let context: CanvasRenderingContext2d = self.canvas.get_context().unwrap();
    context.save();
    context.begin_path();
    context.arc(
      circle.coordinates.x,
      circle.coordinates.y,
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

  fn draw_line(&self, start: Coordinates, end: Coordinates) {
    let context: CanvasRenderingContext2d = self.canvas.get_context().unwrap();
    context.save();
    context.begin_path();
    context.set_line_width(POINTER_LINE_WIDTH);
    context.set_stroke_style_color(POINTER_COLOR);
    context.move_to(start.x, start.y);
    context.line_to(end.x, end.y);
    context.stroke();
    context.restore();
  }

  pub fn offset_from_center(&self) -> (f64, f64) {
    let (x, y) = get_offset(&self.center.coordinates, &self.pointer_line.end);
    (
      if x < 0.0 {
        x - self.center.radius
      } else {
        x + self.center.radius
      },
      if y < 0.0 {
        y - self.center.radius
      } else {
        y + self.center.radius
      },
    )
  }
}

fn get_offset(start: &Coordinates, end: &Coordinates) -> (f64, f64) {
  (end.x - start.x, end.y - start.y)
}

fn distance(start: &Coordinates, end: &Coordinates) -> f64 {
  let (x, y) = get_offset(&start, &end);
  (x.powf(2.0) + y.powf(2.0)).sqrt()
}