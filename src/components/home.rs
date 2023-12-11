
pub mod structs;
use structs::{StatefulList, Animation, StyledLine, DCube};

pub mod ui;
use ui::{create_shades, pad_to_length, create_shade_line, create_shade_lines, create_paragraph_line, create_input_paragraph_line, create_styled_shade_lines};

use std::{collections::HashMap, time::Duration};

use ratatui::widgets::canvas::Shape;

use color_eyre::eyre::Result;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{prelude::*, widgets::*};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::UnboundedSender;

use super::{Component, Frame};
use crate::{
  action::Action,
  config::{Config, KeyBindings},
  colors::{Colors, ColorRGB},
};

use std::sync::OnceLock;
use regex::Regex;


pub static RGB_REGEX: OnceLock<Regex> = OnceLock::new();





#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum InputMode {
  #[default]
  HEX,
  RGB,
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum InputSelector {
  #[default]
  Background,
  A,
  B,
  C,
  Highlight,
}

#[derive(Default, Copy, Clone, PartialEq, Eq)]
pub enum DisplayMode {
  #[default]
  Normal,
  InputPrompt,
  Shades,
}

#[derive(Default, Clone)]
pub struct DRect {
  bottom_left: (f64, f64),
  bottom_right: (f64, f64),
  top_left: (f64, f64),
  top_right: (f64, f64),
  origin: (f64, f64),
}

impl DRect {
  pub fn new(x: f64, y: f64, w:f64, h:f64) -> Self
  {
    DRect{
      bottom_left: (x, y),
      bottom_right: (x+w, y),
      top_left: (x, y+h),
      top_right: (x+w, y+h),
      origin: (x+w/2., y+h/2.),
    }
  }

  pub fn rot(&self, ang: f64) -> Self {
    let (cos_ang, sin_ang) = ang.cos_sin();

    let rotate_point = |(px, py): (f64, f64)| {
        let px_rel = px - self.origin.0;
        let py_rel = py - self.origin.1;

        let new_px_rel = cos_ang * px_rel - sin_ang * py_rel;
        let new_py_rel = sin_ang * px_rel + cos_ang * py_rel;

        (self.origin.0 + new_px_rel, self.origin.1 + new_py_rel)
    };

    DRect {
        bottom_left: rotate_point(self.bottom_left),
        bottom_right: rotate_point(self.bottom_right),
        top_left: rotate_point(self.top_left),
        top_right: rotate_point(self.top_right),
        origin: self.origin,
    }    
  }

  pub fn draw_lines(&self, ctx: &mut canvas::Context, colors: &Colors) {
    // Draw Rect
    // Bottom Left to Top Left
    ctx.draw(&canvas::Line {
        x1: self.bottom_left.0,
        y1: self.bottom_left.1,
        x2: self.top_left.0,
        y2: self.top_left.1,
        color: colors.color_a.color,
    });
    // Top Left to Top Right
    ctx.draw(&canvas::Line {
        x1: self.top_left.0,
        y1: self.top_left.1,
        x2: self.top_right.0,
        y2: self.top_right.1,
        color: colors.color_b.color,
    });
    // Top Right to Bottom Right
    ctx.draw(&canvas::Line {
        x1: self.top_right.0,
        y1: self.top_right.1,
        x2: self.bottom_right.0,
        y2: self.bottom_right.1,
        color: colors.color_c.color,
    });
    // Bottom Right to Bottom Left
    ctx.draw(&canvas::Line {
        x1: self.bottom_right.0,
        y1: self.bottom_right.1,
        x2: self.bottom_left.0,
        y2: self.bottom_left.1,
        color: colors.highlight.color,
    });
    // Bottom Left to Top Right
    ctx.draw(&canvas::Line {
        x1: self.bottom_left.0,
        y1: self.bottom_left.1,
        x2: self.top_right.0,
        y2: self.top_right.1,
        color: colors.highlight.flip_rgb(),
    });
  }


