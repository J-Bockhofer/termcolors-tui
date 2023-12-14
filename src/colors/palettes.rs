
use super::*;

use generators::{
	monochromatic::generate_monochromatic, 
	analogous::generate_analogous, 
	complementary::generate_complementary, 
	triadic::generate_triadic, 
	tetradic::generate_tetradic,
	split_complementary::generate_split_complementary,
};


pub enum Harmony {
    Monochromatic, // just shade/tint
    Complementary,  // one opposite color plus shades
    SplitComplementary, // two opposite colors plus shades
    Triadic,    // even triangle, split 360/3 -> 120 input color + 120 + 120
    Tetradic,   // get one analogous color and its complement + selfs complement
    Analogous, // next to each other... what is next.. how many degrees? 15° ?
}

pub fn generate_palette_with_harmony(color: ColorRGB, harmony: Harmony) -> Colors {
    match harmony {
        Harmony::Monochromatic => {generate_monochromatic(color)},
        Harmony::Complementary => {generate_complementary(color, 5)},
        Harmony::SplitComplementary => {generate_split_complementary(color)},
        Harmony::Triadic => {generate_triadic(color)},
        Harmony::Tetradic => {generate_tetradic(color)},
        Harmony::Analogous => {generate_analogous(color)},
    }
}











pub fn contrast_with_inverted(color: &ColorRGB) -> f32 {
    let inv = color.with_flip_rgb();
    get_contrast(color, &inv)
}

pub struct Palette {


}