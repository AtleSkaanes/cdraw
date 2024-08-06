use std::{
    fmt::Display,
    io::Stdout,
    ops::{Deref, DerefMut},
};

use crossterm::{cursor, event, style::Print, terminal, ExecutableCommand};

pub struct StreamHandler {
    pub stdout: Stdout,
    pub layers: Vec<Vec<Vec<char>>>,
}

impl StreamHandler {
    pub fn new(stdout: Stdout, width: u16, height: u16) -> Self {
        let v = vec![vec![vec![' '; width as usize]; height as usize]];

        Self { stdout, layers: v }
    }
    pub fn start(&mut self) -> std::io::Result<()> {
        terminal::enable_raw_mode()?;
        self.execute(terminal::EnterAlternateScreen)?;
        self.execute(cursor::Hide)?;
        self.execute(cursor::SetCursorStyle::SteadyBar)?;
        self.execute(event::EnableMouseCapture)?;

        Ok(())
    }

    pub fn print_at(
        &mut self,
        string: String,
        layer: usize,
        col: u16,
        row: u16,
    ) -> std::io::Result<()> {
        for (i, c) in string.chars().enumerate() {
            self.layers[layer][row as usize][col as usize + i] = c;
        }
        self.execute(cursor::MoveTo(col, row))?
            .execute(Print(string))?;

        Ok(())
    }

    pub fn print(&mut self, string: String, layer: usize) -> std::io::Result<()> {
        let (col, row) = cursor::position()?;

        for (i, c) in string.chars().enumerate() {
            self.layers[layer][row as usize][col as usize + i] = c;
        }

        self.execute(Print(string))?;

        Ok(())
    }

    pub fn get_char(&self, layer: usize, col: u16, row: u16) -> Option<char> {
        self.layers
            .get(layer)
            .unwrap_or(&vec![])
            .get(row as usize)
            .unwrap_or(&vec![])
            .get(col as usize)
            .cloned()
    }
}

impl Drop for StreamHandler {
    fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Should be able to disable raw mode");
        self.execute(terminal::LeaveAlternateScreen)
            .expect("Should be able to leave alt screen");

        self.execute(cursor::Show)
            .expect("Should be able to show cursor");
    }
}

impl Deref for StreamHandler {
    type Target = Stdout;
    fn deref(&self) -> &Self::Target {
        &self.stdout
    }
}

impl DerefMut for StreamHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.stdout
    }
}
