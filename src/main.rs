#[allow(non_upper_case_globals)]
mod tictactoe;
use crossterm::{
    self, cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute, terminal,
};
use std::io;
use tictactoe::TicTacToe;
use tui::layout::Alignment;
use tui::Terminal;
use tui::{backend::CrosstermBackend, widgets::BorderType};
use tui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, Paragraph},
};

fn main() -> crossterm::Result<()> {
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    terminal::enable_raw_mode()?;

    execute!(
        io::stdout(),
        cursor::Hide,
        cursor::SavePosition,
        terminal::EnterAlternateScreen
    )?;

    let mut game = TicTacToe::new();

    let keys = vec![vec![7, 8, 9], vec![4, 5, 6], vec![1, 2, 3]];

    let mut x_score = 0;
    let mut o_score = 0;

    loop {
        match game.winner().unwrap_or_default().as_str() {
            "X" => {
                x_score += 1;
                game.reset();
            }
            "O" => {
                o_score += 1;
                game.reset();
            }
            _ => {
                if game.is_full() {
                    game.reset()
                }
            }
        };
        terminal.draw(|f| {
            let root = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(75), Constraint::Percentage(25)])
                .margin(4)
                .split(f.size());

            let top = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                ])
                .split(root[0]);

            let bottom = Layout::default()
                .direction(Direction::Horizontal)
                .horizontal_margin(10)
                .constraints([
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                    Constraint::Ratio(1, 3),
                ])
                .split(root[1]);
            for x in 0..3 {
                let top1 = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Ratio(1, 3),
                        Constraint::Ratio(1, 3),
                        Constraint::Ratio(1, 3),
                    ])
                    .split(top[x]);
                for y in 0..3 {
                    let paragraph =
                        Paragraph::new(game.get(keys[x][y]))
                            .block(Block::default().title(Span::styled(
                                format!("                   {}", keys[x][y]),
                                Style::default().fg(Color::Rgb(129, 161, 193)).add_modifier(
                                    Modifier::BOLD | Modifier::ITALIC | Modifier::DIM,
                                ),
                            )))
                            .alignment(Alignment::Center);
                    f.render_widget(paragraph, top1[y]);
                }
            }

            let x: Paragraph = Paragraph::new(game.pretty_num(x_score, 'X'))
                .block(
                    Block::default()
                        .title("X Score")
                        .borders(Borders::all())
                        .border_style(Style::default().fg(Color::Rgb(129, 161, 193)))
                        .border_type(BorderType::Rounded),
                )
                .alignment(Alignment::Center);
            f.render_widget(x, bottom[0]);
            let o: Paragraph = Paragraph::new(game.pretty_num(o_score, 'O'))
                .block(
                    Block::default()
                        .title("O Score")
                        .borders(Borders::all())
                        .border_style(Style::default().fg(Color::Rgb(129, 161, 193)))
                        .border_type(BorderType::Rounded),
                )
                .alignment(Alignment::Center);
            f.render_widget(o, bottom[1]);
            let turn: Paragraph = Paragraph::new(game.turn())
                .block(
                    Block::default()
                        .title("Turn")
                        .borders(Borders::all())
                        .border_style(Style::default().fg(Color::Rgb(129, 161, 193)))
                        .border_type(BorderType::Rounded),
                )
                .alignment(Alignment::Center);
            f.render_widget(turn, bottom[2]);
        })?;

        match event::read()? {
            Event::Key(key) => {
                if key.modifiers.contains(KeyModifiers::CONTROL) {
                    if let KeyCode::Char('c') = key.code {
                        terminal.clear()?;
                        break;
                    };
                }
                match key.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        terminal.clear()?;
                        terminal::disable_raw_mode()?;
                        execute!(
                            io::stdout(),
                            cursor::RestorePosition,
                            cursor::Show,
                            terminal::LeaveAlternateScreen,
                        )?;
                        break;
                    }
                    KeyCode::Char(num) => {
                        let num = num.to_digit(10).unwrap_or_default();
                        if game.is_full() {
                            game.reset();
                        }
                        if num != 0 {
                            game.play(num as i32)
                        };
                    }
                    _ => {}
                }
            }
            _ => {}
        }

        if x_score > 99 {
            terminal.clear()?;
            terminal::disable_raw_mode()?;
            execute!(
                io::stdout(),
                cursor::RestorePosition,
                cursor::Show,
                terminal::LeaveAlternateScreen,
            )?;
            println!("X WON!");
            break;
        } else if o_score > 99 {
            terminal.clear()?;
            terminal::disable_raw_mode()?;
            execute!(
                io::stdout(),
                cursor::RestorePosition,
                cursor::Show,
                terminal::LeaveAlternateScreen,
            )?;
            println!("O WON!");
            break;
        }
    }
    Ok(())
}
