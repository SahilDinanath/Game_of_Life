use std::{io::stdout, io::Result, thread::sleep, time::Duration, usize};

use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    style::Stylize,
    widgets::{
        canvas::{Canvas, Points},
        Block,
    },
    Terminal,
};

fn main() -> Result<()> {
    //settings
    const SIZE: usize = 200;
    let background = 0;
    let cell = 1;
    let spawn_chance = 0.1;
    let cell_color = ratatui::style::Color::White;
    let update_speed = Duration::from_millis(100);

    let mut matrix = [[background; SIZE]; SIZE];
    let mut updated_points: Vec<(f64, f64, i8)> = vec![];

    //(row, column)
    let moves = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    //init matrix with cells
    for row in 0..SIZE {
        for column in 0..SIZE {
            if rand::thread_rng().gen_bool(spawn_chance) {
                matrix[row][column] = cell;
            }
        }
    }

    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    loop {
        //TODO implement stored points to update, needed to add extra field to keep update
        //type ie. dead or alive
        for row in 0..SIZE {
            for column in 0..SIZE {
                //how many cells are around current cell
                let mut counter = 0;
                for (y, x) in moves {
                    //calculate the next move
                    let next_row = row as i32 + y;
                    let next_column = column as i32 + x;

                    //if next move is out of bounds, don't update
                    let row = if next_row < 0 || next_row as usize >= SIZE {
                        continue;
                    } else {
                        next_row as usize
                    };

                    let column = if next_column < 0 || next_column as usize >= SIZE {
                        continue;
                    } else {
                        next_column as usize
                    };

                    if matrix[row][column] == cell {
                        counter += 1;
                    }
                }
                //rules of life
                //https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life#Rules
                if counter < 2 || counter > 3 {
                    updated_points.push((row as f64, column as f64, background));
                } else if counter == 3 {
                    updated_points.push((row as f64, column as f64, cell));
                }
            }
        }

        //Update matrix with new values
        for _i in 0..updated_points.len() {
            let (row, column, state): (f64, f64, i8) = updated_points
                .pop()
                .expect("Error occured when popping from stack.");
            matrix[row as usize][column as usize] = state;
        }

        terminal.draw(|frame| {
            let area = frame.size();
            let canvas = Canvas::default()
                .block(Block::default())
                .paint(|ctx| {
                    //
                    let mut matrix_points: [(f64, f64); SIZE * SIZE] = [(0.0, 0.0); SIZE * SIZE];
                    let mut counter = 0;
                    for row in 0..SIZE {
                        for column in 0..SIZE {
                            if matrix[row][column] == cell {
                                matrix_points[counter] = (column as f64, row as f64);
                                counter += 1;
                            }
                        }
                    }
                    ctx.draw(&Points {
                        coords: &matrix_points,
                        color: cell_color,
                    })
                })
                .x_bounds([0.0, SIZE as f64])
                .y_bounds([0.0, SIZE as f64]);

            frame.render_widget(canvas, area);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
        sleep(update_speed);
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
