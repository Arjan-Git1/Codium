use crate::input::input;
use crate::keys;
use crate::{io, mode::Mode};
use core::error;
use crossterm::event::{KeyCode, MouseEvent};
use crossterm::terminal;
use crossterm::terminal::{Clear, ClearType};
use crossterm::{cursor, execute};
use piecetable::PieceTable;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::text::{Line, Span};
use ratatui::{
    backend::CrosstermBackend,
    layout::Position,
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::{stdout, Error, Write};
use std::path::Path;

use syntect::{easy::HighlightLines, highlighting::ThemeSet, parsing::SyntaxSet};
#[derive(Debug)]
pub struct Editor {
    pub mode: Mode,
    pub cursor_x: u16,
    pub cursor_y: u16,
    pub document: PieceTable,
    pub scroll_x: usize,
    pub scroll_y: usize,
    pub visible_height: usize,
    pub syntax_set: SyntaxSet,
    pub theme_set: ThemeSet,
}

impl Editor {
    fn clamp_cursor(&mut self) {
        let text = self.document.display_result().unwrap();
        let lines: Vec<&str> = text.split('\n').collect();

        if self.cursor_y as usize >= lines.len() {
            self.cursor_y = lines.len().saturating_sub(1) as u16;
            self.cursor_x = 0;
        }

        let line = lines.get(self.cursor_y as usize).unwrap_or(&"");

        if self.cursor_x as usize > line.len() {
            self.cursor_x = line.len() as u16;
        }
    }

    fn recompute_cursor_y(&mut self) {
        let text = self.document.display_result().unwrap();
        let mut count = 0usize;
        let index = self.cursor_offset(&text, self.cursor_x, self.cursor_y) as usize;

        for (i, line) in text.split('\n').enumerate() {
            if index <= count + line.len() {
                self.cursor_y = i as u16;
                return;
            }
            count += line.len() + 1;
        }

        self.cursor_y = text.split('\n').count().saturating_sub(1) as u16;
    }
    pub fn render(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
        path: &String,
        extension: &String,
    ) -> io::Result<()> {
        terminal.draw(|mut frame| {
            let bg = Block::default().style(Style::default().bg(Color::Rgb(40, 40, 40)));

            frame.render_widget(bg, frame.area());

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(1)])
                .split(frame.area());

            let editor_area = chunks[0];
            let status_area = chunks[1];

            self.visible_height = editor_area.height.saturating_sub(2) as usize;

            let text = self.document.display_result().unwrap();
            let lines: Vec<&str> = text.split('\n').collect();

            let syntax = self.syntax_set.find_syntax_by_extension(extension).unwrap();

            let mut highlighter =
                HighlightLines::new(syntax, &self.theme_set.themes["base16-eighties.dark"]);

            let mut rendered_lines = Vec::new();

            for line in lines.iter().skip(self.scroll_y).take(self.visible_height) {
                let ranges = highlighter.highlight_line(line, &self.syntax_set).unwrap();

                let spans = ranges
                    .into_iter()
                    .map(|(style, text)| {
                        Span::styled(
                            text.to_string(),
                            Style::default()
                                .fg(Color::Rgb(
                                    style.foreground.r,
                                    style.foreground.g,
                                    style.foreground.b,
                                ))
                                .add_modifier(Modifier::BOLD),
                        )
                    })
                    .collect::<Vec<_>>();

                rendered_lines.push(Line::from(spans));
            }

            let paragraph = Paragraph::new(rendered_lines)
                .style(
                    Style::default()
                        .fg(Color::Rgb(235, 219, 178))
                        .bg(Color::Rgb(40, 40, 40)),
                )
                .block(Block::default().title(path.as_str()).borders(Borders::ALL));

            frame.render_widget(paragraph, editor_area);

            let status_text = format!(
                " {} | Ln {} Col {} | {} lines | {:?} ",
                path,
                self.cursor_y + 1,
                self.cursor_x + 1,
                lines.len(),
                self.mode
            );

            let status_bar = Paragraph::new(status_text).style(
                Style::default()
                    .bg(Color::Blue)
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            );

            frame.render_widget(status_bar, status_area);

            let screen_y = self.cursor_y.saturating_sub(self.scroll_y as u16);

            frame.set_cursor_position((self.cursor_x + 1, screen_y + 1));
        })?;

        Ok(())
    }

    pub fn input(&mut self, code: &Result<KeyCode, Error>) {
        let text = self.document.display_result().unwrap();
        let lines: Vec<&str> = text.split('\n').collect();

        self.clamp_cursor();

        let current_line = lines.get(self.cursor_y as usize).unwrap_or(&"");

        let index = self.cursor_offset(
            self.document.display_result().unwrap().as_str(),
            self.cursor_x,
            self.cursor_y,
        );

        let index_usize = index as usize;

        match code {
            Ok(KeyCode::Up) => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                }
            }

            Ok(KeyCode::Down) => {
                self.cursor_y += 1;
            }

            Ok(KeyCode::Right) => {
                if (self.cursor_x as usize) < current_line.len() {
                    self.cursor_x += 1;
                }
            }

            Ok(KeyCode::Left) => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            }

            Ok(KeyCode::Backspace) => {
                keys::backspace(&mut self.document, index_usize);
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                }
            }

            Ok(KeyCode::Enter) => {
                self.document.insert("\n", index_usize);
                self.cursor_x = 0;
                self.cursor_y += 1;
            }

            Ok(KeyCode::Home) => {
                self.cursor_x = 0;
            }

            _ => {}
        }

        self.recompute_cursor_y();
        self.scroll(self.visible_height);
    }

    pub fn cursor_offset(&mut self, text: &str, cursor_x: u16, cursor_y: u16) -> u16 {
        let lines: Vec<&str> = text.split('\n').collect();
        let current_line = lines.get(self.cursor_y as usize).unwrap_or(&"");

        let mut offset: u16 = 0;
        let cursor_x = cursor_x.min(current_line.len() as u16);

        for y in 0..cursor_y {
            if let Some(line) = lines.get(y as usize) {
                offset += line.len() as u16 + 1;
            }
        }

        offset + cursor_x
        //get cursor offset
    }

    pub fn insert(&mut self, code: &Result<KeyCode, Error>) {
        let index = self.cursor_offset(
            self.document.display_result().unwrap().as_str(),
            self.cursor_x,
            self.cursor_y,
        );

        let index_usize = index as usize;

        match code {
            Ok(KeyCode::Char(c)) => {
                self.document.insert(&c.to_string(), index_usize);
                self.cursor_x += 1;
            }

            Ok(KeyCode::Esc) => {
                self.mode = Mode::Normal;
            }

            _ => {}
        }
    }

    pub fn normal(&mut self, code: &Result<KeyCode, Error>, path: &Path) {
        match code {
            Ok(KeyCode::Char('q')) => {
                self.mode = Mode::Quit;
            }

            Ok(KeyCode::Char('s')) => {
                self.document.save_to_file(Path::new(path)).unwrap();
            }

            _ => {}
        }
    }

    pub fn scroll(&mut self, visible_height: usize) {
        let line = self.get_cursor_line();

        if line >= self.scroll_y + visible_height {
            self.scroll_y = line - visible_height + 1;
        }

        if line < self.scroll_y {
            self.scroll_y = line;
        }
    }

    pub fn get_cursor_line(&mut self) -> usize {
        let text = self.document.display_result().unwrap();
        let mut count = 0;

        let target = self.cursor_offset(&text, self.cursor_x, self.cursor_y) as usize;

        for (i, line) in text.split('\n').enumerate() {
            if target <= count + line.len() {
                return i;
            }
            count += line.len() + 1;
        }

        text.split('\n').count().saturating_sub(1)
    }
}
