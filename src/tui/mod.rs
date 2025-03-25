use std::{error::Error, io::{self, Write}};
use termion::{clear, cursor, event::Key, input::TermRead, raw::IntoRawMode, terminal_size};
use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Margin, Rect},
    style::{self, Modifier, Style}, 
    text::Text, widgets::{
        Block, BorderType, HighlightSpacing, 
        Paragraph, Row, Scrollbar, 
        ScrollbarOrientation,ScrollbarState, 
        Table, TableState,
    }, DefaultTerminal, Frame
};

const INFO_TEXT: [&str; 1] = [
    "(Esc) quit | (↑) move up | (↓) move down | (←) move left | (→) move right",
];

const ITEM_HEIGHT: usize = 4;

struct CSVData {
    header: Vec<String>,
    content: Vec<Vec<String>>
}

pub struct TableTUI {
    state: TableState,
    items: CSVData,
    column_widths: Vec<u16>,
    scroll_state: ScrollbarState,
}

impl TableTUI {
    pub fn new() -> Self {
        Self {
            state: TableState::default().with_selected(0),
            column_widths: vec![20, 20, 20], // Default widths for each column
            scroll_state: ScrollbarState::default().position(0),
            items: CSVData{
                header: vec![String::from("TEST1"), String::from("TEST2"), String::from("TEST3")],
                content: vec![
                    vec![String::from("1"), String::from("2"), String::from("3")],
                    vec![String::from("1"), String::from("2"), String::from("3")],
                    vec![String::from("1"), String::from("2"), String::from("3")],
                ]
            }
        }
    }
    pub fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.content.len().saturating_sub(1) {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn previous_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.content.len().saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    pub fn next_column(&mut self) {
        self.state.select_next_column();
    }

    pub fn previous_column(&mut self) {
        self.state.select_previous_column();
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                        KeyCode::Char('j') | KeyCode::Down => self.next_row(),
                        KeyCode::Char('k') | KeyCode::Up => self.previous_row(),
                        KeyCode::Char('l') | KeyCode::Right => self.next_column(),
                        KeyCode::Char('h') | KeyCode::Left => self.previous_column(),
                        _ => {}
                    }
                }
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
        let rects = vertical.split(frame.area());

        self.render_table(frame, rects[0]);
        self.render_scrollbar(frame, rects[0]);
        self.render_footer(frame, rects[1]);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED);
        let selected_col_style = Style::default()
            .bg(style::Color::Green)
            .fg(style::Color::White);
        let selected_cell_style = Style::default()
            .add_modifier(Modifier::REVERSED)
            .bg(style::Color::Black)
            .fg(style::Color::White);

        let headers = Row::new(self.items.header
                                                        .iter()
                                                        .map(|h| h.as_str())
                                                        .collect::<Vec<_>>());                                                  
        let mut rows: Vec<Row> = Vec::new();
        for row_str in self.items.content.iter() {
            let temp = Row::new(row_str.iter()
                                                        .map(|s| s.as_str())
                                                        .collect::<Vec<_>>());
            rows.push(temp);
        }
        let bar = " █ ";
        let t = Table::new(
            rows,
            [
                Constraint::Length(10 + 1),
                Constraint::Length(10 + 1),
                Constraint::Length(10 + 1),
            ],
        )
        .header(headers)
        .row_highlight_style(selected_row_style)
        .column_highlight_style(selected_col_style)
        .cell_highlight_style(selected_cell_style)
        .highlight_symbol(Text::from(vec![
            "".into(),
            bar.into(),
            bar.into(),
            "".into(),
        ]))
        .highlight_spacing(HighlightSpacing::Always);
        frame.render_stateful_widget(t, area, &mut self.state);
    }

    fn render_scrollbar(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_stateful_widget(
            Scrollbar::default()
                .orientation(ScrollbarOrientation::VerticalRight)
                .begin_symbol(None)
                .end_symbol(None),
            area.inner(Margin {
                vertical: 1,
                horizontal: 1,
            }),
            &mut self.scroll_state,
        );
    }

    fn render_footer(&self, frame: &mut Frame, area: Rect) {
        let info_footer = Paragraph::new(Text::from_iter(INFO_TEXT))
            .centered()
            .block(
                Block::bordered()
                    .border_type(BorderType::Double)
            );
        frame.render_widget(info_footer, area);
    }
}

pub fn show(file_content: String) -> Result<(), Box<dyn Error>>{
    let mut stdout = io::stdout().into_raw_mode().unwrap();
    let mut stdin = io::stdin().keys();
    
    // Clear screen and move cursor to top
    write!(stdout, "{}{}{}", clear::All, cursor::Goto(1, 1), cursor::BlinkingBlock)?;

    // Get terminal size
    let (mut width, _) = terminal_size()?.into();
    
    // Get content width
    let content_width = file_content.lines()
                                            .next()
                                            .map_or(0, |line| line.len());
    
    // Adjust terminal size if content is wider
    while content_width > width as usize {
        let new_width = content_width + 10; // Add some padding
        width = new_width as u16;
    }
    write!(stdout, "\x1b[8;{}t", width)?;
    stdout.flush()?;
    
    
    // Center the content if possible
    if content_width < width as usize {
        for line in file_content.lines() {
            write!(stdout, "{}\r\n", line)?;
        }
    }
    
    stdout.flush()?;

    loop {
        match stdin.next() {
            Some(Ok(Key::Ctrl('c'))) => {
                write!(stdout, "\r\n{}\r\nExiting...\r\n", cursor::Show)?;
                stdout.flush()?;
                break;
            }
            Some(Err(e)) => eprintln!("Error: {}", e),
            None => (),
            _ => {}
        }
    }
    Ok(())
}
