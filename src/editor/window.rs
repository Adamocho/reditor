use std::io::{self, Write, stdout};
use crossterm::{
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen, DisableLineWrap, EnableLineWrap},
    cursor::{self, MoveTo, SetCursorStyle}, style::Print,
    };

pub struct Window {
    window_dimensions: (u16, u16),
    position: (u16, u16),
    editor_dimensions: (u16, u16),
    line_wrapping: bool,
    line_numbers: bool,
}

// enum used to switch between modes
pub enum Mode {
    Insert,
    CapsLock,
    Normal
}

impl Window {
    pub fn new() -> Self {
        Window::enter_alternate_screen();

        let dimensions = terminal::size().unwrap();
        
        crossterm::queue!(
            io::stdout(),
            MoveTo(3, 0),
        ).unwrap();

        Self {
            window_dimensions: dimensions,
            position: cursor::position().unwrap(), 
            editor_dimensions: dimensions,
            line_wrapping: false,
            line_numbers: false,
        }
    }

    fn enter_alternate_screen() {
        let mut stdout = io::stdout();

        terminal::enable_raw_mode().unwrap();
        crossterm::execute!(
            stdout,
            EnterAlternateScreen,
            DisableLineWrap,
            SetCursorStyle::BlinkingBar,
        ).unwrap();
    }

    pub fn prepare(&self) {
        if self.line_numbers {
            self.draw_line_numbers()
        }
        if self.line_wrapping {
            self.draw_wrap_lines()
        }

        crossterm::queue!(
            io::stdout(),
            MoveTo(0, 0),
        ).unwrap();

        // the final step
        io::stdout().flush().unwrap();
    }

    fn draw_line_numbers(&self) {
        let mut stdout = io::stdout();

        for y in 0..self.window_dimensions.1 {
            crossterm::queue!(
                stdout,
                MoveTo(0, y),
                Print(y),
            ).unwrap();
        }
    }

    fn draw_wrap_lines(&self) {

    }

    pub fn draw_char_at_cursor(&mut self, c: char) {
        if c.is_control() { return }

        crossterm::queue!(
            stdout(),
            Print(c), 
        ).unwrap();

        self.position = cursor::position().unwrap();
    }

    pub fn window_width(&self) -> u16 {
        self.window_dimensions.0
    }

    pub fn window_height(&self) -> u16 {
        self.window_dimensions.1
    }

    pub fn position(&self) -> (u16, u16) {
        self.position
    }

    pub fn is_line_wrapping(&self) -> bool {
        self.line_wrapping
    }

    pub fn is_line_numbers(&self) -> bool {
        self.line_numbers
    }

    pub fn tab_backward(&self) {

    }

    pub fn tab_forward(&self) {

    }

    pub fn go_down(&self) {

    }

    pub fn go_up(&self) {

    }

    pub fn go_left(&self) {

    }

    pub fn go_word_left(&self) {

    }

    pub fn go_right(&self) {

    }

    pub fn go_word_right(&self) {

    }

    pub fn move_page_up(&self) {

    }

    pub fn move_page_down(&self) {

    }

    pub fn goto_start_of_line(&self) {

    }


    pub fn goto_end_of_line(&self) {

    }

    pub fn switch_mode(&self, mode: Mode) {
        match mode {
            Mode::CapsLock => {

            }, 
            Mode::Insert => {

            }, 
            // return everything to 'normal'
            Mode::Normal => {

            }, 
        }
    }

    pub fn remove_char_at_cursor(&self) {

    }

    pub fn remove_char_before_cursor(&self) {

    }

    pub fn newline(&self) {

    }

    // redraw window
    fn redraw(&self) {}

    // move pointer (handle moving up and down with arrows and keep/seek furthest character)
    fn move_pointer(&self) {}
}

impl Drop for Window {
    fn drop(&mut self) {
        crossterm::execute!(
            io::stdout(),
            LeaveAlternateScreen,
            EnableLineWrap,
            SetCursorStyle::DefaultUserShape,
        ).unwrap();

        terminal::disable_raw_mode().unwrap();
    }
}