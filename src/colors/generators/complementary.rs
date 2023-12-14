use crate::colors::{ColorRGB, Colors};

pub fn generate_complementary(color: ColorRGB, num_colors: usize) -> Colors {
    // need to find if the passed color is light or dark
    // need to find if the passed color is light or dark
    let base_hsv = color.rgb_to_hsv();
    let base_hue = base_hsv.0;

    // Generate lighter and darker shades for the base color
    let lighter_shade = ColorRGB::from_hsv((base_hue, base_hsv.1, base_hsv.2 * 1.2));
    let darker_shade = ColorRGB::from_hsv((base_hue, base_hsv.1, base_hsv.2 * 0.8));

    // Generate the complementary color
    let complementary_color = ColorRGB::from_hsv(((base_hue + 180.0) % 360.0, base_hsv.1, base_hsv.2));

    // Generate an additional shade for the complementary color
    let additional_shade = if base_hsv.2 < 0.5 {
        ColorRGB::from_hsv(((base_hue + 180.0) % 360.0, base_hsv.1, base_hsv.2 * 0.6)) // Darker shade
    } else {
        ColorRGB::from_hsv(((base_hue + 180.0) % 360.0, base_hsv.1, base_hsv.2 * 1.4)) // Lighter shade
    };

    Colors {
        background: darker_shade.clone(), // 
        color_a: color.clone(),
        color_b: lighter_shade.clone(),
        color_c: additional_shade.clone(),
        highlight: complementary_color.clone(),
    }
    
}

#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use super::*;
  #[test]
  fn test_generate_complementary() {
    // https://www.rapidtables.com/convert/color/rgb-to-hsv.html
    // hex: #00EEEC
    // rgb: 0, 238, 236
    // hsv: 179° 100% 93.3%

    let color = ColorRGB::from_hex("#00EEEC").unwrap();
		let colors = generate_complementary(color, 5);
		println!("0: {}, 1: {}, 2: {}, 3: {}, 4: {}", colors.background.color.to_string(), colors.color_a.color.to_string(), colors.color_b.color.to_string(), colors.color_c.color.to_string(), colors.highlight.color.to_string());
    //assert_eq!(hex, "#00EEEC".to_string());
  }  
}