
use ratatui::prelude::Color;
use std::{str::FromStr, fmt::Error};

#[derive(Default, Clone)]
pub struct Colors {
  pub background: ColorRGB,
  pub color_a: ColorRGB,
  pub color_b: ColorRGB,
  pub color_c: ColorRGB,
  pub highlight: ColorRGB,
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct ColorRGB {
  pub color: Color,
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

impl ColorRGB {
  pub fn new(r: u8, g: u8, b: u8) -> Self {
    let color = Color::Rgb(r, g, b);
    ColorRGB { color, r, g, b }
  }
  /// Creates a shaded variant of the passed color.
  /// frac is clamped between -1 and 1 -> -100% brightness to +100% brightness.
  pub fn shade(&self, frac: f32) -> Color {
    if frac < 0. {
      // darken
      let r = darken_channel(self.r, frac);
      let g = darken_channel(self.g, frac);
      let b = darken_channel(self.b, frac);
      Color::Rgb(r, g, b)
    } else {
      // brighten
      let r = brighten_channel(self.r, frac);
      let g = brighten_channel(self.g, frac);
      let b = brighten_channel(self.b, frac);
      Color::Rgb(r, g, b)
    }
  }

  /// Creates a new ColorRGB from a hex string, returns Error on invalid input.
  /// TODO: Include all ratatui colors that dont return a valid hex representation
  pub fn from_hex(s: &str) -> Result<Self, Error> { 
    match s
    .to_lowercase()
    .replace([' ', '-', '_'], "")
    .as_ref()
    {
      "white" => {Ok(Self::new(255,255,255))},
      "lightblue" => {Ok(Self::new(48,48,240))},
      _ => { if let (Ok(r), Ok(g), Ok(b)) = {
        if !s.starts_with('#') || s.len() != 7 {
            return Err(Error);
        }
          (
              u8::from_str_radix(&s[1..3], 16),
              u8::from_str_radix(&s[3..5], 16),
              u8::from_str_radix(&s[5..7], 16),
          )
        } {
            Ok(Self::new(r, g, b))
        } else {
            return Err(Error);
        }

      }
    }

  }

  /// Creates a new ColorRGB from a ratatui Color via its hex representation 
  pub fn from_color(color: Color) -> Result<Self, Error> {
    Self::from_hex(&color.to_string())
  }

  pub fn flip_rgb(&self) -> Color {
    let r = u8::MAX - self.r;
    let g = u8::MAX - self.g;
    let b = u8::MAX - self.b;
    Color::Rgb(r, g, b)
  }


  pub fn rgb_to_hsv(&self) -> (f64, f64, f64) {
    let r = f64::from(self.r) / 255.0;
    let g = f64::from(self.g) / 255.0;
    let b = f64::from(self.b) / 255.0;

    let c_max = r.max(g).max(b);
    let c_min = r.min(g).min(b);
    let delta = c_max - c_min;

    let hue = if delta.abs() < f64::EPSILON {
        0.0
    } else if c_max == r {
        60.0 * ((g - b) / delta % 6.0)
    } else if c_max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };

    let saturation = if c_max.abs() < f64::EPSILON {
        0.0
    } else {
        delta / c_max
    };

    let value = c_max;

    (hue, saturation, value)
  }

  pub fn from_hsv(hsv: (f64, f64, f64)) -> Self {
    let (hue, saturation, value) = hsv;

    let c = value * saturation;
    let x = c * (1.0 - ((hue / 60.0) % 2.0 - 1.0).abs());
    let m = value - c;

    let (r, g, b) = if hue < 60.0 {
        (c, x, 0.0)
    } else if hue < 120.0 {
        (x, c, 0.0)
    } else if hue < 180.0 {
        (0.0, c, x)
    } else if hue < 240.0 {
        (0.0, x, c)
    } else if hue < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    ColorRGB {
        color: Color::Rgb( ((r + m) * 255.0) as u8,
                            ((g + m) * 255.0) as u8,
                            ((b + m) * 255.0) as u8),
        r: ((r + m) * 255.0) as u8,
        g: ((g + m) * 255.0) as u8,
        b: ((b + m) * 255.0) as u8,
    }
  }


  pub fn with_hue(&self, new_hue: f64) -> Self {
    let (_, s, v) = self.rgb_to_hsv();
    ColorRGB::from_hsv((new_hue, s, v))
  }

  pub fn with_saturation(&self, new_saturation: f64) -> Self {
    let (h, _, v) = self.rgb_to_hsv();
    ColorRGB::from_hsv((h, new_saturation, v))
  }

  pub fn with_value(&self, new_value: f64) -> Self {
    let (h, s, _) = self.rgb_to_hsv();
    ColorRGB::from_hsv((h, s, new_value))
  }

  pub fn shift_hue(&self, amount: f64) -> Self {
    let (h, s, v) = self.rgb_to_hsv();
    ColorRGB::from_hsv(((h + amount) % 360.0, s, v))
  }

  pub fn shift_saturation(&self, amount: f64) -> Self {
    let (h, s, v) = self.rgb_to_hsv();
    ColorRGB::from_hsv((h, (s + amount).min(1.0).max(0.0), v))
  }

  pub fn shift_value(&self, amount: f64) -> Self {
    let (h, s, v) = self.rgb_to_hsv();
    ColorRGB::from_hsv((h, s, (v + amount).min(1.0).max(0.0)))
  }

}


pub fn normalize_chan(x: u8) -> f32 {
    x as f32 / u8::MAX as f32
}
pub fn linearize_chan(x:f32) -> f32 {
    x.powf(2.2)
}
pub fn get_luminance(x: &ColorRGB) -> f32 {
    let _r = linearize_chan(normalize_chan(x.r));
    let _g = linearize_chan(normalize_chan(x.g));
    let _b = linearize_chan(normalize_chan(x.b));

    _r * 0.2126 + _g * 0.7152 + _b * 0.0722
}

// get the contrast between two colors
pub fn get_contrast(x: &ColorRGB, y: &ColorRGB) -> f32 {
    let lx = get_luminance(x);
    let ly = get_luminance(y);

    if lx < ly {
        // x luminance is lower, so it is darker
        (ly- lx) / (ly + 0.1)
    } else {
        (lx- ly) / (lx + 0.1)
    }
    // https://stackoverflow.com/questions/56198778/what-is-the-efficient-way-to-calculate-human-eye-contrast-difference-for-rgb-val/56200738#56200738   
}


pub fn brighten_channel(x: u8, inc: f32) -> u8 {
  let mut inc = inc;
  if inc <= 0. {inc = inc.abs();};
  if inc >= 1. {return u8::MAX;};
  let mut _x = u8::MAX - x;
  let mut __x: f32 = _x as f32;
  __x = __x * inc;
  _x = __x as u8;
  x.saturating_add(_x)
}

pub fn darken_channel(x: u8, dec: f32) -> u8 {
  let mut dec = dec;
  if dec <= 0. { dec = dec.abs();};
  if dec >= 1. { return u8::MIN;};
  let mut _x: f32 = x as f32;
  _x = _x * dec;
  let __x = _x as u8;
  x.saturating_sub(__x)
}
