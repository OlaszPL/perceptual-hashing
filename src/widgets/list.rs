use ratatui::{
    style::{Stylize},
    text::{Line},
    widgets::{Block, List, ListState},
    symbols::{border},
    style::Style,
    Frame
};
use ratatui_image::StatefulImage;

use crate::{app::App, ui::UI};

pub fn draw_list(frame: &mut Frame, app: &mut App, ui: &mut UI) {
    use ratatui::layout::{Layout, Constraint, Direction};

    let area = frame.area();

    // 3 equal column blocks
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(34), Constraint::Percentage(33)])
        .split(area);

    let title0 = Line::from(format!(" {} ", app.dir_path.as_ref().unwrap().to_str().unwrap().bold())).blue();
    let title1 = Line::from(format!(" {} ", app.hashing_type.as_ref().unwrap()).bold()).green();
    let title2 = Line::from(app.time_elapsed.as_str().bold()).yellow();

    let instructions0 = Line::from(vec![
        " Select ".into(),
        "↑/↓ ".blue().bold(),
        "←/→ ".blue().bold(),
    ]);
    let instructions2 = Line::from(vec![
        " Back ".into(),
        "<Esc>".blue().bold(),
        " Quit ".into(),
        "<Q> ".blue().bold()
    ]);

    // list in block0

    let mut state0 = ListState::default();
    state0.select(Some(ui.selected_button));

    let list0 = List::new(
        app.items_list.as_ref().unwrap().iter().map(|x| x.file_name().unwrap().to_str().unwrap())
        ).block(
            Block::bordered()
                .title(title0.left_aligned())
                .title_bottom(instructions0.left_aligned())
                .border_set(border::THICK)
        )
        .highlight_style(Style::default().bg(ratatui::style::Color::Blue).fg(ratatui::style::Color::White))
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list0, chunks[0], &mut state0);
    // end of list0

    // list in block1
    
    let items1 = app.similarity_analyzer
        .as_ref()
        .unwrap()
        .get_one_file_similarity(&app.items_list.as_ref().unwrap()[ui.selected_button]);

    let mut state1 = ListState::default();
    state1.select(Some(ui.selected_button_2));

    let desc = Line::from(" Distance -> Filename ".bold());

    let list1 = List::new(
        items1.iter()
        .filter(|(path, _)| path != &app.items_list.as_ref().unwrap()[ui.selected_button]) // not ot show duplicated file
        .map(|(path, dist)| format!("{} -> {}", dist, path.file_name().unwrap().to_str().unwrap()))
        )
        .block(
            Block::bordered()
                .title(title1.centered())
                .title_bottom(desc.centered())
                .border_set(border::THICK)
        )
        .highlight_style(Style::default().bg(ratatui::style::Color::Green).fg(ratatui::style::Color::White))
        .highlight_symbol(">> ");

    ui.files_num_column_1 = items1.len() - 1; // without duplicated file

    frame.render_stateful_widget(list1, chunks[1], &mut state1);
    // end of list1

    // right column splitted in the middle
    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[2]);

    // Upper half: image in a frame
    if let Some(ref mut image_mid) = ui.image_mid {
        let image_block = Block::bordered()
            .title(" Selected ")
            .title(title2.right_aligned())
            .border_set(border::THICK);
        let image_area = image_block.inner(right_chunks[0]);
        frame.render_widget(image_block, right_chunks[0]);
        let image = StatefulImage::default();
        frame.render_stateful_widget(image, image_area, image_mid);
    }

    // lower half
    if let Some(ref mut image_right) = ui.image_right {
        let image_block = Block::bordered()
            .title(" Similar ")
            .title_bottom(instructions2.right_aligned())
            .border_set(border::THICK);
        let image_area = image_block.inner(right_chunks[1]);
        frame.render_widget(image_block, right_chunks[1]);
        let image = StatefulImage::default();
        frame.render_stateful_widget(image, image_area, image_right);
    }
}
