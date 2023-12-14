
use crate::colors::{ColorRGB, Colors};

pub fn generate_tetradic(color: ColorRGB) -> Colors {
    // need to find if the passed color is light or dark
    let base_hsv = color.rgb_to_hsv();
    let base_hue = base_hsv.0;

    let mut palette = Vec::with_capacity(5);

    // Add the original color to the palette
    palette.push(color.clone());

    // Generate three additional colors, spaced 90 degrees apart
    for i in 0..3 {
        let hue = (base_hue + 90.0 * ((i + 1) as f64)) % 360.0;
        let tetradic_color = ColorRGB::from_hsv((hue, base_hsv.1, base_hsv.2));
        palette.push(tetradic_color);
    }

    // Generate a lighter or darker shade as the fifth color
    let additional_shade = if base_hsv.2 > 0.5 {
        ColorRGB::from_hsv((base_hue, base_hsv.1, base_hsv.2 * 0.8)) // Darker shade
    } else {
        ColorRGB::from_hsv((base_hue, base_hsv.1, base_hsv.2 * 1.2)) // Lighter shade
    };

    palette.push(additional_shade);

    Colors { 
        background: palette[4].clone(), 
        color_a: palette[0].clone(), 
        color_b: palette[1].clone(), 
        color_c: palette[2].clone(), 
        highlight: palette[3].clone() }
}



#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use super::*;
  #[test]
  fn test_generate_tetradic() {
    // https://www.rapidtables.com/convert/color/rgb-to-hsv.html
    // hex: #00EEEC
    // rgb: 0, 238, 236
    // hsv: 179° 100% 93.3%

    let color = ColorRGB::from_hex("#00EEEC").unwrap();
		let colors = generate_tetradic(color);
		println!("0: {}, 1: {}, 2: {}, 3: {}, 4: {}", colors.background.color.to_string(), colors.color_a.color.to_string(), colors.color_b.color.to_string(), colors.color_c.color.to_string(), colors.highlight.color.to_string());
    //assert_eq!(hex, "#00EEEC".to_string());
  }  
}