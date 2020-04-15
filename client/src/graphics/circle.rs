use super::Coordinates;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::CanvasRenderingContext2d;
use stdweb::web::FillRule;

pub struct Circle {
  pub coordinates: Coordinates,
  pub radius: f64,
  pub fill_color: Option<&'static str>,
  pub stroke_width: Option<f64>,
  pub stroke_color: Option<&'static str>,
}

impl Circle {
  pub fn draw(&self, canvas: &CanvasElement) {
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    context.save();
    context.begin_path();
    context.arc(
      self.coordinates.x,
      self.coordinates.y,
      self.radius,
      0.0,
      2.0 * std::f64::consts::PI,
      false,
    );
    if let Some(color) = self.fill_color {
      context.set_fill_style_color(color);
      context.fill(FillRule::default());
    }
    if let Some(stroke_width) = self.stroke_width {
      context.set_line_width(stroke_width);
    }
    if let Some(stroke_color) = self.stroke_color {
      context.set_stroke_style_color(stroke_color);
    }
    context.stroke();
    context.restore();
  }
}
