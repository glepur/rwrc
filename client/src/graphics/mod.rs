mod button;
mod circle;
mod line;

use button::Button;
use circle::Circle;
use line::Line;
use std::cell::RefCell;
use std::rc::Rc;
use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, window, CanvasRenderingContext2d, RequestAnimationFrameHandle};

const CENTER_RADIUS_RATIO: f64 = 0.11;
const CENTER_COLOR: &'static str = "#333";

const POINTER_LINE_WIDTH: f64 = 8.0;
const POINTER_COLOR: &'static str = "green";

const BUTTON_COLOR: &'static str = "#333";
const BUTTON_WIDTH_RATIO: f64 = 0.3;
const BUTTON_HEIGHT_RATIO: f64 = 0.15;
const BUTTON_MARGIN_RATIO: f64 = 0.05;

pub enum Element {
  Center,
  ButtonLeft,
  ButtonRight,
}

pub struct Graphics {
  canvas: CanvasElement,
  center: Circle,
  pointer_circle: Circle,
  pointer_line: Line,
  button_left: Button,
  button_right: Button,
  request_animation_frame_handle: Option<RequestAnimationFrameHandle>,
}

#[derive(Copy, Clone)]
pub struct Coordinates {
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
    let canvas_center = Coordinates {
      x: canvas.width() as f64 / 2.0,
      y: canvas.height() as f64 / 2.0,
    };

    let mut center = canvas_center.clone();
    center.y -= center_radius;

    let button_margin = canvas.width() as f64 * BUTTON_MARGIN_RATIO;
    let button = Button {
      coordinates: Coordinates {
        x: 0.0,
        y: canvas_center.y + (button_margin * 2.0),
      },
      width: canvas.width() as f64 * BUTTON_WIDTH_RATIO,
      height: canvas.width() as f64 * BUTTON_HEIGHT_RATIO,
      color: BUTTON_COLOR,
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
        width: POINTER_LINE_WIDTH,
        color: POINTER_COLOR,
      },
      button_left: Button {
        coordinates: Coordinates {
          x: canvas_center.x - button.width - (button_margin / 2.0),
          y: button.coordinates.y,
        },
        ..button
      },
      button_right: Button {
        coordinates: Coordinates {
          x: canvas_center.x + (button_margin / 2.0),
          y: button.coordinates.y,
        },
        ..button
      },
      canvas: canvas,
      request_animation_frame_handle: None,
    }
  }

  pub fn draw_center(&self) {
    self.center.draw(&self.canvas);
  }

  pub fn draw_pointer(&self) {
    self.pointer_circle.draw(&self.canvas);
    self.pointer_line.draw(&self.canvas);
  }

  pub fn draw_buttons(&self) {
    self.button_left.draw(&self.canvas);
    self.button_right.draw(&self.canvas);
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

  fn is_inside_center(&self, x: f64, y: f64) -> bool {
    distance(&self.center.coordinates, &Coordinates { x, y }) < self.center.radius
  }

  pub fn animate(&mut self, rc: Rc<RefCell<Self>>) {
    self.clear();
    self.draw_buttons();
    self.draw_pointer();
    self.draw_center();
    self.request_animation_frame_handle =
      Some(window().request_animation_frame(move |_| rc.borrow_mut().animate(rc.clone())));
  }

  pub fn stop_animate(&mut self) {
    self.clear();
    self.draw_center();
    self.draw_buttons();
    if self.request_animation_frame_handle.is_some() {
      self.request_animation_frame_handle.take().unwrap().cancel();
    };
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

  pub fn element_hit(&self, x: f64, y: f64) -> Option<Element> {
    if self.is_inside_center(x, y) {
      return Some(Element::Center);
    } else if self.button_left.contains(x, y) {
      return Some(Element::ButtonLeft);
    } else if self.button_right.contains(x, y) {
      return Some(Element::ButtonRight);
    }
    None
  }
}

fn get_offset(start: &Coordinates, end: &Coordinates) -> (f64, f64) {
  (end.x - start.x, end.y - start.y)
}

fn distance(start: &Coordinates, end: &Coordinates) -> f64 {
  let (x, y) = get_offset(&start, &end);
  (x.powf(2.0) + y.powf(2.0)).sqrt()
}
