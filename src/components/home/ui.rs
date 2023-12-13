pub mod hsv;


use ratatui::{prelude::*, widgets::*};
use super::{ColorRGB, InputMode, StyledLine, get_contrast};

pub fn pad_to_length(input: String, length: usize) -> String {
    format!("{:<width$}", input, width = length)
}
pub fn leftpad_to_length(input: String, length: usize) -> String {
  format!("{:>width$}", input, width = length)
}

pub fn create_shades(color:ColorRGB) -> Vec<Span<'static>> {
  vec![
      Span::styled(format!(" "), Style::new().bg(color.shade(-0.9))),
      Span::styled(format!(" "), Style::new().bg(color.shade(-0.8))),
      Span::styled(format!(" "), Style::new().bg(color.shade(-0.7))),
      Span::styled(format!(" "), Style::new().bg(color.shade(-0.6))),
      Span::styled(format!(" "), Style::new().bg(color.shade(-0.5))),
      Span::styled(format!(" "), Style::new().bg(color.shade(-0.4))),
      Span::styled(format!(" "), Style::new().bg(color.shade(-0.3))),
      Span::styled(format!(" "), Style::new().bg(color.shade(-0.2))),
      Span::styled(format!(" "), Style::new().bg(color.shade(-0.1))),
      Span::styled(format!(" "), Style::new().bg(color.color)),
      Span::styled(format!(" "), Style::new().bg(color.shade(0.1))),
      Span::styled(format!(" "), Style::new().bg(color.shade(0.2))),
      Span::styled(format!(" "), Style::new().bg(color.shade(0.3))),
      Span::styled(format!(" "), Style::new().bg(color.shade(0.4))),
      Span::styled(format!(" "), Style::new().bg(color.shade(0.5))),
      Span::styled(format!(" "), Style::new().bg(color.shade(0.6))),
      Span::styled(format!(" "), Style::new().bg(color.shade(0.7))),
      Span::styled(format!(" "), Style::new().bg(color.shade(0.8))),
      Span::styled(format!(" "), Style::new().bg(color.shade(0.9))),
  ]
  }
  
pub fn create_shade_line(color:ColorRGB, frac: f32, bkgcolor:ColorRGB,) -> Line<'static> {
    let shade = color.shade(frac);
    let shade_hex = shade.to_string();
    let shade_color = ColorRGB::from_hex(&shade_hex);
    let _rgb: String;
    if shade_color.is_ok() {
      let shade_color = shade_color.unwrap();
      _rgb = pad_to_length(format!("({},{},{})", shade_color.r, shade_color.g, shade_color.b), 13);
    }
    else {
      _rgb = "".to_string();
    }
    Line::from(
      vec![
        Span::styled(format!("    {}    ", shade_hex), Style::new().fg(bkgcolor.flip_rgb())),
        Span::styled(format!("    "), Style::new()),
        Span::styled(format!("          "), Style::new().bg(shade)),
        Span::styled(format!("    "), Style::new()),
        Span::styled(format!("    {}    ", _rgb), Style::new().fg(bkgcolor.flip_rgb())),
        Span::styled(format!("    "), Style::new()),
        Span::styled(format!(" Lorem ipsum "), Style::new().fg(shade)),
      ]
    )
  }
  
pub fn create_shade_lines(color:ColorRGB, bkgcolor:ColorRGB) -> Vec<Line<'static>> {
    vec![
      Line::from(format!("")),
      create_shade_line(color.clone(), -0.9, bkgcolor.clone()),
      create_shade_line(color.clone(), -0.8, bkgcolor.clone()),
      create_shade_line(color.clone(), -0.7, bkgcolor.clone()),
      create_shade_line(color.clone(), -0.6, bkgcolor.clone()),
      create_shade_line(color.clone(), -0.5, bkgcolor.clone()),
      create_shade_line(color.clone(), -0.4, bkgcolor.clone()),
      create_shade_line(color.clone(), -0.3, bkgcolor.clone()),
      create_shade_line(color.clone(), -0.2, bkgcolor.clone()),
      create_shade_line(color.clone(), -0.1, bkgcolor.clone()),
      create_shade_line(color.clone(), 0.0, bkgcolor.clone()),
      create_shade_line(color.clone(), 0.1, bkgcolor.clone()),
      create_shade_line(color.clone(), 0.2, bkgcolor.clone()),
      create_shade_line(color.clone(), 0.3, bkgcolor.clone()),
      create_shade_line(color.clone(), 0.4, bkgcolor.clone()),
      create_shade_line(color.clone(), 0.5, bkgcolor.clone()),
      create_shade_line(color.clone(), 0.6, bkgcolor.clone()),
      create_shade_line(color.clone(), 0.7, bkgcolor.clone()),
      create_shade_line(color.clone(), 0.8, bkgcolor.clone()),
      create_shade_line(color.clone(), 0.9, bkgcolor.clone()),
      Line::from(format!("")),
    ]
  }

