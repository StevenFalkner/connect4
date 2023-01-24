extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate rand;

use glutin_window::GlutinWindow as Window;
use graphics::ellipse;
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
    player_turn: i32 // value of 1 or 2, based on the player
}

pub struct App {
    gl: GlGraphics     // OpenGL drawing backend
}

impl App {
    const DARKGREEN: [f32; 4] = [0.0, 0.2, 0.0, 1.0];
    const LIGHTGREEN: [f32; 4] = [0.0, 0.3, 0.0, 1.0];
    const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
    const RED: [f32; 4] = [0.8, 0.0, 0.0, 1.0];

    fn render(&mut self, args: &RenderArgs, game: &GameStruct) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 50.0);

        self.gl.draw(args.viewport(), |c, gl| {
            // clear the screen
            clear(Self::DARKGREEN, gl);

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

    // fn update(&mut self, args: &UpdateArgs) {
    //     //user input??
    // }
}


fn main() {

    // Instantiate game data
    let mut game = GameStruct {
        board: [[0;MAXROWS];MAXCOLUMNS],
        player_turn: 1
    };

    // hard coded samples to demo the UI
    game.board[0][0] = 1;
    game.board[1][2] = 2;
    game.board[MAXCOLUMNS-1][MAXROWS-1] = 2;


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
    let mut success: bool = false;

// TODO remove call of get_ai_choice here
    // let ai_choice: i32 = get_ai_choice(&mut game);
    // println!("ai_choice = {}", ai_choice);


    while let Some(e) = events.next(&mut window) {

        // user input
        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::D1 => success = add_coin_to_column(&mut game, 0),
                Key::D2 => success = add_coin_to_column(&mut game, 1),
                Key::D3 => success = add_coin_to_column(&mut game, 2),
                Key::D4 => success = add_coin_to_column(&mut game, 3),
                Key::D5 => success = add_coin_to_column(&mut game, 4),
                Key::D6 => success = add_coin_to_column(&mut game, 5),
                Key::D7 => success = add_coin_to_column(&mut game, 6),
                _ => {}
            }

            // flip the player to the AI player
            if success {
                let ai_choice: i32 = get_ai_choice(&mut game);
                if ai_choice >= 0 {
                    success = add_coin_to_column(&mut game, ai_choice as usize);
                } else {
                    // TODO failure if could not place coin
                }
                if success {

                }
            }
        }

        // render graphics
        if let Some(args) = e.render_args() {
            app.render(&args, &game);
        }

        // if let Some(args) = e.update_args() {
        //     app.update(&args);
        // }
    }
}

fn add_coin_to_column(game: &mut GameStruct, col: usize) -> bool {

    // Harshini
    // check if column is full.
    // if full then return false (user needs to pick again)
    // else populate the coin on the board, using gravity.

    // sample code:
    game.board[col][5] = 1;
    // comment this out
    return false;
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
    if game.board[1][column_number as usize] == 0 {
        return true;
    }
    return false;
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
        if game.board[col][1] == 0 {
            return 0
        }
    }

    return 3;
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
    for row in 0..7 {
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