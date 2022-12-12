use crate::renderer::io::Stdout;
use crossterm::event::DisableMouseCapture;
use crossterm::event::EnableMouseCapture;
use crossterm::terminal::disable_raw_mode;
use crossterm::terminal::EnterAlternateScreen;
use crossterm::terminal::LeaveAlternateScreen;
use crossterm::{execute, terminal::enable_raw_mode};
use std::{
    io::{self, Write},
};
use tui::backend::Backend;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use super::map::Map;

pub struct Renderer {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

impl Renderer {
    // add code here
    pub fn new() -> Self {
        // setup terminal
        enable_raw_mode().unwrap();
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture).unwrap();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.hide_cursor().unwrap();

        Renderer { terminal }
    }

    pub fn draw(&mut self, map: &Map) {
        let bckend = self.terminal.backend_mut();
        let (_, bottom, _, right) = map.get_map_properties();
        let map = map.get_map();
        // let items = Vec::new();

        for x in 0..right {
            for y in 0..bottom {
                // let block = Cell{
                //     symbol: map[(right * y + x) as usize].to_string(),
                //     fg: tui::style::Color::White,
                //     bg: tui::style::Color::Reset,
                //     modifier: Modifier::empty()
                // };

                // items.push((x as u16, y as u16, &block));
                bckend.set_cursor(x as u16, y as u16).unwrap();
                let a = map[(right * y + x) as usize];
                let mut b = [0; 2];
                a.encode_utf8(&mut b);
                bckend.write(&b).expect("write error");
            }
        }

        // bckend.draw(items.iter());
    }

    pub fn clear_all(&mut self) {
        self.terminal.backend_mut().clear().unwrap();
    }

    /**
     * return (left, right, top, bottom)
     */
    pub fn get_termianl_properties(&self) -> (u16, u16, u16, u16) {
        let rect = self.terminal.size().unwrap();
        (rect.left(), rect.right(), rect.top(), rect.bottom())
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        // restore terminal
        disable_raw_mode().unwrap();
        execute!(
            self.terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )
        .unwrap();
        self.terminal.show_cursor().unwrap();
    }
}
