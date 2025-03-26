pub mod csv_data;
use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind}, layout::{Constraint, Layout, Margin, Rect}, style::{self, Modifier, Style}, text::Text, widgets::{
        Block, BorderType, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table, TableState
    }, DefaultTerminal, Frame
};
use csv_data::CSVData;

const INFO_TEXT: [&str; 1] = [
    "(Esc) quit | (↑ / k) move up | (↓ / j) move down | (← / h) move left | (→ / l) move right",
];

const ITEM_HEIGHT: usize = 4;

pub struct TableTUI {
    state: TableState,
    items: Option<CSVData>,
    column_widths: Vec<u16>,
    scroll_state: ScrollbarState,
}

impl TableTUI {
    pub fn new(data: &CSVData) -> Self {
        Self {
            state: TableState::default().with_selected(0),
            scroll_state: ScrollbarState::default().position(0),
            column_widths: Self::max_col_width(data),
            items: Some(data.clone())
        }
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

    fn max_col_width(data: &CSVData) -> Vec<u16> {
        let mut max_col: u16 = 0;
        for col in data.content.iter() {
            max_col = max_col.max(col.len() as u16);
        }
        
        let mut col_width: Vec<u16> = vec![0; max_col as usize];
        for (idx, col) in data.header.iter().enumerate() {
            if idx < col_width.len(){
                col_width[idx] = col_width[idx].max(col.len() as u16 + 2);
            } else {
                break;
            }
        }
        for row in data.content.iter() {
            for (idx, col) in row.iter().enumerate() {
                if idx < col_width.len() - 1 {
                    col_width[idx] = col_width[idx].max(col.len() as u16 + 2);
                }
            }
        }
        return col_width;
    }

    fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items
                            .as_mut()
                            .unwrap()
                            .content
                            .len()
                            .saturating_sub(1) {
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

    fn previous_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items
                        .as_mut()
                        .unwrap()
                        .content
                        .len()
                        .saturating_sub(1)
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
        self.scroll_state = self.scroll_state.position(i * ITEM_HEIGHT);
    }

    fn next_column(&mut self) {
        self.state.select_next_column();
    }

    fn previous_column(&mut self) {
        self.state.select_previous_column();
    }
    
    fn draw(&mut self, frame: &mut Frame) {
        let vertical = &Layout::vertical([Constraint::Min(5), Constraint::Length(3)]);
        let rects = vertical.split(frame.area());

        self.render_table(frame, rects[0]);
        self.render_scrollbar(frame, rects[0]);
        self.render_footer(frame, rects[1]);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let selected_row_style = Style::default()
            .add_modifier(Modifier::REVERSED);
        let selected_cell_style = Style::default()
            .add_modifier(Modifier::ITALIC)
            .bg(style::Color::Black)
            .fg(style::Color::White);
        let header_style = Style::default()
            .bg(style::Color::LightCyan)
            .fg(style::Color::White)
            .add_modifier(Modifier::BOLD);

        let mut rows: Vec<Row> = Vec::new();
        let headers: Row;

        match &self.items {
            Some(items) => {
                headers = Row::new(items.header
                    .iter()
                    .map(|h| h.as_str())
                    .collect::<Vec<_>>())
                    .style(header_style);

                for row_str in items.content.iter() {
                    let temp = Row::new(row_str.iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<_>>());
                    rows.push(temp);
                }
            }
            None => {
                log::error!("Content data is None");
                return;
            }
        }
        let col_widths: Vec<Constraint> = self.column_widths.clone()
                                        .into_iter()    
                                        .map(|length| Constraint::Length(length))
                                        .collect();
        let t = Table::new(rows, col_widths)
            .header(headers)
            .row_highlight_style(selected_row_style)
            .cell_highlight_style(selected_cell_style);
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
        let footer = Paragraph::new(Text::from(INFO_TEXT.join(" ")))
            .centered()
            .block(
                Block::bordered()
                    .border_type(BorderType::Double)
            );
        frame.render_widget(footer, area);
    }
}
