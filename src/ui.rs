use crate::app::Editor;
use itertools::Itertools;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(f: &mut Frame<'_>, editor: &mut Editor) {
    let Rect { width, height, .. } = f.size();
    let paragraph_x = (width - editor.line_width) / 2;
    let paragraph_y = height / 2 - 1;

    // Itertools::intersperse(editor.text.lines(), " ")
    let mut lines = editor
        .text
        .lines()
        .flat_map(|line| split_line(line, editor.line_width))
        .collect_vec();
    lines.reverse();
    lines
        .into_iter()
        .enumerate()
        .take(5)
        .fold(paragraph_y, |y, (i, line)| {
            // set new paragraph one line higher
            let mut y = y - 1;
            // handle newlines
            if i != 0 && line.is_last {
                y -= 1;
            }
            if i == 0 && editor.newline() {
                y -= 1;
            }
            f.render_widget(
                Paragraph::new(line.text)
                    .block(Block::default().borders(Borders::NONE))
                    .style(Style::default().fg(Color::White)),
                Rect::new(paragraph_x, y, editor.line_width, 1),
            );
            y
        });

    f.render_widget(
        Paragraph::new(editor.input.as_str())
            .block(Block::default().borders(Borders::NONE))
            .style(Style::default().fg(Color::White).bold()),
        Rect::new(paragraph_x, paragraph_y, editor.line_width, 1),
    );
    f.render_widget(
        Paragraph::new("ENTER to create new line; BACKSPACE to delete")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::NONE))
            .style(Style::default().fg(Color::Blue)),
        Rect::new(0, height - 4, width, 1),
    );
    f.render_widget(
        Paragraph::new("ESC to save and quit.")
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::NONE))
            .style(Style::default().fg(Color::Blue)),
        Rect::new(0, height - 3, width, 2),
    );
    f.set_cursor(paragraph_x + editor.input.len() as u16, paragraph_y);
}

fn split_line(line: &str, width: u16) -> Vec<LineChunk> {
    let line = if line.is_empty() { " " } else { line };
    let mut line = line
        .chars()
        .chunks(width as usize)
        .into_iter()
        .map(|chunk| LineChunk {
            text: chunk.collect(),
            is_last: false,
        })
        .collect_vec();
    if let Some(last) = line.last_mut() {
        last.is_last = true
    }
    line
}

struct LineChunk {
    pub text: String,
    pub is_last: bool,
}
