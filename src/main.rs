extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, PressEvent, Button, Key};
use piston::window::WindowSettings;

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

    // fn update(&mut self, args: &UpdateArgs) {
    //     //user input??
    // }
}


fn main() {

    // Instantiate game data
    let mut game = GameStruct {
        board: [[0;MAXROWS];MAXCOLUMNS],
        player_turn: 1,
        player_won: 0
    };


    //game.board[0][1] = 1;
    //game.board[1][2] = 2;
    //game.board[MAXCOLUMNS-1][MAXROWS-1] = 2;


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
    let mut coin_placed: bool = false;

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
                _ => {}
            }

            if coin_placed {
                game.player_won = game_finished(&game);

                // AI turn
                if game.player_won == 0 {
                    game.player_turn = 2;

                    // todo: call Jesse's AI code.

                    game.player_won = game_finished(&game);
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

    /// .
    // Harshini
    // check if column is full.
    // if full then return false (user needs to pick again)
    // else populate the coin on the board, using gravity.
    fn add_coin_to_column(game: &mut GameStruct, col: usize) -> bool {
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
}

fn game_finished(_game: &GameStruct) -> i32 {

    // Jesse

    return 0;
}
