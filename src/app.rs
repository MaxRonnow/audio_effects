use crate::EffectParams;
use crate::ui;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use std::io;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

pub fn init_ui(running: Arc<AtomicBool>, ui_params: Arc<EffectParams>) -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut app = App::new(running, ui_params);
    let app_result = app.run(&mut terminal);
    disable_raw_mode()?;
    ratatui::restore();
    app_result
}

#[derive(Debug, Default)]
pub struct App<'a> {
    running: Arc<AtomicBool>,
    pub tabs: TabsState<'a>,
    effect_params: Arc<EffectParams>,
    param_selection: ParamSelection,
}

impl<'a> App<'a> {
    pub fn new(running: Arc<AtomicBool>, effect_params: Arc<EffectParams>) -> Self {
        App {
            tabs: TabsState::new(vec!["Distorion", "Delay", "Reverb"]),
            running,
            effect_params,
            param_selection: ParamSelection::new(),
        }
    }

    /// runs the application's main loop until the user quits
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        enable_raw_mode()?;
        while self.running.load(Ordering::SeqCst) {
            terminal.draw(|frame| ui::draw(frame, self))?;
            self.handle_events()?;
        }
        Ok(())
    }

    /// updates the application's state based on user input
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Left => self.previous_tab(),
            KeyCode::Right => self.next_tab(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.running
            .store(false, std::sync::atomic::Ordering::SeqCst);
    }

    fn next_tab(&mut self) {
        self.tabs.next();
    }

    fn previous_tab(&mut self) {
        self.tabs.previous();
    }
}

#[derive(Debug, Default)]
pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub const fn new(titles: Vec<&'a str>) -> Self {
        Self { titles, index: 0 }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

#[derive(Debug, Default)]
pub struct ParamSelection {
    delay_index: usize,
    distortion_index: usize,
    reverb_index: usize,
}

impl ParamSelection {
    pub fn new() -> Self {
        Self {
            delay_index: 0,
            distortion_index: 0,
            reverb_index: 0,
        }
    }

    pub fn next(&mut self, selected_effect: usize) {
        match selected_effect {
            0 => self.distortion_index = (self.distortion_index + 1) % 3,
            1 => self.delay_index = (self.delay_index + 1) % 3,
            2 => self.reverb_index = (self.delay_index + 1) % 3,
            _ => {}
        }
    }
}
