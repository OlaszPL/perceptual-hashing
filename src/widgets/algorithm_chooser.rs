use std::ops::ControlFlow;
use super::letters::*;
use crate::app::App;
use crate::handler::handle::HashingType;

use ratatui::{
    buffer::Buffer,
    crossterm::{event::{self, KeyCode}},
    layout::{Constraint, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line},
    widgets::{Widget, Block, canvas::Canvas},
    symbols::{border, Marker},
    Frame,
};

/// A custom widget that renders a button with a label, button_theme and state.
#[derive(Debug, Clone)]
struct Button<'a> {
    label: Line<'a>,
    button_theme: ButtonTheme,
    state: State,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Normal,
    Selected,
    Active,
}

#[derive(Debug, Clone, Copy)]
struct ButtonTheme {
    text: Color,
    background: Color,
    highlight: Color,
    shadow: Color,
}

const BLUE: ButtonTheme = ButtonTheme {
    text: Color::Rgb(16, 24, 48),
    background: Color::Rgb(48, 72, 144),
    highlight: Color::Rgb(64, 96, 192),
    shadow: Color::Rgb(32, 48, 96),
};

const RED: ButtonTheme = ButtonTheme {
    text: Color::Rgb(48, 16, 16),
    background: Color::Rgb(144, 48, 48),
    highlight: Color::Rgb(192, 64, 64),
    shadow: Color::Rgb(96, 32, 32),
};

/// A button with a label that can be button_themed.
impl<'a> Button<'a> {
    pub fn new<T: Into<Line<'a>>>(label: T) -> Self {
        Button {
            label: label.into(),
            button_theme: BLUE,
            state: State::Normal,
        }
    }

    pub const fn button_theme(mut self, button_theme: ButtonTheme) -> Self {
        self.button_theme = button_theme;
        self
    }

    pub const fn state(mut self, state: State) -> Self {
        self.state = state;
        self
    }
}

impl<'a> Widget for Button<'a> {
    #[allow(clippy::cast_possible_truncation)]
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (background, text, shadow, highlight) = self.colors();
        buf.set_style(area, Style::new().bg(background).fg(text));

        // render top line if there's enough space
        if area.height > 2 {
            buf.set_string(
                area.x,
                area.y,
                "▔".repeat(area.width as usize),
                Style::new().fg(highlight).bg(background),
            );
        }
        // render bottom line if there's enough space
        if area.height > 1 {
            buf.set_string(
                area.x,
                area.y + area.height - 1,
                "▁".repeat(area.width as usize),
                Style::new().fg(shadow).bg(background),
            );
        }
        // render label centered
        buf.set_line(
            area.x + (area.width.saturating_sub(self.label.width() as u16)) / 2,
            area.y + (area.height.saturating_sub(1)) / 2,
            &self.label,
            area.width,
        );
    }
}

impl Button<'_> {
    const fn colors(&self) -> (Color, Color, Color, Color) {
        let button_theme = self.button_theme;
        match self.state {
            State::Normal => (button_theme.background, button_theme.text, button_theme.shadow, button_theme.highlight),
            State::Selected => (button_theme.highlight, button_theme.text, button_theme.shadow, button_theme.highlight),
            State::Active => (button_theme.background, button_theme.text, button_theme.highlight, button_theme.shadow),
        }
    }
}

pub fn handle_key_event(
    key: event::KeyEvent,
    button_states: &mut [State; 2],
    selected_button: &mut usize,
    app: &mut App
) -> ControlFlow<()> {
    match key.code {
        KeyCode::Char('q') => return ControlFlow::Break(()),
        KeyCode::Left | KeyCode::Char('h') => {
            button_states[*selected_button] = State::Normal;
            *selected_button = selected_button.saturating_sub(1);
            button_states[*selected_button] = State::Selected;
        }
        KeyCode::Right | KeyCode::Char('l') => {
            button_states[*selected_button] = State::Normal;
            *selected_button = selected_button.saturating_add(1).min(1);
            button_states[*selected_button] = State::Selected;
        }
        KeyCode::Enter => {
            if button_states[*selected_button] == State::Active {
                button_states[*selected_button] = State::Normal;
            } else {
                button_states[*selected_button] = State::Active;
                app.hashing_type = HashingType::from_index(*selected_button);
            }
        }
        _ => (),
    }
    ControlFlow::Continue(())
}


pub fn draw(frame: &mut Frame, states: [State; 2]) {
    let area = frame.area();
    let title = Line::from(" Select hashing algorithm ".bold());
    let instructions = Line::from(vec![
        " Select ".into(),
        "←/→".blue().bold(),
        " Toggle ".into(),
        "<Enter>".blue().bold(),
        " Back ".into(),
        "<Esc>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold(),
    ]);
    
    // Frame with title
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);
    let inner = block.inner(area);
    
    // Layout: [top_padding][text][middle_padding][buttons][bottom_padding]
    let vertical = Layout::vertical([
        Constraint::Percentage(25), // top padding - centers the text vertically
        Constraint::Length(7),      // height for the text made of letters
        Constraint::Percentage(25), // middle padding between text and buttons
        Constraint::Length(5),      // height for the buttons
        Constraint::Percentage(25), // bottom padding
    ]);
    let [_, word_area, _, buttons_area, _] = vertical.areas(inner);
    
    // Centered and scalable buttons according to window width
    let horizontal = Layout::horizontal([
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
        Constraint::Percentage(25),
    ]);
    let [_, left_btn_area, right_btn_area, _] = horizontal.areas(buttons_area);
    
    // Frame render
    frame.render_widget(block, area);
    
    let word_str = "hashing algorithm";
    let letter_spacing = 7.0;
    let word_len = word_str.len() as f64;
    
    // Calculate available width and height
    let available_width = word_area.width as f64;
    let available_height = word_area.height as f64;
    
    // Set the scale so that the text fits
    let scale_x = available_width / (word_len * letter_spacing);
    let scale_y = available_height / 7.0;
    let scale = scale_x.min(scale_y).max(1.0);
    
    // Center the text
    let total_word_width = word_len * letter_spacing * scale;
    let starting_x = ((available_width - total_word_width) / 2.0).max(0.0);
    
    let word = Word::new(word_str.to_string(), starting_x, scale as u8);
    frame.render_widget(
        Canvas::default()
            .x_bounds([0.0, available_width])
            .y_bounds([0.0, available_height])
            .marker(Marker::HalfBlock)
            .paint(move |ctx| {
                ctx.draw(&word);
            }),
        word_area,
    );
    
    // Button render
    frame.render_widget(
        Button::new("dHash").button_theme(RED).state(states[0]),
        left_btn_area,
    );
    frame.render_widget(
        Button::new("pHash").button_theme(BLUE).state(states[1]),
        right_btn_area,
    );
}