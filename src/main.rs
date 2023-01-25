extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, PressEvent, Button, Key};
use piston::window::WindowSettings;
use rand::Rng;

const MAXCOLUMNS: usize = 7;
const MAXROWS: usize = 6;

// Render constants
const BLOCK_SIZE: f64 = 50.0;
const BLOCK_SPACING: f64 = 10.0;
const POSITION_LEFT: f64 = 150.0;
const POSITION_BOTTOM: f64 = 400.0;

struct GameStruct {
    board: [[i32;MAXROWS];MAXCOLUMNS],
    player_turn: i32,   // 0 if no player turn yet, 1 = player 1's turn, 2 = player 2's turn
    player_won: i32     // 0 if no winner yet, 1 = player 1 won, 2 = player 2 won, 3 is stalemate
}

pub struct App {
    gl: GlGraphics     // OpenGL drawing backend
}

impl App {
    const DARKGREEN: [f32; 4] = [0.0, 0.2, 0.0, 1.0];
    const LIGHTGREEN: [f32; 4] = [0.0, 0.3, 0.0, 1.0];
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const RED: [f32; 4] = [0.8, 0.0, 0.0, 1.0];
    const GRAYBLACK: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
    const LIGHTRED: [f32; 4] = [0.3, 0.0, 0.0, 1.0];
    const PURPLE: [f32; 4] = [0.3, 0.0, 0.3, 1.0];

    fn render(&mut self, args: &RenderArgs, game: &GameStruct) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 50.0);

        self.gl.draw(args.viewport(), |c, gl| {

            // clear the screen
            match game.player_won {
                1 => clear(Self::LIGHTRED, gl),     // indicate red (human) won
                2 => clear(Self::GRAYBLACK, gl),    // indicate black (Jesse's superior 1000x AI) won
                3 => clear(Self::PURPLE, gl),       // stalemate
                _ => clear(Self::DARKGREEN, gl),    // green field to play
            }

            // draw the game board
            for _row in 0..MAXROWS {
                for _col in 0..MAXCOLUMNS {

                    let transform1 = c
                        .transform
                        .trans(POSITION_LEFT, POSITION_BOTTOM)
                        .trans((BLOCK_SIZE + BLOCK_SPACING) * (_col as f64) - BLOCK_SIZE, -60.0 * (_row as f64) - BLOCK_SIZE);

                    // Draw a box around the middle of the screen.
                    if game.board[_col][_row] == 1 {
                        rectangle(Self::RED, square, transform1, gl);
                    }
                    else if game.board[_col][_row] == 2 {
                        rectangle(Self::BLACK, square, transform1, gl);
                    }
                    else {
                        rectangle(Self::LIGHTGREEN, square, transform1, gl);
                    }
                }
            }
        });
    }
}


fn main() {

    // Instantiate game data
    let mut game = GameStruct {
        board: [[0;MAXROWS];MAXCOLUMNS],
        player_turn: 1,
        player_won: 0
    };

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window (which is the Piston backing window)
    let mut window: Window = WindowSettings::new("Connect-4", [600, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
    };

    let mut events = Events::new(EventSettings::new());
    let mut coin_placed: bool;

    while let Some(e) = events.next(&mut window) {

        // user input
        game.player_turn = 1;

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::D1 => coin_placed = add_coin_to_column(&mut game, 0),
                Key::D2 => coin_placed = add_coin_to_column(&mut game, 1),
                Key::D3 => coin_placed = add_coin_to_column(&mut game, 2),
                Key::D4 => coin_placed = add_coin_to_column(&mut game, 3),
                Key::D5 => coin_placed = add_coin_to_column(&mut game, 4),
                Key::D6 => coin_placed = add_coin_to_column(&mut game, 5),
                Key::D7 => coin_placed = add_coin_to_column(&mut game, 6),
                Key::Space => {reset(&mut game); coin_placed=false},
                _       => coin_placed = false
            }

            if coin_placed {
                game.player_won = game_finished(&mut game);

                // AI turn
                if game.player_won == 0 {
                    game.player_turn = 2;

                    let ai_choice: i32 = get_ai_choice(&mut game);
                    if ai_choice >= 0 {
                        add_coin_to_column(&mut game, ai_choice as usize);
                    }

                    game.player_won = game_finished(&mut game);
                }
            }
        }

        // render graphics
        if let Some(args) = e.render_args() {
            app.render(&args, &game);
        }
    }
}

