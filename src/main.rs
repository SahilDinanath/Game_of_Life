use std::{thread::sleep, time::Duration, usize};

use rand::Rng;
fn main() {
    //settings
    const SIZE: usize = 20;
    let background = ' ';
    let cell = '■';
    let update_speed = Duration::from_millis(200);
    let spawn_chance = 0.09;

    let mut matrix = [[background; SIZE]; SIZE];

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

    loop {
        for row in 0..SIZE {
            for column in 0..SIZE {
                //how many cells are around current cell
                let mut counter = 0;
                for (y, x) in moves {
                    //calculate the next move
                    let next_row = row as i8 + y;
                    let next_column = column as i8 + x;

                    //if next move is out of bounds, don't update
                    let row = if next_row < 0 || next_row as usize >= SIZE {
                        row
                    } else {
                        next_row as usize
                    };

                    let column = if next_column < 0 || next_column as usize >= SIZE {
                        column
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
                    matrix[row][column] = background;
                } else if counter == 3 {
                    matrix[row][column] = cell;
                }
            }
        }
        for row in matrix {
            println!("{:?}", row);
        }
        println!("");
        sleep(update_speed);
    }
}
