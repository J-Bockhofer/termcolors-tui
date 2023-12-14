use crate::colors::{ColorRGB, Colors};

pub fn generate_complementary(color: ColorRGB, num_colors: usize) -> Colors {
    // need to find if the passed color is light or dark
    let base_hsv = color.rgb_to_hsv();
    let base_hue = base_hsv.0;

    let mut palette = Vec::with_capacity(num_colors);

    // Add the original color to the palette
    palette.push(color.clone());

    for i in 0..(num_colors - 1) {
        let hue = (base_hue + 180.0 * ((i + 1) as f64)) % 360.0;
        let complementary_color = ColorRGB::from_hsv((hue, base_hsv.1, base_hsv.2));
        palette.push(complementary_color);
    }

		Colors {
			background: palette[0].clone(),
			color_a: palette[1].clone(),
			color_b: palette[2].clone(),
			color_c: palette[3].clone(),
			highlight: palette[4].clone(),
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