pub fn create_styled_shade_line(color:ColorRGB, frac: f32, bkgcolor:ColorRGB) -> (StyledLine, String) {
    let shade = color.shade(frac);
    let shade_hex = shade.to_string();
    let shade_color = ColorRGB::from_hex(&shade_hex);
    let _rgb: String;
    let contrast: f32;
    if shade_color.is_ok() {
      let shade_color = shade_color.unwrap();
      contrast = get_contrast(&shade_color, &bkgcolor);
      _rgb = pad_to_length(format!("({},{},{})", shade_color.r, shade_color.g, shade_color.b), 13);
    }
    else {
      contrast = 0.0;
      _rgb = "".to_string();
    }
    let mut res = StyledLine::default();
    res.words = vec![
        (format!("    {}    ", shade_hex), Style::new().fg(bkgcolor.flip_rgb())),
        (format!("    "), Style::new()),
        (format!("          "), Style::new().bg(shade)),
        (format!("    "), Style::new()),
        (format!("    {}    ", _rgb), Style::new().fg(bkgcolor.flip_rgb())),
        (format!("    "), Style::new()),
        (format!(" Lorem ipsum "), Style::new().fg(shade)),
        (format!("  {:.2}  ", contrast), Style::new().fg(bkgcolor.flip_rgb())),
    ];
    (res, shade.to_string())
  }

pub fn create_styled_shade_lines(color:ColorRGB, bkgcolor:ColorRGB) -> Vec<(StyledLine, String)> {
    vec![
        (StyledLine{words: vec![(String::from(""), Style::default())]}, "".to_string()),
      create_styled_shade_line(color.clone(), -0.9, bkgcolor.clone()),
      create_styled_shade_line(color.clone(), -0.8, bkgcolor.clone()),
      create_styled_shade_line(color.clone(), -0.7, bkgcolor.clone()),
      create_styled_shade_line(color.clone(), -0.6, bkgcolor.clone()),
      create_styled_shade_line(color.clone(), -0.5, bkgcolor.clone()),
      create_styled_shade_line(color.clone(), -0.4, bkgcolor.clone()),
      create_styled_shade_line(color.clone(), -0.3, bkgcolor.clone()),
      create_styled_shade_line(color.clone(), -0.2, bkgcolor.clone()),
      create_styled_shade_line(color.clone(), -0.1, bkgcolor.clone()),
      create_styled_shade_line(color.clone(), 0.0, bkgcolor.clone()),
      create_styled_shade_line(color.clone(), 0.1, bkgcolor.clone()),
      create_styled_shade_line(color.clone(), 0.2, bkgcolor.clone()),
      create_styled_shade_line(color.clone(), 0.3, bkgcolor.clone()),
      create_styled_shade_line(color.clone(), 0.4, bkgcolor.clone()),
      create_styled_shade_line(color.clone(), 0.5, bkgcolor.clone()),
      create_styled_shade_line(color.clone(), 0.6, bkgcolor.clone()),
      create_styled_shade_line(color.clone(), 0.7, bkgcolor.clone()),
      create_styled_shade_line(color.clone(), 0.8, bkgcolor.clone()),
      create_styled_shade_line(color.clone(), 0.9, bkgcolor.clone()),
      (StyledLine{words: vec![(String::from(""), Style::default())]}, "".to_string()),
    ]
  }

