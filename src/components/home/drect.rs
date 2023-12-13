
use super::*;

#[derive(Default, Clone)]
pub struct DRect {
  pub bottom_left: (f64, f64),
  pub bottom_right: (f64, f64),
  pub top_left: (f64, f64),
  pub top_right: (f64, f64),
  pub origin: (f64, f64),
}

impl DRect {
  pub fn new(x: f64, y: f64, w:f64, h:f64) -> Self
  {
    DRect{
      bottom_left: (x, y),
      bottom_right: (x+w, y),
      top_left: (x, y+h),
      top_right: (x+w, y+h),
      origin: (x+w/2., y+h/2.),
    }
  }

  pub fn rot(&self, ang: f64) -> Self {
    let (cos_ang, sin_ang) = ang.cos_sin();

    let rotate_point = |(px, py): (f64, f64)| {
        let px_rel = px - self.origin.0;
        let py_rel = py - self.origin.1;

        let new_px_rel = cos_ang * px_rel - sin_ang * py_rel;
        let new_py_rel = sin_ang * px_rel + cos_ang * py_rel;

        (self.origin.0 + new_px_rel, self.origin.1 + new_py_rel)
    };

    DRect {
        bottom_left: rotate_point(self.bottom_left),
        bottom_right: rotate_point(self.bottom_right),
        top_left: rotate_point(self.top_left),
        top_right: rotate_point(self.top_right),
        origin: self.origin,
    }    
  }

  pub fn draw_lines(&self, ctx: &mut canvas::Context, colors: &Colors) {
    // Draw Rect
    // Bottom Left to Top Left
    ctx.draw(&canvas::Line {
        x1: self.bottom_left.0,
        y1: self.bottom_left.1,
        x2: self.top_left.0,
        y2: self.top_left.1,
        color: colors.color_a.color,
    });
    // Top Left to Top Right
    ctx.draw(&canvas::Line {
        x1: self.top_left.0,
        y1: self.top_left.1,
        x2: self.top_right.0,
        y2: self.top_right.1,
        color: colors.color_b.color,
    });
    // Top Right to Bottom Right
    ctx.draw(&canvas::Line {
        x1: self.top_right.0,
        y1: self.top_right.1,
        x2: self.bottom_right.0,
        y2: self.bottom_right.1,
        color: colors.color_c.color,
    });
    // Bottom Right to Bottom Left
    ctx.draw(&canvas::Line {
        x1: self.bottom_right.0,
        y1: self.bottom_right.1,
        x2: self.bottom_left.0,
        y2: self.bottom_left.1,
        color: colors.highlight.color,
    });
    // Bottom Left to Top Right
    ctx.draw(&canvas::Line {
        x1: self.bottom_left.0,
        y1: self.bottom_left.1,
        x2: self.top_right.0,
        y2: self.top_right.1,
        color: colors.highlight.flip_rgb(),
    });
  }


  pub fn fill_rect_with_points(
    &self,
    painter: &mut canvas::Painter,
    rect: &Rect,
    color: Color,
    ) {
    for y in rect.y..rect.y + rect.height {
        for x in rect.x..rect.x + rect.width {
            // Map the x, y coordinates within the Rect to the DRect
            let mapped_point = self.map_rect_to_drect(x, y, rect.width, rect.height);

            // Paint the point with the specified color and marker type
            painter.paint(mapped_point.0 as usize, mapped_point.1 as usize, color);
        }
    }
  }
  pub fn fill_rect_with_points_dumb(
    &self,
    painter: &mut canvas::Painter,
    rect: &Rect,
    color: Color,
    increment: f64,
  ) {
    let mut x = self.bottom_left.0;
    let mut y = self.bottom_left.1;

    while x <= self.bottom_right.0 && x <= rect.width as f64 {
        while y <= self.top_left.1 && y <= rect.height as f64 {
            painter.paint(x as usize, y as usize, color);
            y += increment;
        }
        x += increment;
        y = self.bottom_left.1;
    }
  }


/// Does not work correctly, but looks fun
pub fn map_rect_to_drect(&self, x: u16, y: u16, width: u16, height: u16) -> (f64, f64) {
    // Map x, y to coordinates within the DRect
    let mapped_x = self.bottom_left.0
        + (x as f64 / width as f64) * (self.bottom_right.0 - self.bottom_left.0);
    let mapped_y = self.bottom_left.1
        + (y as f64 / height as f64) * (self.top_left.1 - self.bottom_left.1);

    (mapped_x, mapped_y)
}

}
trait TrigExt {
  fn cos_sin(&self) -> (f64, f64);
}

impl TrigExt for f64 {
  fn cos_sin(&self) -> (f64, f64) {
      (self.cos(), self.sin())
  }
}


