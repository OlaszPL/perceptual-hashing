use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::sync::mpsc::{Receiver, channel};
use std::thread::spawn;
use std::cmp::max;

use ratatui::{prelude::*, widgets::*, Frame, symbols::{border, Marker}, widgets::canvas::Canvas};
use ratatui_explorer::{FileExplorer, Theme};
use ratatui_image::{picker::Picker, protocol::StatefulProtocol};
use color_eyre::{eyre::Ok, Result, Report};
use crossterm::event::{poll, read, Event, KeyCode, KeyEventKind};
use image::ImageReader;
use crate::{app::{App, CurrentScreen}, handler::similarity_analyzer::SimilarityAnalyzer};
use crate::widgets::{algorithm_chooser::*,letters::*, list::draw_list};

const POLL_DURATION: Duration = Duration::from_millis(50);

pub enum ImageTarget {Mid, Right}

pub struct UI {
    file_explorer: FileExplorer,
    pub selected_button: usize,
    button_states: [State; 2],
    similarity_analyzer_rx: Option<Receiver<Result<SimilarityAnalyzer, Report>>>,
    pub selected_button_2: usize,
    pub selected_column: usize,
    pub files_num_column_1: usize,
    // for image preview
    pub image_mid: Option<StatefulProtocol>,
    pub image_mid_rx: Option<Receiver<StatefulProtocol>>,
    // for the next image
    pub image_right: Option<StatefulProtocol>,
    pub image_right_rx: Option<Receiver<StatefulProtocol>>
}

impl UI {
    pub fn new() -> Result<Self> {
        let theme = Theme::default()
            .add_default_title()
            .with_title_top(|_fe| {
                // Centred title
                Line::from(
                    Span::styled(
                        "Select a folder with images",
                        Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                    )
                ).alignment(Alignment::Center)
            })
            .with_title_bottom(|fe| {
                // Number of files aligned to the left
                Line::from(
                    Span::styled(
                        format!("[{} files]", fe.files().len()),
                        Style::default().fg(Color::Green)
                    )
                )
            })
            .with_title_bottom(|_fe|{
                // Info how to pick a folder
                Line::from(
                    Span::styled(
                        "Press 'c' to select a folder",
                        Style::default().fg(Color::Red)
                    )
                ).alignment(Alignment::Center)
            })
            .with_title_bottom(|_fe| {
                // Info aligned to the right
                Line::from(
                    Span::styled(
                        "Press 'q' to exit",
                        Style::default().fg(Color::Yellow)
                    )
                ).alignment(Alignment::Right)
            })
            .with_block(Block::default().borders(Borders::ALL).border_type(BorderType::Rounded))
            .with_highlight_item_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .with_highlight_dir_style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
            .with_highlight_symbol("> ");
    
        Ok(Self {
            file_explorer: FileExplorer::with_theme(theme)?,
            selected_button: 0,
            button_states: [State::Selected, State::Normal],
            similarity_analyzer_rx: None,
            selected_button_2: 0,
            selected_column: 0,
            files_num_column_1: 0,
            image_mid: None,
            image_mid_rx: None,
            image_right: None,
            image_right_rx: None
        })
    }

    pub fn start_async_image_load(&mut self, path: PathBuf, target: ImageTarget) -> Result<()> {
        let picker = Picker::from_query_stdio()?;
        let (tx, rx) = channel();
        match target {
            ImageTarget::Mid => self.image_mid_rx = Some(rx),
            ImageTarget::Right => self.image_right_rx = Some(rx),
        }
        spawn(move || {
            if let std::result::Result::Ok(image_reader) = ImageReader::open(&path)
                && let std::result::Result::Ok(image_source) = image_reader.decode() {
                    let protocol = picker.new_resize_protocol(image_source);
                    tx.send(protocol).ok();
                }
        });
        Ok(())
    }

    pub fn load_second_img(&mut self, app: &mut App) -> Result<()> {
        self.start_async_image_load(app.similarity_analyzer
            .as_ref()
            .unwrap()
            .get_one_file_similarity(&app.items_list.as_ref().unwrap()[self.selected_button])
            .iter()
            .filter(|(path, _)| path != &app.items_list.as_ref().unwrap()[self.selected_button])
            .collect::<Vec<_>>()
            [self.selected_button_2].0.clone(),
            ImageTarget::Right)?;
        Ok(())
    }

