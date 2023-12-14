use crate::colors::{ColorRGB, Colors};

pub fn generate_split_complementary(color: ColorRGB) -> Colors {
    // need to find if the passed color is light or dark
    let base_hsv = color.rgb_to_hsv();
    let base_hue = base_hsv.0;

    let mut palette = Vec::with_capacity(5);

    // Add the original color to the palette
    palette.push(color.clone());

    // Generate two additional colors, spaced 150 degrees apart
    for i in 0..2 {
        let hue = (base_hue + 150.0 * ((i + 1) as f64)) % 360.0;
        let split_complementary_color = ColorRGB::from_hsv((hue, base_hsv.1, base_hsv.2));
        palette.push(split_complementary_color);
    }

    // Generate two shades of the passed color
    let darker_shade = ColorRGB::from_hsv((base_hue, base_hsv.1, base_hsv.2 * 0.8));
    let lighter_shade = ColorRGB::from_hsv((base_hue, base_hsv.1, base_hsv.2 * 1.2));

    palette.push(darker_shade);
    palette.push(lighter_shade);
    
    Colors {
        background: palette[4].clone(),
        color_a: palette[0].clone(),
        color_b: palette[1].clone(),
        color_c: palette[2].clone(),
        highlight: palette[3].clone(),
    }


}


mod tests {
    use pretty_assertions::assert_eq;
    use super::*;
    #[test]
    fn test_generate_splitcomp() {
      // https://www.rapidtables.com/convert/color/rgb-to-hsv.html
      // hex: #00EEEC
      // rgb: 0, 238, 236
      // hsv: 179° 100% 93.3%
  
      let color = ColorRGB::from_hex("#00EEEC").unwrap();
          let colors = generate_split_complementary(color);
          println!("0: {}, 1: {}, 2: {}, 3: {}, 4: {}", colors.background.color.to_string(), colors.color_a.color.to_string(), colors.color_b.color.to_string(), colors.color_c.color.to_string(), colors.highlight.color.to_string());
      //assert_eq!(hex, "#00EEEC".to_string());
    }  
  }