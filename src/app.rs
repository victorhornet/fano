use color_eyre::eyre::Result;
use itertools::Itertools;
use std::path::PathBuf;

pub struct Editor {
    pub path: Option<PathBuf>, // the file to append to, or create if it doesn't exist
    pub input: String,         // the next word/line to append to the file
    pub text: String,          // the text of the file
    pub line_width: u16,       // the width of the editor
}

impl Editor {
    /// Opens a new editor for the file from the given path.
    pub fn new(path: Option<PathBuf>) -> Result<Self> {
        let text = path
            .as_ref()
            .map(fano::read_file)
            .unwrap_or(Ok(String::new()))?;
        let mut editor = Self {
            input: String::new(),
            line_width: 60,
            path,
            text,
        };
        editor.reload();
        Ok(editor)
    }

    /// Is called when the user types a character.
    /// Appends the character to the input.
    pub fn type_char(&mut self, c: char) {
        self.input.push(c);
        if self.input.len() as u16 >= self.line_width {
            self.append();
        }
    }

    /// Is called when the user presses the backspace key.
    /// Removes the last character from the input.
    /// If the input is empty, removes the last character from the text.
    pub fn backspace(&mut self) {
        if self.input.pop().is_none() {
            self.text.pop();
            self.reload();
        }
    }

    /// Is called when the user types a character and the
    /// input reaches the line width.
    fn append(&mut self) {
        self.text.push_str(&self.input);
        self.input.clear();
    }

    /// Is called when the user presses the enter key.
    /// Appends the input to the text, then adds a newline.
    pub fn appendln(&mut self) {
        self.text.push_str(&self.input);
        self.text.push('\n');
        self.input.clear();
    }

    /// Is called when the user presses ESC to exit.
    /// Saves the text to the file.
    pub fn save(&mut self) -> Result<()> {
        if !self.input.is_empty() {
            self.append();
        }
        if let Some(p) = &self.path {
            fano::write_file(p, &self.text)?
        }
        Ok(())
    }

    /// Returns whether the text ends in newline
    pub fn newline(&self) -> bool {
        self.text.ends_with('\n')
    }

    /// Loads the last line of text into the input line, given by its width.
    pub fn reload(&mut self) {
        let last_line = self
            .lines()
            .last()
            .cloned()
            .map(|lc| lc.text)
            .unwrap_or_default();
        if last_line.chars().count() < self.line_width as usize {
            self.input = last_line;
            self.text.truncate(self.text.len() - self.input.len());
        }
    }

    /// Returns the chunks of text that fit into the line width.
    pub fn lines(&self) -> Vec<LineChunk> {
        self.lines_iter().collect()
    }

    fn lines_iter(&self) -> impl Iterator<Item = LineChunk> + '_ {
        self.text
            .lines()
            .flat_map(|line| split_line(line, self.line_width))
    }
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

#[derive(Clone)]
pub struct LineChunk {
    pub text: String,
    pub is_last: bool,
}