    pub fn set_ui(&mut self, f: &mut Frame, app: &mut App) -> Result<()> {
        match app.current_screen {

            CurrentScreen::FolderChoose => {

                f.render_widget(&self.file_explorer.widget(), f.area());
                // Read the next event from the terminal.
                if poll(POLL_DURATION)? {
                    let event = read()?;
                    if let Event::Key(key) = event {
                        match key.code {
                            KeyCode::Char('q') => {
                                app.stop();
                            }
                            KeyCode::Char('c') => {
                                // get selected element
                                let selected = self.file_explorer.current();
                                if selected.is_dir() {
                                    // set directory path
                                    app.dir_path = Some(selected.path().to_path_buf());
                                    // change screen
                                    app.current_screen = CurrentScreen::ChooseAnAlgorithm;
                                    return Ok(())
                                }
                            }
                            _ => {}
                        }
                    }
                    // Handle the event in the file explorer.
                    self.file_explorer.handle(&event).ok(); // ok not to crash because of the permissions
                }
            },

            CurrentScreen::ChooseAnAlgorithm => {
                draw(f, self.button_states);
                if poll(POLL_DURATION)? {
                    if let Event::Key(key) = read()? {
                        if handle_key_event(key, &mut self.button_states, &mut self.selected_button, app).is_break() {
                            app.stop();
                        }
                        if key.kind == KeyEventKind::Press && key.code == KeyCode::Esc {
                            app.current_screen = CurrentScreen::FolderChoose;
                            return Ok(())
                        }
                    }

                    if app.hashing_type.is_some() {                        
                        // initialize similarity_analyzer in a different thread - nonblocking
                        // create a channel
                        let (tx, rx) = channel();
                        let dir_path = app.dir_path.as_ref().unwrap().clone();
                        let hashing_type = app.hashing_type.unwrap();

                        // save time stamp
                        app.time_start = Some(Instant::now());
                        
                        // spawn the thread (with error propagation)
                        spawn(move || {
                            let result = SimilarityAnalyzer::new(dir_path, hashing_type);
                            tx.send(result).ok();
                        });

                        // store the receiver in app
                        self.similarity_analyzer_rx = Some(rx);
                        
                        // change screen
                        app.current_screen = CurrentScreen::Calculating;
                        return Ok(())
                    }
                }
            },

            CurrentScreen::Calculating => {
                let area = f.area();
                let time = Line::from(format!(" Time: {:.2}s ", app.time_start.unwrap().elapsed().as_secs_f32())).yellow();
                let title = Line::from(format!(" Selected algorithm: {} ", app.hashing_type.unwrap()).bold()).green();
                let instructions = Line::from(vec![
                    " Quit ".into(),
                    "<Q> ".blue().bold(),
                ]);
                
                // Frame
                let block = Block::bordered()
                    .title(title.centered())
                    .title(time.right_aligned())
                    .title_bottom(instructions.centered())
                    .border_set(border::THICK);

                let inner = block.inner(area);

                let vertical = Layout::vertical([
                    Constraint::Percentage(50), // top padding
                    Constraint::Length(7),      // height of the large text
                    Constraint::Length(2),      // spacing
                    Constraint::Length(2),      // height of the small text
                    Constraint::Percentage(35), // bottom padding
                ]);
                let [_, word_area, _, info_area, _] = vertical.areas(inner);

                let word_str = "calculating";
                let letter_spacing = 7.0;
                let word_len = word_str.len() as f64;

                let available_width = word_area.width as f64;
                let available_height = word_area.height as f64;

                let scale_x = available_width / (word_len * letter_spacing);
                let scale_y = available_height / 7.0;
                let scale = scale_x.min(scale_y).max(1.0);

                let total_word_width = word_len * letter_spacing * scale;
                let starting_x = ((available_width - total_word_width) / 2.0).max(0.0);

                let word = Word::new(word_str.to_string(), starting_x, scale as u8);

                f.render_widget(
                    Canvas::default()
                        .x_bounds([0.0, available_width])
                        .y_bounds([0.0, available_height])
                        .marker(Marker::HalfBlock)
                        .paint(move |ctx| {
                            ctx.draw(&word);
                        }),
                    word_area,
                );

                let info = Paragraph::new("Please wait...")
                    .alignment(Alignment::Center)
                    .style(Style::default().fg(Color::Gray));
                f.render_widget(info, info_area);

                // Frame render
                f.render_widget(block, area);

                if poll(POLL_DURATION)?
                    && let Event::Key(key) = read()?
                        && key.code == KeyCode::Char('q') {
                            app.stop();
                        }

                // check whether an analyzer is initializated
                if let Some(rx) = &self.similarity_analyzer_rx
                    && let std::result::Result::Ok(analyzer) = rx.try_recv() {
                        app.time_elapsed = format!(" Time: {:.2}s ", app.time_start.unwrap().elapsed().as_secs_f32());
                        app.similarity_analyzer = Some(analyzer?);

                        self.similarity_analyzer_rx = None; // free
                        self.selected_button = 0; // re-use
                        app.time_start = None;

                        let mut items: Vec<PathBuf> = app.similarity_analyzer
                            .as_ref()
                            .unwrap()
                            .similarity_map
                            .keys()
                            .cloned()
                            .collect();

                        if items.len() < 2 { // not enough files
                            self.selected_button = 0;
                            app.hashing_type = None;
                            app.similarity_analyzer = None;
                            app.time_start = None;
                            app.items_list = None;

                            app.current_screen = CurrentScreen::FolderChoose;
                            return Ok(())
                        }

                        items.sort();
                        app.items_list = Some(items); // to keep consistent list in every iteration

                        // initalize structures for photo preview
                        // disabled on windows, won't fail on other unsupported terminals
                        #[cfg(not(target_os = "windows"))]
                        {
                            if let std::result::Result::Ok(picker) = Picker::from_query_stdio() {
                                let image_source_mid = ImageReader::open(
                                    app.items_list.as_ref().unwrap().first().unwrap()
                                )?.decode()?;
                                self.image_mid = Some(picker.new_resize_protocol(image_source_mid));
                                // and the next
                                let image_source_right = ImageReader::open(
                                    &app.similarity_analyzer
                                            .as_ref()
                                            .unwrap()
                                            .get_one_file_similarity(&app.items_list.as_ref().unwrap()[self.selected_button])
                                            .iter()
                                            .filter(|(path, _)| path != &app.items_list.as_ref().unwrap()[self.selected_button])
                                            .collect::<Vec<_>>()
                                            [self.selected_button_2].0 // read from the tuple
                                    )?.decode()?;
                                self.image_right = Some(picker.new_resize_protocol(image_source_right));
                            }
                        }

                        app.current_screen = CurrentScreen::Main; // change screen

                        return Ok(())
                    }
            },
            
            CurrentScreen::Main => {
                if let Some(rx) = &self.image_mid_rx
                    && let std::result::Result::Ok(protocol) = rx.try_recv() {
                        self.image_mid = Some(protocol);
                        self.image_mid_rx = None;
                        // start loading second img corelated with the first
                        self.load_second_img(app)?;
                    }
                if let Some(rx) = &self.image_right_rx
                    && let std::result::Result::Ok(protocol) = rx.try_recv() {
                        self.image_right = Some(protocol);
                        self.image_right_rx = None;
                    }
                
                draw_list(f, app, self);

                if poll(POLL_DURATION)?
                    && let Event::Key(key) = read()?
                        && key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Char('q') => app.stop(),
                                KeyCode::Char('j') | KeyCode::Down => {
                                    if self.selected_column == 0 {
                                        self.selected_button = (self.selected_button + 1) % app.similarity_analyzer.as_ref().unwrap().similarity_map.len();
                                        self.selected_button_2 = 0;
                                        // set a new image
                                        if self.image_mid.is_some() {
                                            self.start_async_image_load(app.items_list.as_ref().unwrap()[self.selected_button].clone(), ImageTarget::Mid)?;
                                        }
                                    }
                                    else {
                                        self.selected_button_2 = (self.selected_button_2 + 1) % self.files_num_column_1;
                                        // set a new image
                                        if self.image_mid.is_some() {
                                            self.load_second_img(app)?;
                                        }
                                    }
                                },
                                KeyCode::Char('k') | KeyCode::Up => {
                                    if self.selected_column == 0 {
                                        let max = app.similarity_analyzer.as_ref().unwrap().similarity_map.len();
                                        self.selected_button = (self.selected_button + max - 1) % max;
                                        self.selected_button_2 = 0;
                                        // set a new image
                                        if self.image_mid.is_some() {
                                            self.start_async_image_load(app.items_list.as_ref().unwrap()[self.selected_button].clone(), ImageTarget::Mid)?;
                                        }
                                    }
                                    else {
                                        self.selected_button_2 = (self.selected_button_2 + self.files_num_column_1 - 1) % self.files_num_column_1;
                                        // set a new image
                                        if self.image_mid.is_some() {
                                            self.load_second_img(app)?;
                                        }
                                    }
                                },
                                KeyCode::Char('h') | KeyCode::Left => {
                                    self.selected_column = self.selected_column.saturating_sub(1);
                                }
                                KeyCode::Char('l') | KeyCode::Right => {
                                    self.selected_column = max(self.selected_column + 1, 1);
                                }
                                KeyCode::Esc => {
                                    self.image_right_rx = None;
                                    self.image_right = None;
                                    self.image_mid_rx = None;
                                    self.image_mid = None;
                                    self.files_num_column_1 = 0;
                                    self.selected_column = 0;
                                    self.selected_button_2 = 0;
                                    self.selected_button = 0;
                                    app.hashing_type = None;
                                    app.similarity_analyzer = None;
                                    app.time_start = None;
                                    app.items_list = None;

                                    app.current_screen = CurrentScreen::ChooseAnAlgorithm;
                                }
                                _ => {}
                            }
                        }
            }
        }

        Ok(())
    }
}