  pub fn fill_rect_with_points(
    &self,
    painter: &mut canvas::Painter,
    rect: &Rect,
    color: Color,
    ) {
    for y in rect.y..rect.y + rect.height {
        for x in rect.x..rect.x + rect.width {
            // Map the x, y coordinates within the Rect to the DRect
            let mapped_point = self.map_rect_to_drect(x, y, rect.width, rect.height);

            // Paint the point with the specified color and marker type
            painter.paint(mapped_point.0 as usize, mapped_point.1 as usize, color);
        }
    }
  }
  pub fn fill_rect_with_points_dumb(
    &self,
    painter: &mut canvas::Painter,
    rect: &Rect,
    color: Color,
    increment: f64,
  ) {
    let mut x = self.bottom_left.0;
    let mut y = self.bottom_left.1;

    while x <= self.bottom_right.0 && x <= rect.width as f64 {
        while y <= self.top_left.1 && y <= rect.height as f64 {
            painter.paint(x as usize, y as usize, color);
            y += increment;
        }
        x += increment;
        y = self.bottom_left.1;
    }
  }


/// Does not work correctly, but looks fun
fn map_rect_to_drect(&self, x: u16, y: u16, width: u16, height: u16) -> (f64, f64) {
    // Map x, y to coordinates within the DRect
    let mapped_x = self.bottom_left.0
        + (x as f64 / width as f64) * (self.bottom_right.0 - self.bottom_left.0);
    let mapped_y = self.bottom_left.1
        + (y as f64 / height as f64) * (self.top_left.1 - self.bottom_left.1);

    (mapped_x, mapped_y)
}

}
trait TrigExt {
  fn cos_sin(&self) -> (f64, f64);
}

impl TrigExt for f64 {
  fn cos_sin(&self) -> (f64, f64) {
      (self.cos(), self.sin())
  }
}



#[derive(Default)]
pub struct Home {
  command_tx: Option<UnboundedSender<Action>>,
  config: Config,
  colors: Colors,
  
  display_mode: DisplayMode,
  input_mode: InputMode,
  input_selector: InputSelector,
  selected_color: ColorRGB,
  color_history: Vec<Colors>,
  redo_history: Vec<Colors>,

  inputstr: String,
  inputerr: String,
  anim_querycursor: Animation<String>,

  marker_type: Marker,
  anim_rect: Animation<DRect>,
  _anim_rect: DRect,
  _anim_cube: DCube,

  shade_list: StatefulList<(StyledLine, String)>, // string is shade

}

impl Home {
  pub fn new() -> Self {
    let mut this = Self::default();
    let colors: Colors = Colors { 
      background: ColorRGB::new(32,32,32),
      color_a: ColorRGB::new(255,255,255),
      color_b: ColorRGB::new(144,72,93),
      color_c: ColorRGB::new(26,97,127),
      highlight: ColorRGB::new(72,220,3) };
    this.colors = colors;
    this.anim_querycursor = Animation::with_items(vec!["".to_string()," ".to_string()]);
    this.anim_rect = Animation::with_items(vec![
      DRect{bottom_left:(30.0, 30.0),bottom_right: (70.0, 30.0),top_left: (30.0, 70.0), top_right: (70.0, 70.0), origin: (30.0+20., 30.0+20.)},
      ]);
    this.marker_type = Marker::Braille;
    this._anim_rect = DRect::new(80.0, 30.0, 40.0, 40.0);
    this._anim_cube = DCube::new(70.0, 30.0, 40.0, 40.0);
    this.shade_list = this.create_shade_list();
    this
  }


  pub fn next_color(&mut self) {
    match self.input_selector {
      InputSelector::Background => {self.input_selector = InputSelector::A},
      InputSelector::A => {self.input_selector = InputSelector::B},
      InputSelector::B => {self.input_selector = InputSelector::C},
      InputSelector::C => {self.input_selector = InputSelector::Highlight},
      InputSelector::Highlight => {self.input_selector = InputSelector::Background},
    }
    self.shade_list = self.create_shade_list();  
  }