pub fn create_paragraph_line(text: String, color: ColorRGB, bkgcolor: ColorRGB) -> Line<'static> {
    const PADTO: usize = 10;
    let _text = format!("{}", pad_to_length(text, PADTO));
    let _rgb = pad_to_length(format!("({},{},{})", color.r, color.g, color.b), 13);
    let flip = ColorRGB::from_color(color.flip_rgb()).unwrap();    
    let mut line = Line::from( vec![
      Span::styled(format!(" {} ", _text), Style::new().fg(color.color)),
      Span::styled(format!(" {} ", _text), Style::new().bg(color.color).fg(color.flip_rgb())),
      Span::styled(        "       ", Style::new().fg(bkgcolor.flip_rgb())),
      Span::styled(format!(" {} ", color.color.to_string()), Style::new().fg(bkgcolor.flip_rgb())),
      Span::styled(format!(" {} ", _rgb), Style::new().fg(bkgcolor.flip_rgb())),      
    ]);
    line.spans.push(Span::styled(format!(" Ctr: {:.2} ", get_contrast(&color, &bkgcolor)), Style::new().fg(bkgcolor.flip_rgb())));
    let _hsv = color.rgb_to_hsv();
    let h = leftpad_to_length(format!("{:.0}", _hsv.0), 4);
    let s = pad_to_length(format!("{:.2}", _hsv.1), 4);
    let v = pad_to_length(format!("{:.2}", _hsv.2), 4 );
    let _hsv = pad_to_length(format!(" HSV: {} {} {} ", h, s, v),20);
    line.spans.push(Span::styled(_hsv, Style::new().fg(bkgcolor.flip_rgb())));
    line.spans.push(Span::styled(        " Shd:  ", Style::new().fg(bkgcolor.flip_rgb())));
    let shades = create_shades(color.clone());
    for shade in shades {line.spans.push(shade);}

    line.spans.push(Span::styled(        " Inv:  ", Style::new().fg(bkgcolor.flip_rgb())));
    line.spans.push(Span::styled(format!(" {} ", _text), Style::new().fg(flip.color)));
    line.spans.push(Span::styled(format!(" {} ", _text), Style::new().bg(flip.color).fg(color.color)));
    line.spans.push(Span::styled(        "       ", Style::new()));
    line.spans.push(Span::styled(format!(" {} ", flip.color.to_string()), Style::new().fg(bkgcolor.flip_rgb())));
    line.spans.push(Span::styled(format!(" Ctr: {:.2} ", get_contrast(&flip, &bkgcolor)), Style::new().fg(bkgcolor.flip_rgb())));
    let _hsv = flip.rgb_to_hsv();
    let h = leftpad_to_length(format!("{:.0}", _hsv.0), 4);
    let s = pad_to_length(format!("{:.2}", _hsv.1), 4);
    let v = pad_to_length(format!("{:.2}", _hsv.2), 4);
    let _hsv = pad_to_length(format!(" HSV: {} {} {} ", h, s, v),20);
    line.spans.push(Span::styled(_hsv, Style::new().fg(bkgcolor.flip_rgb())));
    line.spans.push(Span::styled(        " Shd:  ", Style::new().fg(bkgcolor.flip_rgb())));
    let shades = create_shades(flip);
    for shade in shades {line.spans.push(shade);}
    line

  }

pub fn create_input_paragraph_line(input_mode: InputMode, text: String, color: ColorRGB, bkgcolor: ColorRGB) -> Line<'static> {
    const PADTO: usize = 10;

    let rgb_style = if input_mode == InputMode::RGB {Style::new().fg(bkgcolor.color).bg(bkgcolor.flip_rgb())} else {Style::new().fg(bkgcolor.flip_rgb())};
    let hex_style = if input_mode == InputMode::HEX {Style::new().fg(bkgcolor.color).bg(bkgcolor.flip_rgb())} else {Style::new().fg(bkgcolor.flip_rgb())};

    let _text = format!("{}", pad_to_length(text, PADTO));
    let _rgb = pad_to_length(format!("({},{},{})", color.r, color.g, color.b), 13);
    let flip = ColorRGB::from_color(color.flip_rgb()).unwrap();    
    let mut line = Line::from( vec![
      Span::styled(format!(" {} ", _text), Style::new().fg(color.color)),
      Span::styled(format!(" {} ", _text), Style::new().bg(color.color).fg(color.flip_rgb())),
      Span::styled(        "       ", Style::new()),
      Span::styled(format!(" {} ", color.color.to_string()), hex_style),
      Span::styled(format!(" {} ", _rgb), rgb_style),      
    ]);
    line.spans.push(Span::styled(        " Shd:  ", Style::new().fg(bkgcolor.flip_rgb())),);
    let shades = create_shades(color.clone());
    for shade in shades {line.spans.push(shade);}
    line.spans.push(Span::styled(        " Inv:  ", Style::new().fg(bkgcolor.flip_rgb())));
    line.spans.push(Span::styled(format!(" {} ", _text), Style::new().fg(flip.color)));
    line.spans.push(Span::styled(format!(" {} ", _text), Style::new().bg(flip.color).fg(color.color.clone())));
    line.spans.push(Span::styled(        "       ", Style::new()));
    line.spans.push(Span::styled(format!(" {} ", flip.color.to_string()), Style::new().fg(bkgcolor.flip_rgb())));
    line.spans.push(Span::styled(        " Shd:  ", Style::new().fg(bkgcolor.flip_rgb())),);

    let shades = create_shades(flip);
    for shade in shades {line.spans.push(shade);}
    line  

  }

