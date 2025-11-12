use crate::ui::App;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Clear},
    Frame,
};

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(frame.area());

    render_header(frame, chunks[0], app);
    render_container_list(frame, app, chunks[1]);
    render_footer(frame, chunks[2]);

    if app.show_delete_confirm {
        render_delete_confirmation(frame, app);
    }
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let title = if let Some(msg) = &app.status_message {
        format!("Docker Container Manager - {}", msg)
    } else {
        format!("Docker Container Manager - {} containers", app.containers.len())
    };

    let header = Paragraph::new(title)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(header, area);
}

fn render_container_list(frame: &mut Frame, app: &App, area: Rect) {
    if app.containers.is_empty() {
        let empty_msg = Paragraph::new("No containers found. Start some containers to see them here.")
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL).title("Containers"));
        frame.render_widget(empty_msg, area);
        return;
    }

    let items: Vec<ListItem> = app
        .containers
        .iter()
        .map(|container| {
            let name = container
                .names
                .as_ref()
                .and_then(|n| n.first())
                .map(|s| s.trim_start_matches('/'))
                .unwrap_or("unknown");

            let status = container.state.as_deref().unwrap_or("unknown");
            let image = container.image.as_deref().unwrap_or("unknown");
            let id = container.id.as_deref().unwrap_or("unknown");
            let short_id = if id.len() >= 12 { &id[..12] } else { id };

            let status_color = match status {
                "running" => Color::Green,
                "exited" => Color::Red,
                "paused" => Color::Yellow,
                _ => Color::Gray,
            };

            let (health_indicator, health_color) = get_health_status(container);

            let content = vec![Line::from(vec![
                Span::styled(
                    format!("{:<28}", name),
                    Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{:<3}", health_indicator),
                    Style::default().fg(health_color).add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    format!("{:<12}", short_id),
                    Style::default().fg(Color::DarkGray),
                ),
                Span::styled(
                    format!("{:<12}", status),
                    Style::default().fg(status_color),
                ),
                Span::styled(
                    format!("{}", image),
                    Style::default().fg(Color::Gray),
                ),
            ])];

            ListItem::new(content)
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Containers"))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    frame.render_stateful_widget(list, area, &mut app.list_state.clone());
}

fn get_health_status(container: &bollard::models::ContainerSummary) -> (&'static str, Color) {
    if let Some(state) = &container.state {
        if state != "running" {
            return ("", Color::Gray);
        }
    }

    container
        .status
        .as_ref()
        .and_then(|status| {
            if status.contains("(healthy)") {
                Some(("●", Color::Green))
            } else if status.contains("(unhealthy)") {
                Some(("●", Color::Red))
            } else if status.contains("(health: starting)") || status.contains("(starting)") {
                Some(("◐", Color::Yellow))
            } else {
                None
            }
        })
        .unwrap_or(("", Color::Gray))
}

fn render_footer(frame: &mut Frame, area: Rect) {
    let help_text = vec![
        Span::raw("↑/↓: Navigate  "),
        Span::styled("s", Style::default().fg(Color::Green)),
        Span::raw(": Start  "),
        Span::styled("x", Style::default().fg(Color::Red)),
        Span::raw(": Stop  "),
        Span::styled("r", Style::default().fg(Color::Yellow)),
        Span::raw(": Restart  "),
        Span::styled("d", Style::default().fg(Color::Red)),
        Span::raw(": Delete  "),
        Span::styled("R", Style::default().fg(Color::Cyan)),
        Span::raw(": Refresh  "),
        Span::styled("q", Style::default().fg(Color::Cyan)),
        Span::raw(": Quit  "),
        Span::styled("●", Style::default().fg(Color::Green)),
        Span::raw(": Healthy  "),
        Span::styled("●", Style::default().fg(Color::Red)),
        Span::raw(": Unhealthy"),
    ];

    let footer = Paragraph::new(Line::from(help_text))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(footer, area);
}

fn render_delete_confirmation(frame: &mut Frame, app: &App) {
    let area = centered_rect(50, 20, frame.area());

    let container_name = app
        .list_state
        .selected()
        .and_then(|i| app.containers.get(i))
        .and_then(|c| c.names.as_ref())
        .and_then(|n| n.first())
        .map(|s| s.trim_start_matches('/'))
        .unwrap_or("unknown");

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Delete Container?",
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(
            container_name,
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
        Line::from("Press 'y' to confirm, 'n' to cancel"),
    ];

    let popup = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Red))
        );

    frame.render_widget(Clear, area);
    frame.render_widget(popup, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}