use std::rc::Rc;
use ratatui::{prelude::*, widgets::*};
use super::{ColorRGB, InputMode, StyledLine, get_contrast};

pub fn create_hue_column(color: &ColorRGB) -> Paragraph {

    let hsv = color.rgb_to_hsv();

    let p: Vec<Line> = vec![
			Line::from(Span::styled("    ", Style::new())),
			Line::from(Span::styled("    ", Style::new())),
			Line::from(Span::styled("    ", Style::new().bg(color.with_hue(359.0).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_hue(330.0).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_hue(300.0).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_hue(270.0).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_hue(240.0).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_hue(210.0).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_hue(180.0).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_hue(150.0).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_hue(120.0).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_hue(90.0).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_hue(60.0).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_hue(30.0).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_hue(0.0).color))),
    ];
    Paragraph::new(p)
}

pub fn create_sat_column(color: &ColorRGB) -> Paragraph {

	let hsv = color.rgb_to_hsv();

	let p: Vec<Line> = vec![
			Line::from(Span::styled("    ", Style::new())),
			Line::from(Span::styled("    ", Style::new())),
			Line::from(Span::styled("    ", Style::new().bg(color.with_saturation(1.0).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_saturation(0.9).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_saturation(0.8).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_saturation(0.7).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_saturation(0.6).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_saturation(0.5).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_saturation(0.4).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_saturation(0.3).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_saturation(0.2).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_saturation(0.1).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_saturation(0.0).color))),
	];
	Paragraph::new(p)
}

pub fn create_val_column(color: &ColorRGB) -> Paragraph {

	let hsv = color.rgb_to_hsv();
	let p: Vec<Line> = vec![
			Line::from(Span::styled("    ", Style::new())),
			Line::from(Span::styled("    ", Style::new())),
			Line::from(Span::styled("    ", Style::new().bg(color.with_value(1.0).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_value(0.9).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_value(0.8).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_value(0.7).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_value(0.6).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_value(0.5).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_value(0.4).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_value(0.3).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_value(0.2).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_value(0.1).color))),
			Line::from(Span::styled("    ", Style::new().bg(color.with_value(0.0).color))),
	];
	Paragraph::new(p)
}

pub fn create_bar(bkgcolor: &ColorRGB, value: u64, max_val: u64, title: String, border_color: Color) -> impl Widget {
    let bar  = Bar::default()
			.value(value)//value	
			.style(Style::default().fg(bkgcolor.flip_rgb()))
			.value_style(Style::default().bg(bkgcolor.flip_rgb()).fg(bkgcolor.color))
			.text_value(format!("{}",value));

    BarChart::default()
			.block(Block::default().title(title).borders(Borders::ALL).border_style(Style::new().fg(border_color)))
			.bar_width(3)
			.bar_gap(1)
			.group_gap(3)
			.bar_style(Style::new().fg(bkgcolor.color).bg(bkgcolor.color))
			.value_style(Style::new().fg(bkgcolor.flip_rgb()).bold())
			.label_style(Style::new().fg(bkgcolor.flip_rgb()))
			.data(BarGroup::default().bars(&[bar]))
			.max(max_val)
}


pub fn make_slider_layout(area: Rect) -> Rc<[Rect]> {
	let v_layout = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([Constraint::Percentage(10), Constraint::Percentage(80), Constraint::Percentage(10)])
		.split(area);
	let v_layout = Layout::default()
		.direction(Direction::Vertical)
		.constraints([Constraint::Percentage(10), Constraint::Percentage(80), Constraint::Percentage(10)])
		.split(v_layout[1]);
	let v_layout = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([Constraint::Percentage(19), Constraint::Percentage(30), Constraint::Percentage(2), Constraint::Percentage(30), Constraint::Percentage(19)])
		.split(v_layout[1]);
	v_layout
}