  pub fn previous_color(&mut self) {
    match self.input_selector {
      InputSelector::Background => {self.input_selector = InputSelector::Highlight},
      InputSelector::A => {self.input_selector = InputSelector::Background},
      InputSelector::B => {self.input_selector = InputSelector::A},
      InputSelector::C => {self.input_selector = InputSelector::B},
      InputSelector::Highlight => {self.input_selector = InputSelector::C},
    }
    self.shade_list = self.create_shade_list(); 
  }

/*   pub fn select_color_by_mode(&mut self) {
    match self.input_selector {
      InputSelector::Background => {self.selected_color = self.colors.background.clone();},
      InputSelector::A => {self.selected_color = self.colors.color_a.clone();},
      InputSelector::B => {self.selected_color = self.colors.color_b.clone();},
      InputSelector::C => {self.selected_color = self.colors.color_c.clone();},
      InputSelector::Highlight => {self.selected_color = self.colors.highlight.clone();},
    }
  } */

  pub fn get_color_by_mode(&self) -> ColorRGB {
    match self.input_selector {
      InputSelector::Background => {self.colors.background.clone()},
      InputSelector::A => {self.colors.color_a.clone()},
      InputSelector::B => {self.colors.color_b.clone()},
      InputSelector::C => {self.colors.color_c.clone()},
      InputSelector::Highlight => {self.colors.highlight.clone()},
    }  
  }

  pub fn make_colors_by_mode(&mut self, color:ColorRGB) -> Colors {
    let mut colors = self.colors.clone();
    match self.input_selector {
      InputSelector::Background => {colors.background= color;},
      InputSelector::A => {colors.color_a = color;},
      InputSelector::B => {colors.color_b = color;},
      InputSelector::C => {colors.color_c = color;},
      InputSelector::Highlight => {colors.highlight = color;},
    }
    colors
  }

  pub fn invert_color(&mut self) {
    let color = self.get_color_by_mode();
    let color = ColorRGB::from_color(color.flip_rgb()).unwrap();
    let colors = self.make_colors_by_mode(color);
    self.change_color(colors);
  }

  pub fn invert_all(&mut self) {
    let mut colors = self.colors.clone();
    colors.background = ColorRGB::from_color(colors.background.flip_rgb()).unwrap();
    colors.color_a = ColorRGB::from_color(colors.color_a.flip_rgb()).unwrap();
    colors.color_b = ColorRGB::from_color(colors.color_b.flip_rgb()).unwrap();
    colors.color_c = ColorRGB::from_color(colors.color_c.flip_rgb()).unwrap();
    colors.highlight = ColorRGB::from_color(colors.highlight.flip_rgb()).unwrap();
    self.change_color(colors);
  }

  pub fn create_styled_paragraph(&self) -> Paragraph {
    Paragraph::new(
      vec![
        create_paragraph_line("Background".to_string(), self.colors.background.clone(), self.colors.background.clone()),
        create_paragraph_line("Lorem".to_string(), self.colors.color_a.clone(), self.colors.background.clone()),
        create_paragraph_line("ipsum".to_string(), self.colors.color_b.clone(), self.colors.background.clone()),
        create_paragraph_line("doloret".to_string(), self.colors.color_c.clone(), self.colors.background.clone()),
        create_paragraph_line("volce".to_string(), self.colors.highlight.clone(), self.colors.background.clone()),
      ]

    ).block(
      Block::new()
      .border_style(
        Style::new().fg(self.colors.background.flip_rgb()))
      .borders(Borders::ALL)
      .bg(self.colors.background.color))

  }

  pub fn create_input_box(&self, selection: InputSelector, color: ColorRGB, text:String) -> Paragraph {
    let selected = if self.input_selector == selection {true} else {false};
    let fddhus = text.clone();

    Paragraph::new(
      vec![
        create_input_paragraph_line(self.input_mode.clone(), fddhus , color.clone(), self.colors.background.clone()),
      ]
    ).block(
      Block::new()
      .border_style(
        Style::new().fg(if selected{self.colors.highlight.color.clone()} else {self.colors.background.flip_rgb().clone()} ))
      .borders(Borders::ALL)
      .bg(self.colors.background.color.clone()))

  }

