use ratatui::{
    Frame, layout::{Constraint, Direction, Layout, Margin}, style::{Color, Modifier, Style}, symbols::Marker, text::Text, widgets::{Block, List, ListState, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, canvas::{Canvas, Circle, Line, Rectangle}}
};
use crate::app::{App, SelectedBox};

pub fn ui(frame: &mut Frame, app: &App) {
    let box_style_normal = Style::new();
    let box_style_selected = Style::new().green();

    /*
     * Main horizontal layout
     */
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(10),
        ])
        .split(frame.area());
    
    /*
     * Title
     */
    let title = Paragraph::new(Text::styled(
        "spaceautomats",
        Style::default().fg(Color::Green),
        ))
        .block(Block::bordered());
    frame.render_widget(title, chunks[0]);

    /*
     * Main gameplay area
     */
    let gameplay = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Percentage(80)
        ])
        .split(chunks[1]);

    let items: Vec<String> = app.sim.get_automats().into_iter().map(|automat| {
         format!("{} (💜 {}) (🔋 {})", automat.ship_hw.get_name(), automat.ship_hw.get_health(), automat.ship_hw.propulsion.get_fuel())
    }).collect();
    let automats_list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ")
        .block(Block::bordered().border_style(if app.selected_box == SelectedBox::Automats { box_style_selected } else { box_style_normal }));

    let mut state = ListState::default().with_selected(Some(app.selected_automat));
    frame.render_stateful_widget(automats_list, gameplay[0], &mut state);

    let canvas = Canvas::default()
        .marker(Marker::Braille)
        .x_bounds([0.0, app.args.x as f64])
        .y_bounds([0.0, app.args.y as f64])
        .paint(|ctx| {
            for automat in app.sim.get_automats() {
                let pos = automat.ship_hw.object.get_pos();
                let dir = automat.ship_hw.object.get_dir();

                // Highlight
                if automat.get_id() == app.selected_automat as u32 {
                    ctx.draw(&Rectangle {
                        x: (pos.0 as f64) - 50000.0,
                        y: (pos.1 as f64) - 50000.0,
                        width: 100000.0,
                        height: 100000.0,
                        color: Color::DarkGray
                    });
                }
                ctx.draw(&Line {
                    x1: pos.0 as f64,
                    y1: pos.1 as f64,
                    x2: pos.0 as f64 + 75000.0 * dir.cos(),
                    y2: pos.1 as f64 + 75000.0 * dir.sin(),
                    color: Color::White,
                });
                ctx.draw(&Circle {
                    x: pos.0 as f64,
                    y: pos.1 as f64,
                    radius: 8000.0,
                    color: Color::Red,});
                
            }
            for plasma in app.sim.get_plasmas() {
                let pos = plasma.object.get_pos();
                ctx.draw(&Circle {
                    x: pos.0 as f64,
                    y: pos.1 as f64,
                    radius: 1000.0,
                    color: Color::Green,});
            }
        });
    frame.render_widget(canvas, gameplay[1]);

    /*
     * Log
     */
    let selected_automat = app.sim.get_automats()[app.selected_automat];
    let selected_log = selected_automat.ship_hw.get_log();
    let nlogs = selected_log.split("\n").count();

    let vertical_scroll = nlogs.saturating_sub(10); // from app state
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));
    let log = Paragraph::new(selected_log)
        .block(Block::bordered().border_style(if app.selected_box == SelectedBox::Log { box_style_selected } else { box_style_normal })
        .title(format!("Log {}", selected_automat.ship_hw.get_name())))
        .scroll((vertical_scroll as u16, 0));
    let mut scrollbar_state = ScrollbarState::new(nlogs).position(vertical_scroll);

    frame.render_widget(log, chunks[2]);
    frame.render_stateful_widget(scrollbar,chunks[2]
        .inner(Margin {
            vertical: 1, // using an inner vertical margin of 1 unit makes the scrollbar inside the block
            horizontal: 0,
        }), &mut scrollbar_state);
}