use crossterm::event::{KeyEvent, KeyModifiers, KeyCode};

pub mod window;
pub mod keyboard;
pub mod piece_table;

pub fn editor() {

    // Creating the window.
    // When it goes out of scope, it automatically
    // Goes back to normal screen, cleaning up
    let mut window = window::Window::new();
    window.prepare();

    // the main editor loop
    loop {
        if let Some(key) = keyboard::Keyboard::read_key() {
            match key {
                // KeyEvent { code: KeyCode::F(4), modifiers: KeyModifiers::ALT, .. } => todo!("Exit"),
                // Do not save changes on exit
                KeyEvent { 
                    code: KeyCode::Esc,
                    .. } => break,

                KeyEvent { 
                    code: KeyCode::Char('s'),
                    modifiers: KeyModifiers::CONTROL,
                    .. } => {
                        // file.save();
                    },

                KeyEvent { 
                    code: KeyCode::Char('z'),
                    modifiers: KeyModifiers::CONTROL,
                    .. } => {
                        // file.undo();
                    },

                KeyEvent { 
                    code: KeyCode::Char('y'),
                    modifiers: KeyModifiers::CONTROL,
                    .. } => {
                        // file.redo();
                    },
                    
                // return to 'normal' mode
                KeyEvent { 
                    code: KeyCode::Char('n'),
                    modifiers: KeyModifiers::CONTROL,
                    .. } => {
                        window.switch_mode(window::Mode::Normal);
                    },

                KeyEvent { 
                    code: KeyCode::Tab,
                    modifiers: KeyModifiers::SHIFT,
                    .. } => {
                        window.tab_backward();
                    },

                KeyEvent { 
                    code: KeyCode::Tab,
                    .. } => {
                        window.tab_forward();
                    },

                KeyEvent { 
                    code: KeyCode::Down,
                    .. } => {
                        window.go_down();
                    },

                KeyEvent { 
                    code: KeyCode::Up,
                    .. } => {
                        window.go_up();
                    },

                KeyEvent { 
                    code: KeyCode::Left,
                    modifiers: KeyModifiers::CONTROL,
                    .. } => {
                        window.go_word_left();
                    },

                KeyEvent { 
                    code: KeyCode::Left,
                    .. } => {
                        window.go_left();
                    },
                
                KeyEvent { 
                    code: KeyCode::Right,
                    modifiers: KeyModifiers::CONTROL,
                    .. } => {
                        window.go_word_right();
                    },

                KeyEvent { 
                    code: KeyCode::Right,
                    .. } => {
                        window.go_right();
                    },

                KeyEvent { 
                    code: KeyCode::PageUp,
                    .. } => {
                        window.move_page_up();
                    },

                KeyEvent { 
                    code: KeyCode::PageDown,
                    .. } => {
                        window.move_page_down();
                    },

                KeyEvent { 
                    code: KeyCode::Home,
                    .. } => {
                        window.goto_start_of_line();
                    },

                KeyEvent { 
                    code: KeyCode::End,
                    .. } => {
                        window.goto_end_of_line();
                    },

                KeyEvent { 
                    code: KeyCode::Enter,
                    .. } => {
                        window.newline();
                    },

                KeyEvent { 
                    code: KeyCode::Delete,
                    .. } => {
                        window.remove_char_at_cursor();
                    },

                KeyEvent { 
                    code: KeyCode::Insert,
                    .. } => {
                        window.switch_mode(window::Mode::Insert);
                    },

                KeyEvent { 
                    code: KeyCode::CapsLock,
                    .. } => {
                        window.switch_mode(window::Mode::CapsLock);
                    },

                KeyEvent { 
                    code: KeyCode::Backspace,
                    .. } => {
                        window.remove_char_before_cursor();
                    },

                KeyEvent { 
                    code: KeyCode::Char(c),
                    .. } => {
                        window.draw_char_at_cursor(c);
                    },

                _ => (),
            };
        }
    }

    // window is dropped and the terminal
    // returns to it's original state
}