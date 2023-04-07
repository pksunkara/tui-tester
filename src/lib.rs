use std::{
    fmt::Debug,
    io::{Result, Write},
    os::fd::AsRawFd,
};

use vt100::{Cell, Parser, Screen};

#[derive(Default)]
pub struct Terminal {
    parser: Parser,
}

impl Terminal {
    #[must_use]
    pub fn new(rows: u16, cols: u16, scrollback_len: usize) -> Self {
        Self {
            parser: Parser::new(rows, cols, scrollback_len),
        }
    }

    pub fn screen(&self) -> &Screen {
        self.parser.screen()
    }

    pub fn cell(&self, row: u16, col: u16) -> Option<&Cell> {
        self.screen().cell(row, col)
    }
}

impl Debug for Terminal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Terminal").finish()
    }
}

impl AsRawFd for Terminal {
    fn as_raw_fd(&self) -> i32 {
        0
    }
}

impl Write for Terminal {
    fn write(&mut self, content: &[u8]) -> Result<usize> {
        self.parser.process(content);
        Ok(content.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use vt100::Color;

    #[test]
    fn write() {
        let mut term = Terminal::default();

        write!(term, "He\x1b[31m\x1B[Dxll\x1b[mo").unwrap();

        let cell = term.cell(0, 0).unwrap();
        assert_eq!(cell.contents(), "H");
        assert_eq!(cell.fgcolor(), Color::Default);
        assert_eq!(cell.bgcolor(), Color::Default);

        let cell = term.cell(0, 1).unwrap();
        assert_eq!(cell.contents(), "x");
        assert_eq!(cell.fgcolor(), Color::Idx(1));
        assert_eq!(cell.bgcolor(), Color::Default);

        let cell = term.cell(0, 2).unwrap();
        assert_eq!(cell.contents(), "l");
        assert_eq!(cell.fgcolor(), Color::Idx(1));
        assert_eq!(cell.bgcolor(), Color::Default);

        let cell = term.cell(0, 3).unwrap();
        assert_eq!(cell.contents(), "l");
        assert_eq!(cell.fgcolor(), Color::Idx(1));
        assert_eq!(cell.bgcolor(), Color::Default);

        let cell = term.cell(0, 4).unwrap();
        assert_eq!(cell.contents(), "o");
        assert_eq!(cell.fgcolor(), Color::Default);
        assert_eq!(cell.bgcolor(), Color::Default);
    }
}
