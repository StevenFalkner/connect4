extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate device_query;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

const MAXCOLUMNS: usize = 7;
const MAXROWS: usize = 6;

// Render constants
const BLOCK_SIZE: f64 = 50.0;
const BLOCK_SPACING: f64 = 10.0;
const POSITION_LEFT: f64 = 150.0;
const POSITION_BOTTOM: f64 = 400.0;

pub struct GameStruct {
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
        let window_size_x = args.window_size[0];
        let window_size_y = args.window_size[1];

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

    let mut game = GameStruct {
        board: [[0;MAXROWS];MAXCOLUMNS],
        player_turn: 1
    };


    game.board[0][1] = 1;
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
    while let Some(e) = events.next(&mut window) {
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
        if col > MAXCOLUMNS-1 || game.board[col][MAXCOLUMNS-1] != 0 {
            return false;
        }

        for n in 0..MAXROWS-1 {
            if game.board[col][n] != 0 { //if there is empty spot then make sure the coin takes the bottom empty spot
                game.board[col][n] = game.player_turn;
            }
        }
        return true;
    }
}
