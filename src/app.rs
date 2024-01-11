use color_eyre::eyre::Result;
use std::path::PathBuf;

pub struct Editor {
    pub path: PathBuf,   // the file to append to, or create if it doesn't exist
    pub input: String,   // the next word/line to append to the file
    pub text: String,    // the text of the file
    pub line_width: u16, // the width of the editor
}

impl Editor {
    /// Opens a new editor for the file from the given path.
    pub fn new(path: PathBuf) -> Result<Self> {
        let text = wrt::read_file(&path)?;
        Ok(Self {
            input: String::new(),
            line_width: 60,
            path,
            text,
        })
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
        wrt::write_file(&self.path, &self.text)?;
        Ok(())
    }

    /// Returns whether the text ends in newline
    pub fn newline(&self) -> bool {
        self.text.ends_with('\n')
    }
}
