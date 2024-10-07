use clap::{App, Arg};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{io, time::Duration};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame, Terminal,
};
use cypher_windexer::{Settings, rpc, storage, indexer, metrics};

struct App {
    indexed_blocks: u64,
    total_blocks: u64,
    last_error: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let matches = App::new("Cypher Windexer CLI")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("CLI tool for Cypher Windexer")
        .arg(
            Arg::with_name("config")
                .short('c')
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    let config_path = matches.value_of("config").unwrap_or("config/default.toml");
    let settings = Settings::new(config_path)?;

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let rpc_client = rpc::SolanaRpcClient::new(&settings.solana.rpc_url);
    let storage = storage::create_storage(&settings.database)?;
    let indexer = indexer::Indexer::new(rpc_client.clone(), storage.clone());

    let mut app = App {
        indexed_blocks: 0,
        total_blocks: rpc_client.get_slot()? as u64,
        last_error: None,
    };

    let res = run_app(&mut terminal, &mut app, indexer);

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("Error: {:?}", err)
    }

    Ok(())
}

fn run_app<B: tui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    mut indexer: indexer::Indexer,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                if let KeyCode::Char('q') = key.code {
                    return Ok(());
                }
            }
        }

        if let Err(e) = indexer.index_next_block() {
            app.last_error = Some(format!("Error indexing block: {:?}", e));
        } else {
            app.indexed_blocks += 1;
            app.total_blocks = indexer.rpc_client.get_slot().unwrap() as u64;
            metrics::increment_indexed_blocks();
        }
    }
}

fn ui<B: tui::backend::Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(f.size());

    let title = Paragraph::new("Cypher Windexer")
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(title, chunks[0]);

    let progress = (app.indexed_blocks as f64 / app.total_blocks as f64 * 100.0) as u16;
    let gauge = Gauge::default()
        .block(Block::default().title("Indexing Progress").borders(Borders::ALL))
        .gauge_style(Style::default().fg(Color::Yellow))
        .ratio(progress as f64 / 100.0)
        .label(format!("{}%", progress));
    f.render_widget(gauge, chunks[1]);

    let stats = Paragraph::new(vec![
        Spans::from(vec![
            Span::raw("Indexed Blocks: "),
            Span::styled(
                app.indexed_blocks.to_string(),
                Style::default().fg(Color::Green),
            ),
        ]),
        Spans::from(vec![
            Span::raw("Total Blocks: "),
            Span::styled(
                app.total_blocks.to_string(),
                Style::default().fg(Color::Blue),
            ),
        ]),
    ])
    .block(Block::default().title("Statistics").borders(Borders::ALL));
    f.render_widget(stats, chunks[2]);

    let error_msg = app
        .last_error
        .as_ref()
        .map(|e| Spans::from(Span::styled(e, Style::default().fg(Color::Red))))
        .unwrap_or_else(|| Spans::from(Span::raw("No errors")));

    let errors = Paragraph::new(vec![error_msg])
        .block(Block::default().title("Last Error").borders(Borders::ALL));
    f.render_widget(errors, chunks[3]);
}