
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

#[derive(Default, Clone, PartialEq, Eq, Debug)]
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
  /// 
  /// Will return Error for colors that don't return a Hex !
  pub fn from_color(color: Color) -> Result<Self, Error> {
    Self::from_hex(&color.to_string())
  }

  pub fn flip_rgb(&self) -> Color {
    let r = u8::MAX - self.r;
    let g = u8::MAX - self.g;
    let b = u8::MAX - self.b;
    Color::Rgb(r, g, b)
  }


  /// Classical forward conversion (from RGB to HSV) requires the following steps:
  /// 1. Find maximum (M) and minimum (m) of R; G, and B.
  /// M = max(R; G; B)
  /// m = min(R; G; B)
  /// 2. Assign V = M.
  /// 3. Calculate delta (d) between M and m.
  ///   d = M - m
  /// 4. If d is equal to 0 then assign S with 0 and return. H is undefined in this case.
  /// 5. Calculate S as a ratio of d and M.
  ///   S = d / M
  /// 6. Calculate H depending on what are M and m. - based on hue being between 0. and 1.
  /// 
  /// Ref: https://doi.org/10.1016/j.compeleceng.2015.08.005.
  /// 
  /// Ref: https://dl.acm.org/doi/abs/10.1145/965139.807361

  pub fn rgb_to_hsv(&self) -> (f64, f64, f64) {
    let r = f64::from(self.r) / 255.0;
    let g = f64::from(self.g) / 255.0;
    let b = f64::from(self.b) / 255.0;
    // 1.
    let c_max = r.max(g).max(b);
    let c_min = r.min(g).min(b);

    // 2.
    let value = c_max;
    // 3.
    let delta = c_max - c_min;
    // 4.
    let saturation = if c_max.abs() < f64::EPSILON {
      0.0
    } else {
      // 5.
        delta / c_max
    };

    // 6. is different due to needing to convert to degrees
    let hue = if delta.abs() < f64::EPSILON {
        0.0
    } else if c_max == r {
        60.0 * ((g - b) / delta % 6.0)
    } else if c_max == g {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };

    
    round_hsv((hue, saturation, value))

  }


  /// Ref: https://en.wikipedia.org/wiki/HSL_and_HSV#Color_conversion_formulae
  /// 
  /// Lossy !
  /// 
  /// Same behaviour as https://lib.rs/crates/hsv
  pub fn from_hsv(hsv: (f64, f64, f64)) -> Self {

    let (hue, saturation, value) = round_hsv(hsv);


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

/// rounds hue to nearest integer and saturation / value to three decimal places
pub fn round_hsv(hsv: (f64, f64, f64)) -> (f64, f64, f64) {
  let mut h = hsv.0;
  let mut s = hsv.1;
  let mut v = hsv.2;
  // round degrees to nearest integer
  let _dhue = h - h.floor();
  if _dhue >= 0.5 {
    h = h.ceil();
  } else {
    h = h.floor();
  }
  // round sat & val to three decimal places
  s = (s * 1000.0).round() / 1000.0;
  v = (v * 1000.0).round() / 1000.0;

  (h,s,v)
}


fn normalize_chan(x: u8) -> f32 {
    x as f32 / u8::MAX as f32
}
fn linearize_chan(x:f32) -> f32 {
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



#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;

  use super::*;


  #[test]
  fn test_output_hex_from_rgb() {
    // https://www.rapidtables.com/convert/color/rgb-to-hsv.html
    // hex: #00EEEC
    // rgb: 0, 238, 236
    // hsv: 179° 100% 93.3%
    
    let rgb: (u8, u8, u8) = (0, 238, 236);
    let color = ColorRGB::new(rgb.0, rgb.1, rgb.2);
    let hex = color.color.to_string();

    assert_eq!(hex, "#00EEEC".to_string());
  }  
  #[test]
  fn test_output_hex_from_hex() {
    // https://www.rapidtables.com/convert/color/rgb-to-hsv.html
    // hex: #00EEEC
    // rgb: 0, 238, 236
    // hsv: 179° 100% 93.3%

    let color = ColorRGB::from_hex("#00EEEC").unwrap();
    let hex = color.color.to_string();

    assert_eq!(hex, "#00EEEC".to_string());
  } 

  #[test]
  fn test_output_rgb_from_hsv() {
    // https://www.rapidtables.com/convert/color/rgb-to-hsv.html
    // hex: #00EEEC
    // rgb: 0, 238, 236
    // hsv: 179° 100% 93.3%

    let color = ColorRGB::from_hsv((179.0, 1.0, 0.933));
    assert_eq!(format!("{}, {}, {}", color.r, color.g, color.b), "0, 238, 236".to_string());
  }


  #[test]
  fn test_output_hex_from_hsv() {
    // https://www.rapidtables.com/convert/color/rgb-to-hsv.html
    // hex: #00EEEC
    // rgb: 0, 238, 236
    // hsv: 179° 100% 93.3%

    let color = ColorRGB::from_hsv((179.0, 1.0, 0.933));
    let hex = color.color.to_string();

    assert_eq!(hex, "#00EEEC".to_string());
  }

  #[test]
  fn test_output_hsv_from_rgb() {
    // https://www.rapidtables.com/convert/color/rgb-to-hsv.html
    // hex: #00EEEC
    // rgb: 0, 238, 236
    // hsv: 179° 100% 93.3%
    
    let rgb: (u8, u8, u8) = (0, 238, 236);
    let color = ColorRGB::new(rgb.0, rgb.1, rgb.2);
    let hsv = color.rgb_to_hsv();

    assert_eq!(hsv, (179.0, 1.0, 0.933));
  }  
  #[test]
  fn test_output_hsv_from_hex() {
    // https://www.rapidtables.com/convert/color/rgb-to-hsv.html
    // hex: #00EEEC
    // rgb: 0, 238, 236
    // hsv: 179° 100% 93.3%

    let color = ColorRGB::from_hex("#00EEEC").unwrap();
    let hsv = color.rgb_to_hsv();

    assert_eq!(hsv, (179.0, 1.0, 0.933));
  } 
  
  #[test]
  fn test_output_hsv_from_hsv() {
    // https://www.rapidtables.com/convert/color/rgb-to-hsv.html
    // hex: #00EEEC
    // rgb: 0, 238, 236
    // hsv: 179° 100% 93.3%

    let color = ColorRGB::from_hsv((179.0, 1.0, 0.933)); // this method has precision loss

    let hsv = color.rgb_to_hsv();

    assert_eq!(hsv, (179.0, 1.0, 0.933));
  }


  #[test]
  fn test_output_hsv() {
    // https://www.rapidtables.com/convert/color/rgb-to-hsv.html
    // hex: #00EEEC
    // rgb: 0, 238, 236
    // hsv: 179° 100% 93.3%
    
    let rgb: (u8, u8, u8) = (0, 238, 236);
    let color = ColorRGB::new(rgb.0, rgb.1, rgb.2);

    let hsv = color.rgb_to_hsv();

    assert_eq!(format!("{:.0}, {:.2}, {:.3}", hsv.0, hsv.1, hsv.2), format!("179, 1.00, 0.933"));
  }  

  #[test]
  fn test_parse_rgb_hex_rgb() {
    let rgb: (u8, u8, u8) = (144, 76, 98);
    let color = ColorRGB::new(rgb.0, rgb.1, rgb.2);
    let hex = color.color.to_string();
    let fromhex = ColorRGB::from_hex(&hex).unwrap();
    let _rgb = (fromhex.r, fromhex.g, fromhex.b);

    assert_eq!(rgb, _rgb);
  }

  #[test]
  fn test_parse_rgb_hsv_rgb() {
    let rgb: (u8, u8, u8) = (144, 76, 98);
    let color = ColorRGB::new(rgb.0, rgb.1, rgb.2);
    let hsv = color.rgb_to_hsv();
    let _color = ColorRGB::from_hsv(hsv);
    let _rgb = (_color.r, _color.g, _color.b);

    assert_eq!(rgb, _rgb);
  }  



}

