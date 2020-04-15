use super::Coordinates;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::CanvasRenderingContext2d;

pub struct Line {
  pub start: Coordinates,
  pub end: Coordinates,
  pub width: f64,
  pub color: &'static str,
}

impl Line {
  pub fn draw(&self, canvas: &CanvasElement) {
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    context.save();
    context.begin_path();
    context.set_line_width(self.width);
    context.set_stroke_style_color(self.color);
    context.move_to(self.start.x, self.start.y);
    context.line_to(self.end.x, self.end.y);
    context.stroke();
    context.restore();
  }
}
