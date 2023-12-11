
use serde::Serialize;
use ratatui::{prelude::*, widgets::*};
use ratatui::widgets::canvas::*;

use std::f64::consts::PI;

use super::Colors;

#[derive(Default, Clone)]
pub struct StatefulList<T> {
  pub state: ListState,
  pub items: Vec<T>,
}

impl<T> StatefulList<T> {
  pub fn with_items(items: Vec<T>) -> StatefulList<T> {
      StatefulList {
          state: ListState::default(),
          items,
      }
  }

  pub fn next(&mut self) {
    if self.items.is_empty() {return;}
      let i = match self.state.selected() {
        Some(i) => {
            if i >= self.items.len() - 1 {
                0
            } else {
                i + 1
            }
        }
        None => 0,
      };
      //println!("next Item: {i}");
      self.state.select(Some(i));
  }

  pub fn previous(&mut self) {
    if self.items.is_empty() {return;}
    let i = match self.state.selected() {
        Some(i) => {
            if i == 0 {
                self.items.len() - 1
            } else {
                i - 1
            }
        }
        None => 0,
    };
    self.state.select(Some(i));
  }

  pub fn unselect(&mut self) {
    self.state.select(None);
  }

  pub fn trim_to_length(&mut self, max_length: usize) {
    while self.items.len() > max_length {
        self.items.remove(0);
    }
  }
}





#[derive(Default, Clone)]
pub struct Animation<T> {
  pub state: ListState,
  pub keyframes: Vec<T>,
}

impl<T> Animation<T> {
  pub fn with_items(keyframes: Vec<T>) -> Animation<T> {
      Animation {
          state: ListState::default(),
          keyframes,
      }
  }

  pub fn next(&mut self) {
    if self.keyframes.is_empty() {return;}
      let i = match self.state.selected() {
          Some(i) => {
              if i >= self.keyframes.len() - 1 {
                  0
              } else {
                  i + 1
              }
          }
          None => 0,
      };
      //println!("next Item: {i}");
      self.state.select(Some(i));
  }

  pub fn previous(&mut self) {
    if self.keyframes.is_empty() {return;}
      let i = match self.state.selected() {
          Some(i) => {
              if i == 0 {
                  self.keyframes.len() - 1
              } else {
                  i - 1
              }
          }
          None => 0,
      };
      self.state.select(Some(i));
  }

}


#[derive(Default, Clone)]
pub struct StyledLine {
  pub words: Vec<(String, Style)>,
}




#[derive(Default, Clone)]
pub struct DCube {
    bottom_left_front: (f64, f64),
    bottom_right_front: (f64, f64),
    top_left_front: (f64, f64),
    top_right_front: (f64, f64),
    bottom_left_back: (f64, f64),
    bottom_right_back: (f64, f64),
    top_left_back: (f64, f64),
    top_right_back: (f64, f64),
    rotation_angle: f64,
}

impl DCube {
    pub fn new(x: f64, y: f64, z: f64, size: f64) -> Self {
        let half_size = size / 2.0;

        DCube {
            bottom_left_front: (x - half_size, y - half_size),
            bottom_right_front: (x + half_size, y - half_size),
            top_left_front: (x - half_size, y + half_size),
            top_right_front: (x + half_size, y + half_size),
            bottom_left_back: (x - half_size, y - half_size),
            bottom_right_back: (x + half_size, y - half_size),
            top_left_back: (x - half_size, y + half_size),
            top_right_back: (x + half_size, y + half_size),
            rotation_angle: 0.0,
        }
    }

