use super::Coordinates;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::CanvasRenderingContext2d;
use stdweb::web::FillRule;

pub struct Button {
  pub coordinates: Coordinates,
  pub width: f64,
  pub height: f64,
  pub color: &'static str,
}

impl Button {
  pub fn draw(&self, canvas: &CanvasElement) {
    let context: CanvasRenderingContext2d = canvas.get_context().unwrap();
    context.save();
    context.begin_path();
    context.rect(
      self.coordinates.x,
      self.coordinates.y,
      self.width,
      self.height,
    );
    context.set_fill_style_color(self.color);
    context.fill(FillRule::default());
    context.stroke();
    context.restore();
  }

  pub fn contains(&self, x: f64, y: f64) -> bool {
    x > self.coordinates.x
      && x < self.coordinates.x + self.width
      && y > self.coordinates.y
      && y < self.coordinates.y + self.height
  }
}
