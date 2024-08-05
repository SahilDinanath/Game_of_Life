use clap::Parser;
use std::{
    io::{stdout, Result},
    process::exit,
    thread::sleep,
    time::Duration,
    usize,
};

use rand::Rng;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, KeyCode, KeyEventKind},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    widgets::{
        canvas::{Canvas, Points},
        Block,
    },
    Terminal,
};

///A Game Of Life simulator in the terminal.
#[derive(Parser, Debug)]
#[command(name = "gol", version, about, long_about = None)]
struct Args {
    ///Speed of simulation in milliseconds
    #[arg(short, long, default_value_t = 50)]
    speed: u64,
    ///Density of simulation, multiple of total allowable cells
    #[arg(short, long, default_value_t = 2)]
    density: usize,

    ///Initial spawn rate of cells with probability between 0.0 - 1.0
    #[arg(short, long, default_value_t = 0.05)]
    rate: f64,

    ///Color of cells. prefix: none or bright. Values: red, green, blue, yellow, magenta, cyan, grey etc
    #[arg(short, long, default_value_t = ratatui::style::Color::White)]
    color: ratatui::style::Color,

    ///Color of background. prefix: none or bright. Values: red, green, blue, yellow, magenta, cyan, grey etc
    #[arg(short, long, default_value_t = ratatui::style::Color::Black)]
    background_color: ratatui::style::Color,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let size = terminal
        .size()
        .expect("Unable to determine height and width of terminal.");

    //settings
    //used to increase point density
    let multiplier = if args.density > 0 && args.density <= 10 {
        args.density
    } else {
        println!("Invalid cell density. Range [1 - 10].");
        exit(1)
    };

    //used to keep aspect ratio
    let height = size.height as usize * multiplier;
    let width = size.width as usize * multiplier;

    let spawn_chance = if args.rate >= 0.0 && args.rate <= 1.0 {
        args.rate
    } else {
        println!("Invalid spawn rate. Range [0.0 - 1.0].");
        exit(1)
    };

    let cell_color = args.color;
    let background_color = args.background_color;
    let update_speed = if args.speed > 0 && args.speed <= 1000 {
        Duration::from_millis(args.speed)
    } else {
        println!("Invalid speed in ms. Range [1 - 1000].");
        exit(1)
    };

    let background = 0;
    let cell = 1;
    let mut matrix = vec![vec![background; width]; height];
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
    for row in 0..height {
        for column in 0..width {
            if rand::thread_rng().gen_bool(spawn_chance) {
                matrix[row][column] = cell;
            }
        }
    }

    //setup terminal
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    terminal.clear()?;

    loop {
        //draw first to display initial grid
        terminal.draw(|frame| {
            let area = frame.size();
            let canvas = Canvas::default()
                .background_color(background_color)
                .block(Block::default())
                .paint(|ctx| {
                    //
                    let mut matrix_points = vec![(0.0, 0.0); height * width];
                    let mut counter = 0;
                    for row in 0..height {
                        for column in 0..width {
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
                .x_bounds([0.0, width as f64])
                .y_bounds([0.0, height as f64]);

            frame.render_widget(canvas, area);
        })?;

        //read keyboard input to quit
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
        //sleep after display
        sleep(update_speed);

        //Calculate next state
        for row in 0..height {
            for column in 0..width {
                //how many cells are around current cell
                let mut counter = 0;
                for (y, x) in moves {
                    //calculate the next move
                    let next_row = row as i32 + y;
                    let next_column = column as i32 + x;

                    //if next move is out of bounds, don't update
                    let row = if next_row < 0 || next_row as usize >= height {
                        continue;
                    } else {
                        next_row as usize
                    };

                    let column = if next_column < 0 || next_column as usize >= width {
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
            let (row, column, state) = updated_points
                .pop()
                .expect("Error occured when popping from stack.");
            matrix[row as usize][column as usize] = state;
        }
    }

    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
