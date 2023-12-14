
use crate::colors::{ColorRGB, Colors};

/// Example 
/// ```
/// 
/// let color = ColorRGB::from_hex("#976cdc").unwrap()
/// let colors = generate_monochromatic(color)
/// 
/// println!("0: {}, 1: {}, 2: {}, 3: {}, 4: {}", colors.background.color.to_string(), colors.color_a.color.to_string(), colors.color_b.color.to_string(), colors.color_c.color.to_string(), colors.highlight.color.to_string());
/// 
/// ```
pub fn generate_monochromatic(color: ColorRGB) -> Colors {
    // need to find if the passed color is light or dark
    //let lum = get_luminance(&color);
    const NUM_SHADES: usize = 5;
    // bkg color needs 4.5:1 contrast with self - yes!
    let base_hsv = color.rgb_to_hsv();
    let mut palette = Vec::with_capacity(NUM_SHADES);

		let base_v = base_hsv.2;

		let low_lim: f64;
		let up_lim: f64;

		if base_v < 0.5 {low_lim = base_v} else {low_lim = 0.1};
		if base_v > 0.5 {up_lim = base_v} else {up_lim = 0.9};  
		


    const MIN_TINT_VALUE: f64 = 0.4;
    const MAX_SHADE_VALUE: f64 = 0.6;

    for i in 0..NUM_SHADES {
        let value = (i as f64) / ((NUM_SHADES - 1) as f64);
        let adjusted_v = low_lim + value * (up_lim - low_lim);

        let shade = ColorRGB::from_hsv((base_hsv.0, base_hsv.1, adjusted_v));
        palette.push(shade);
    }


    // generate some shades with incremental difference
    // calculate their contrast
		// TODO improve contrasts

		Colors { 
			background: palette[0].clone(), 
			color_a: palette[1].clone(), 
			color_b: palette[2].clone(), 
			color_c: palette[3].clone(), 
			highlight: palette[4].clone() }

}

#[cfg(test)]
mod tests {
  use pretty_assertions::assert_eq;
  use super::*;
  #[test]
  fn test_generate_monochromatic() {
    // https://www.rapidtables.com/convert/color/rgb-to-hsv.html
    // hex: #00EEEC
    // rgb: 0, 238, 236
    // hsv: 179° 100% 93.3%

    let color = ColorRGB::from_hex("#00EEEC").unwrap();
		let colors = generate_monochromatic(color);
		println!("0: {}, 1: {}, 2: {}, 3: {}, 4: {}", colors.background.color.to_string(), colors.color_a.color.to_string(), colors.color_b.color.to_string(), colors.color_c.color.to_string(), colors.highlight.color.to_string());
    //assert_eq!(hex, "#00EEEC".to_string());
  }  
}