  pub fn create_shade_list(&mut self) -> StatefulList<(StyledLine, String)> {
    let color = self.get_color_by_mode();
    StatefulList::with_items(create_styled_shade_lines(color, self.colors.background.clone()))
  }

  pub fn popup_shades(&mut self) -> impl Widget + '_ {
    let color = self.get_color_by_mode();
    let lines = create_shade_lines(color.clone(), self.colors.background.clone());
    let titlestr = format!(" Shades for {} ", color.color.to_string());

    let shadebox = Paragraph::new(lines)
    .set_style(Style::new().fg(self.colors.background.flip_rgb()))
    .block(Block::default()
    .bg(self.colors.background.color)
    .borders(Borders::ALL)
    .border_style(Style::new().fg(self.colors.background.flip_rgb()))

    .title(titlestr));
    shadebox
  }

  pub fn popup_input_prompt(&mut self) -> impl Widget + '_ {
    
    let sel_col = self.get_color_by_mode();
    let isbkg = if self.input_selector == InputSelector::Background {true} else {false};

    let mut titlestr = "[ Insert RGB (r,g,b) ]";
    if self.input_mode == InputMode::HEX { titlestr = "[ Insert Hex # ]";};

    let querycursor = self.anim_querycursor.state.selected().unwrap();
    let querycursor = self.anim_querycursor.keyframes[querycursor].clone();
    let selected_ip: String;

    let mut querytext: Vec<Line> = vec![];
    let queryline =   Line::from(vec![
      Span::styled(format!("  {}", self.inputstr), Style::default().fg(self.colors.background.flip_rgb())) , 
      Span::styled(querycursor, Style::default().bg(self.colors.background.flip_rgb()))
      ]);
    querytext.push(Line::from(""));
    //queryline.patch_style(self.apptheme.selected_ip_bg);
    querytext.push(queryline);
    let mut queryerror =   Line::from(format!(" -> {}", self.inputerr));
    queryerror.patch_style(Style::new().fg(self.colors.background.flip_rgb()));
    querytext.push(queryerror);

    let querybox = Paragraph::new(querytext)
    .set_style(Style::new().fg(self.colors.background.flip_rgb()))
    .block(Block::default()
    .bg(self.colors.background.color)
    .borders(Borders::ALL)
    .border_style(Style::new().fg(
      if isbkg {sel_col.flip_rgb()} else {sel_col.color}
    ))
    .title(titlestr).title_alignment(Alignment::Center)).alignment(Alignment::Left);
    querybox

  }

  pub fn popup_palette(&mut self)  -> impl Widget + '_ {
    // Palette should be pickable either as a random palette or based on selected color
    // https://www.thecolorapi.com/docs
    
    Paragraph::default()
  }


  pub fn add_to_inputstr(&mut self, ch: char) {
    self.inputstr.push(ch);
  }

  fn rm_last_char_from_inputstr(&mut self) {
    self.inputstr.pop();
  }

  fn submit_input(&mut self) {
    // parse input if rgb or hex
    if self.input_mode == InputMode::HEX {
      // parse input hex
      self.parse_hex();
    } else {
      // parse input rgb
      self.parse_rgb();
    }

  }

  fn submit_shade(&mut self) {
    if self.display_mode == DisplayMode::Shades {

      let shd_idx = self.shade_list.state.selected();

      if shd_idx.is_some() && !self.shade_list.items.is_empty() {

        let sel_hex: String = self.shade_list.items[shd_idx.unwrap()].clone().1; // this is the hex string

        if !sel_hex.is_empty() {

          let shade_col = ColorRGB::from_hex(&sel_hex).unwrap();

          let colors = self.make_colors_by_mode(shade_col);

          self.change_color(colors);
        }
      }
    }



    
  }

  fn parse_rgb(&mut self) {

   let reg = RGB_REGEX.get_or_init(|| Regex::new(r"(\d{1,3})").unwrap());
    let parsestr = self.inputstr.clone();
    if !parsestr.contains(',') {
      self.inputerr = "Invalid RGB, no delimiter -> r, g, b ".to_string();
      return         
    }

    let mut results = vec![];
    for (_, [line]) in reg.captures_iter(&parsestr).map(|c| c.extract()) {
        results.push(line);
    }
    if results.len() == 3 {
      let mut vals: Vec<u8> = vec![];
      for res in results {
        let val = res.parse::<u8>();
        if val.is_err() {
          self.inputerr = format!("Invalid Value: {}", res);
          return  
        } else {
          vals.push(val.unwrap());
        }       
      }
      let color = ColorRGB::new(vals[0], vals[1], vals[2]);
      let colors = self.make_colors_by_mode(color);
      self.change_color(colors);
      self.inputerr = "Changed Colors".to_string();
    } else {

      self.inputerr = "Invalid RGB".to_string();
      return      
    }

    // (\d{1,3})
  }

  fn parse_hex(&mut self) {
    let parsestr: String;
    if self.inputstr.contains('#') {
      parsestr = self.inputstr.clone();
    } else {
      parsestr = format!("#{}",self.inputstr);
    }
    let color = ColorRGB::from_hex(&parsestr);
    if color.is_err() {
      self.inputerr = "Invalid HEX".to_string();
      return
    } else {
      let color = color.unwrap();
      let colors = self.make_colors_by_mode(color);
      self.change_color(colors);
      self.inputerr = "Changed Colors".to_string();
    }
  }

  fn change_color(&mut self, colors:Colors) {
    self.inputstr = "".to_string();
    self.color_history.push(self.colors.clone());
    self.colors = colors;
    self.shade_list = self.create_shade_list();
  }

  fn undo_change(&mut self) {
    let last = self.color_history.pop();
    if last.is_some() {
      self.redo_history.push(self.colors.clone());
      self.colors = last.unwrap();
      self.shade_list = self.create_shade_list();
    }
  }

  fn redo_change(&mut self) {
    let next = self.redo_history.pop();
    if next.is_some() {
      self.color_history.push(self.colors.clone());
      self.colors = next.unwrap();
      self.shade_list = self.create_shade_list();
    }
  }


  pub fn create_canvas(&mut self, area: &Rect) -> impl Widget + '_ { 

    let sel_rect = self._anim_rect.rot(15.0);
    self._anim_rect = sel_rect.clone();
    //let sel_rect = sel_rect.clone();

    //self._anim_cube.rotate(15.0, 'z');

    //let _anim_cube = self._anim_cube.clone();

    canvas::Canvas::default()
    .background_color(self.colors.background.color)
    .block(Block::default().borders(Borders::ALL).title("").bg(self.colors.background.color).fg(self.colors.background.flip_rgb()))
    .marker(self.marker_type)
    .paint(move |ctx| {

      // Draw Rect
      sel_rect.draw_lines(ctx, &self.colors);


    })
    .x_bounds([-180.0, 180.0])
    .y_bounds([-90.0, 90.0])  

  }

}