fn reset(game: &mut GameStruct) {
    game.player_turn = 0;
    game.player_won = 0;
    
    for col in 0..MAXCOLUMNS {
        for row in 0..MAXROWS {
            game.board[col][row] = 0;
        }
    }
}

/// .
// Harshini
// check if column is full.
// if full then return false (user needs to pick again)
// else populate the coin on the board, using gravity.
fn add_coin_to_column(game: &mut GameStruct, col: usize) -> bool {

    // check if game is finished
    if game.player_won != 0 {
        return false;
    }

    // check the col is not max and the column is not full
    if col > MAXCOLUMNS-1 || game.board[col][MAXROWS-1] != 0 {
        return false;
    }

    for n in 0..MAXROWS {
        if game.board[col][n] == 0 { //if there is empty spot then make sure the coin takes the bottom empty spot
            game.board[col][n] = game.player_turn;
            break;
        }
    }
    return true;
}

/*
    game_finished determines whether the game is done and returns one of four values:
    0 - game is not finished
    1 - player won
    2 - AI won
    3 - stalemate
*/
fn game_finished(game: &mut GameStruct) -> i32 {
    // check for player 1 win
    if game_won(game, 1) {
        return 1;
    }
    // check for player 2 win
    if game_won(game, 2) {
        return 2;
    }
    // check for open columns
    for col in 0..7 {
        if game.board[col][5] == 0 {
            return 0
        }
    }

    return 3;
}

/*
get_ai_choice randomly determines which column the ai chooses. If the column is already filled it will retry until an
an available column is found. Columns returned will be 0 - 6. If all columns are filled it will return -1
 */
fn get_ai_choice(game: &mut GameStruct) -> i32 {
    let mut tried_columns: [bool; 7] = [false, false, false, false, false, false, false];
    let ai_selection: i32 = loop {
        let col_index = rand::thread_rng().gen_range(0..7);
        if tried_columns[col_index] == false {
            if is_column_empty(game, col_index as i32) {
                break col_index as i32;
            }
            tried_columns[col_index] = true;
        }
        if ! tried_columns.contains(&false) {
            // can't move, return -1
            break -1;
        }
    };
    return ai_selection;
}

fn is_column_empty(game: &mut GameStruct, column_number: i32) -> bool {
    if game.board[column_number as usize][5] == 0 {
        return true;
    }
    return false;
}

fn game_won(game: &mut GameStruct, player: i32) -> bool {
    // Vertical check
    for col in 0..7 {
        for row in 0..3 {
            if game.board[col][row] == player &&
                game.board[col][row+1] == player &&
                game.board[col][row+2] == player &&
                game.board[col][row+3] == player {
                    return true;
                }
        }
    }
    // Horizontal check
    for row in 0..6 {
        for col in 0..4 {
            if game.board[col][row] == player &&
                game.board[col+1][row] == player &&
                game.board[col+2][row] == player &&
                game.board[col+3][row] == player {
                    return true;
                }
        }
    }

    // Ascending diagonal check
    for col in 3..7 {
        for row in 0..3 {
            if game.board[col][row] == player &&
                game.board[col-1][row+1] == player &&
                game.board[col-2][row+2] == player &&
                game.board[col-3][row+3] == player {
                    return true;
                }
        }
    }

    // Decending diagonal check
    for col in 3..7 {
        for row in 3..6 {
            if game.board[col][row] == player &&
                game.board[col-1][row-1] == player &&
                game.board[col-2][row-2] == player &&
                game.board[col-3][row-3] == player {
                    return true;
                }
        }
    }
    // Player did not win
    return false;
}
