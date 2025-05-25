use std::time::{Duration, Instant};
use std::thread::sleep;
use std::path::PathBuf;

use crate::handler::handle::HashingType;
use crate::handler::similarity_analyzer::SimilarityAnalyzer;
use crate::ui::UI;
use color_eyre::{eyre::Ok, Result};
use ratatui::{DefaultTerminal};

const DURATION: Duration = Duration::from_millis(17);

// logic and handling of the app

pub enum CurrentScreen {
    FolderChoose,
    ChooseAnAlgorithm,
    Calculating,
    Main
}

pub struct App {
    pub current_screen: CurrentScreen,
    pub dir_path: Option<PathBuf>,
    pub hashing_type: Option<HashingType>,
    pub similarity_analyzer: Option<SimilarityAnalyzer>,
    pub time_start: Option<Instant>,
    pub time_elapsed: String,
    pub items_list: Option<Vec<PathBuf>>,
    pub exit: bool
}

impl App {
    pub fn new() -> Self {
        App {
            current_screen: CurrentScreen::FolderChoose,
            dir_path: None,
            hashing_type: None,
            similarity_analyzer: None,
            time_start: None,
            time_elapsed: String::new(),
            items_list: None,
            exit: false
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let mut ui = UI::new()?;

        while !self.exit {
            let mut ui_result = Ok(());
            // draw a frame composed by set_ui() function
            terminal.draw(|f| {ui_result = ui.set_ui(f, self);})?;
            ui_result?;

            sleep(DURATION); // to limit CPU usage
        }

        Ok(())
    }

    pub fn stop(&mut self) {
        self.exit = true;
    }
    
}