impl Component for Home{
  fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
    self.command_tx = Some(tx);
    Ok(())
  }

  fn register_config_handler(&mut self, config: Config) -> Result<()> {
    self.config = config;
    Ok(())
  }

  fn handle_key_events(&mut self, key: KeyEvent) -> Result<Option<Action>> {
    if self.display_mode == DisplayMode::InputPrompt {
      match self.input_mode {
        InputMode::HEX => {      
          match key.code {
            KeyCode::Backspace => {self.rm_last_char_from_inputstr();}
            KeyCode::Char(keychar) => {
              match keychar {
                '0' => {self.add_to_inputstr('0');},
                '1' => {self.add_to_inputstr('1');},
                '2' => {self.add_to_inputstr('2');},
                '3' => {self.add_to_inputstr('3');},
                '4' => {self.add_to_inputstr('4');},
                '5' => {self.add_to_inputstr('5');},
                '6' => {self.add_to_inputstr('6');},
                '7' => {self.add_to_inputstr('7');},
                '8' => {self.add_to_inputstr('8');},
                '9' => {self.add_to_inputstr('9');},
                'A'|'a' => {self.add_to_inputstr('a');},
                'B'|'b' => {self.add_to_inputstr('b');},
                'C'|'c' => {self.add_to_inputstr('c');},
                'D'|'d' => {self.add_to_inputstr('d');},
                'E'|'e' => {self.add_to_inputstr('e');},
                'F'|'f' => {self.add_to_inputstr('f');},
                '#' => {self.add_to_inputstr('#');},
                _ => {}
              }
            return Ok(Some(Action::Render))
            },
            _ => {}       
          }
        },
        InputMode::RGB => {
          match key.code {
            KeyCode::Backspace => {self.rm_last_char_from_inputstr();}
            KeyCode::Char(keychar) => {
              match keychar {
                '0' => {self.add_to_inputstr('0');},
                '1' => {self.add_to_inputstr('1');},
                '2' => {self.add_to_inputstr('2');},
                '3' => {self.add_to_inputstr('3');},
                '4' => {self.add_to_inputstr('4');},
                '5' => {self.add_to_inputstr('5');},
                '6' => {self.add_to_inputstr('6');},
                '7' => {self.add_to_inputstr('7');},
                '8' => {self.add_to_inputstr('8');},
                '9' => {self.add_to_inputstr('9');},
                ',' => {self.add_to_inputstr(',');},
                '(' => {self.add_to_inputstr('(');},
                ')' => {self.add_to_inputstr(')');},
                _ => {}
              }
              return Ok(Some(Action::Render))
            },
            _ => {}
          }
        }
      };
    }
    Ok(None)
  }

  fn update(&mut self, action: Action) -> Result<Option<Action>> {
    match action {
      Action::Tick => {
        
      },
      Action::Render => {
        self.anim_querycursor.next();
        self.anim_rect.next();}
      Action::NextColor => {self.next_color(); }, // self.select_color_by_mode();
      Action::PreviousColor => {self.previous_color();}, // self.select_color_by_mode();
      Action::InputHEX => {self.input_mode = InputMode::HEX;},
      Action::InputRGB => {self.input_mode = InputMode::RGB;},
      Action::InputPrompt => {if self.display_mode != DisplayMode::InputPrompt {self.display_mode = DisplayMode::InputPrompt} else {self.display_mode = DisplayMode::Normal};}
      Action::SubmitInput => {self.submit_input(); self.submit_shade();},
      Action::ChangeUndo => {self.undo_change();},
      Action::ChangeRedo => {self.redo_change();},
      Action::ShowShades => {if self.display_mode != DisplayMode::Shades {self.display_mode = DisplayMode::Shades} else {self.display_mode = DisplayMode::Normal};},
      Action::NextShade => {self.shade_list.next();}
      Action::PreviousShade => {self.shade_list.previous();}
      Action::InvertColor => {self.invert_color();}
      Action::InvertAll => {self.invert_all();}
      _ => {},
    }
    Ok(None)
  }

  fn draw(&mut self, f: &mut Frame<'_>, area: Rect) -> Result<()> {
    let layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints([Constraint::Percentage(15), Constraint::Percentage(50), Constraint::Percentage(35)])
      .split(f.size());

    let input_layout = Layout::default()
      .direction(Direction::Vertical)
      .constraints([Constraint::Percentage(20),Constraint::Percentage(20),Constraint::Percentage(20),Constraint::Percentage(20),Constraint::Percentage(20),])
      .split(layout[2]);

    f.render_widget(self.create_input_box(InputSelector::Background, self.colors.background.clone(), "Background".to_string()), input_layout[0]);
    f.render_widget(self.create_input_box(InputSelector::A, self.colors.color_a.clone(), "A".to_string()), input_layout[1]);
    f.render_widget(self.create_input_box(InputSelector::B, self.colors.color_b.clone(), "B".to_string()), input_layout[2]);
    f.render_widget(self.create_input_box(InputSelector::C, self.colors.color_c.clone(), "C".to_string()), input_layout[3]);
    f.render_widget(self.create_input_box(InputSelector::Highlight, self.colors.highlight.clone(), "D".to_string()), input_layout[4]);

    f.render_widget(self.create_styled_paragraph().alignment(Alignment::Center), layout[0]);

    f.render_widget(self.create_canvas(&layout[1]), layout[1]);

    let canvaslayout = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([Constraint::Percentage(10), Constraint::Percentage(20), Constraint::Percentage(2),  Constraint::Percentage(36), Constraint::Percentage(32)])
    .split(layout[1]);

    let popuplayout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Percentage(10), Constraint::Percentage(80), Constraint::Percentage(10)])
    .split(canvaslayout[3]);

    let div = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Percentage(10), Constraint::Percentage(80), Constraint::Percentage(10)])
    .split(canvaslayout[1]);

    let blocklayout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Percentage(2), Constraint::Percentage(20), Constraint::Percentage(5), Constraint::Percentage(20), Constraint::Percentage(5), Constraint::Percentage(20), Constraint::Percentage(5), Constraint::Percentage(20), Constraint::Percentage(2)])
    .split(div[1]);
    f.render_widget(Paragraph::new("").bg(self.colors.background.color), div[1]);
    f.render_widget(Paragraph::new("").bg(self.colors.color_a.color), blocklayout[1]);
    f.render_widget(Paragraph::new("").bg(self.colors.color_b.color), blocklayout[3]);
    f.render_widget(Paragraph::new("").bg(self.colors.color_c.color), blocklayout[5]);
    f.render_widget(Paragraph::new("").bg(self.colors.highlight.color), blocklayout[7]);




    match self.display_mode {
      DisplayMode::Normal => {},
      DisplayMode::InputPrompt =>{
        let centered = centered_rect(popuplayout[1], 50, 30);
        f.render_widget(Clear, centered);
        f.render_widget(self.popup_input_prompt(), centered);
      },
      DisplayMode::Shades => {
        let shadelines: Vec<ListItem> = self.shade_list
        .items
        .iter()
        .map(|i| {
          let mut line: Line = Line::default();
          for word in i.0.words.clone() {
            let cspan = Span::styled(word.0, word.1);
            line.spans.push(cspan);
          }
          ListItem::new(line)}).collect();
          let color = self.get_color_by_mode();
          let isbkg = if self.input_selector == InputSelector::Background {true} else {false};
          let titlestr = format!(" Shades for {} ", color.color.to_string());
          let shadelist = List::new( shadelines) //home.styledio.clone()
            .block(Block::default()
              .bg(self.colors.background.color)
              .borders(Borders::ALL)
              .border_style(Style::new().fg(
                if isbkg {
                  color.flip_rgb()
                } else {
                  color.color
                }
              ))
              .title(block::Title::from(titlestr).alignment(Alignment::Left))
            )
            .highlight_style(Style::new().fg(self.colors.highlight.color))
            .highlight_symbol(">> ");


        //f.render_widget(Clear, centered_rect(area, 32, 38));
        //f.render_widget(self.popup_shades(), centered_rect(area, 32, 38));
        //f.render_stateful_widget(shadelist, centered_rect(area, 32, 38), &mut self.shade_list.state);
        f.render_widget(Clear, popuplayout[1]);
        f.render_stateful_widget(shadelist, popuplayout[1], &mut self.shade_list.state);
      }
    };

    Ok(())
  }
}

pub fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
  let popup_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
    Constraint::Percentage((100 - percent_y) / 2),
    Constraint::Percentage(percent_y),
    Constraint::Percentage((100 - percent_y) / 2),
    ])
    .split(r);

  Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
    Constraint::Percentage((100 - percent_x) / 2),
    Constraint::Percentage(percent_x),
    Constraint::Percentage((100 - percent_x) / 2),
    ])
    .split(popup_layout[1])[1]
}
