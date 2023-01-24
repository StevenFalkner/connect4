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
use device_query::{DeviceEvents, DeviceState};

const MAXCOLUMNS: usize = 7;
const MAXROWS: usize = 6;

pub struct GameStruct {
    board: [[i32;MAXROWS];MAXCOLUMNS],
    playerTurn: i32 // value of 1 or 2, based on the player
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
        let (x,y) = (window_size_x / 4.0, window_size_y / 1.25);

        self.gl.draw(args.viewport(), |c, gl| {
            // clear the screen
            clear(Self::DARKGREEN, gl);

            // draw the game board
            for _row in 0..MAXROWS {
                for _col in 0..MAXCOLUMNS {

                    let transform1 = c
                        .transform
                        .trans(x, y)
                        .trans(60.0 * (_col as f64) - 50.0, -60.0 * (_row as f64) - 50.0);

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
        playerTurn: 1
    };

    let device_state = DeviceState::new();

    let _guard = device_state.on_mouse_move(|position| {
        println!("Mouse position: {:#?}", position);
    });
    let _guard = device_state.on_mouse_down(|button| {
        println!("Mouse button down: {:#?}", button);
    });
    let _guard = device_state.on_mouse_up(|button| {
        println!("Mouse button up: {:#?}", button);
    });
    let _guard = device_state.on_key_down(|key| {
        println!("Keyboard key down: {:#?}", key);
    });
    let _guard = device_state.on_key_up(|key| {
        println!("Keyboard key up: {:#?}", key);
    });


    game.board[0][1] = 1;
    game.board[1][2] = 2;
    game.board[MAXCOLUMNS-1][MAXROWS-1] = 2;


    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.
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

}