    pub fn rotate(&mut self, angle: f64, axis: char) {
      self.rotation_angle = angle;
      // Implement rotation based on the specified axis
      match axis {
          'x' => {
              // Rotate around the X-axis
              // Implement rotation logic for X-axis
              // ...
          }
          'y' => {
              // Rotate around the Y-axis
              // Implement rotation logic for Y-axis
              // ...
          }
          'z' => {
              // Rotate around the Z-axis
              let cos_angle = self.rotation_angle.cos();
              let sin_angle = self.rotation_angle.sin();

              let rotate_point = |(px, py): (f64, f64)| {
                  let new_px = cos_angle * px - sin_angle * py;
                  let new_py = sin_angle * px + cos_angle * py;
                  (new_px, new_py)
              };

              self.bottom_left_front = rotate_point(self.bottom_left_front);
              self.bottom_right_front = rotate_point(self.bottom_right_front);
              self.top_left_front = rotate_point(self.top_left_front);
              self.top_right_front = rotate_point(self.top_right_front);
              self.bottom_left_back = rotate_point(self.bottom_left_back);
              self.bottom_right_back = rotate_point(self.bottom_right_back);
              self.top_left_back = rotate_point(self.top_left_back);
              self.top_right_back = rotate_point(self.top_right_back);
          }
          _ => {
              // Handle invalid axis
              println!("Invalid axis specified for rotation");
          }
      }
  }


  pub fn draw_lines_orthographic(&self, ctx: &mut Context, colors: &Colors) {
    // Draw lines after applying orthographic projection
    // You can use the rotated coordinates here

    // Connect front face vertices
    ctx.draw(&canvas::Line {
        x1: self.bottom_left_front.0,
        y1: self.bottom_left_front.1,
        x2: self.bottom_right_front.0,
        y2: self.bottom_right_front.1,
        color: colors.color_a.color,
    });
    ctx.draw(&canvas::Line {
        x1: self.bottom_right_front.0,
        y1: self.bottom_right_front.1,
        x2: self.top_right_front.0,
        y2: self.top_right_front.1,
        color: colors.color_b.color,
    });
    ctx.draw(&canvas::Line {
        x1: self.top_right_front.0,
        y1: self.top_right_front.1,
        x2: self.top_left_front.0,
        y2: self.top_left_front.1,
        color: colors.color_c.color,
    });
    ctx.draw(&canvas::Line {
        x1: self.top_left_front.0,
        y1: self.top_left_front.1,
        x2: self.bottom_left_front.0,
        y2: self.bottom_left_front.1,
        color: colors.highlight.color,
    });

    // Connect back face vertices
    ctx.draw(&canvas::Line {
        x1: self.bottom_left_back.0,
        y1: self.bottom_left_back.1,
        x2: self.bottom_right_back.0,
        y2: self.bottom_right_back.1,
        color: colors.color_a.color,
    });
    ctx.draw(&canvas::Line {
        x1: self.bottom_right_back.0,
        y1: self.bottom_right_back.1,
        x2: self.top_right_back.0,
        y2: self.top_right_back.1,
        color: colors.color_b.color,
    });
    ctx.draw(&canvas::Line {
        x1: self.top_right_back.0,
        y1: self.top_right_back.1,
        x2: self.top_left_back.0,
        y2: self.top_left_back.1,
        color: colors.color_c.color,
    });
    ctx.draw(&canvas::Line {
        x1: self.top_left_back.0,
        y1: self.top_left_back.1,
        x2: self.bottom_left_back.0,
        y2: self.bottom_left_back.1,
        color: colors.highlight.color,
    });

    // Connect corresponding vertices between front and back faces
    ctx.draw(&canvas::Line {
        x1: self.bottom_left_front.0,
        y1: self.bottom_left_front.1,
        x2: self.bottom_left_back.0,
        y2: self.bottom_left_back.1,
        color: colors.highlight.flip_rgb(),
    });
    ctx.draw(&canvas::Line {
        x1: self.bottom_right_front.0,
        y1: self.bottom_right_front.1,
        x2: self.bottom_right_back.0,
        y2: self.bottom_right_back.1,
        color: colors.highlight.flip_rgb(),
    });
    ctx.draw(&canvas::Line {
        x1: self.top_left_front.0,
        y1: self.top_left_front.1,
        x2: self.top_left_back.0,
        y2: self.top_left_back.1,
        color: colors.highlight.flip_rgb(),
    });
    ctx.draw(&canvas::Line {
        x1: self.top_right_front.0,
        y1: self.top_right_front.1,
        x2: self.top_right_back.0,
        y2: self.top_right_back.1,
        color: colors.highlight.flip_rgb(),
    });
  }
}