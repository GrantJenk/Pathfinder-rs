use crate::grid::{Location, Node, Grid};

use piston::{MouseButton, MouseCursorEvent, window::WindowSettings};
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, PressEvent, RenderArgs, RenderEvent};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
const LIGHT: [f32; 4] = [0.79, 0.79, 1.0, 1.0];
const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const CYAN: [f32; 4] = [0.0, 0.79, 1.0, 1.0];

pub struct App {
    window: GlutinWindow,
    gl: GlGraphics,
    cursor: [f64; 2],
    grid: Grid,
    sq_dim: f64,
    start_loc: Option<Location>,
    dest_loc: Option<Location>,
}

impl App {
    pub fn new() -> App {
        let opengl = OpenGL::V3_2;
        let window_dim: u32 = 800;
        let grid_dim = 50;
        let sq_dim = (window_dim as f64)/grid_dim as f64;
        let window: GlutinWindow = WindowSettings::new("A* Pathfinder", [window_dim, window_dim])
            .graphics_api(opengl)
            .exit_on_esc(true)
            .build()
            .unwrap();
        let gl = GlGraphics::new(opengl);
        let mut grid = Grid::new(grid_dim, grid_dim);
        grid.randomize_walls(20);

        App {window, gl, cursor: [-1.0, -1.0], grid, sq_dim, start_loc: None, dest_loc: None}
    }

    pub fn run(&mut self) {
        let mut events = Events::new(EventSettings::new());
        while let Some(e) = events.next(&mut self.window) {
            e.mouse_cursor(|pos| {
                self.cursor = pos;
            });

            if let Some(args) = e.render_args() {
                self.render(&args);
            }

            if let Some(Button::Mouse(button)) = e.press_args() {
                if button == MouseButton::Left {
                    self.handle_click();
                } else {
                    self.grid.reset();
                    self.start_loc = None;
                    self.dest_loc = None;
                }
            }
        }
    }

    fn handle_click(&mut self) {
        let x = self.cursor[0]/self.sq_dim;
        let y = self.cursor[1]/self.sq_dim;
        let clicked_node = self.grid.get_node(Location(x as i32, y as i32));
        if clicked_node.is_wall {
            return;
        }

        if self.start_loc == None {
            self.start_loc = Some(clicked_node.loc);
        } else if self.dest_loc == None {
            self.dest_loc = Some(clicked_node.loc);
            self.grid.a_star(self.start_loc.unwrap(), self.dest_loc.unwrap());
        } else if self.start_loc != None && self.dest_loc != None {
            self.grid.reset();
            self.start_loc = None;
            self.dest_loc = None;
        }
    }

    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        let get_square_color = |node: &Node| -> [f32; 4] {
            if node.is_path {
                CYAN
            } else if node.is_wall {
                BLACK
            } else if node.visited {
                LIGHT
            } else {
                WHITE
            }
        };

        let sq_dim = self.sq_dim;
        let square = rectangle::square(0.0, 0.0, sq_dim - 2 as f64);
        let grid = &self.grid;
        let start_loc = self.start_loc;
        let dest_loc = self.dest_loc;

        self.gl.draw(args.viewport(), |c, gl| {
            clear(WHITE, gl);

            for node in grid.get_nodes() {
                let color = get_square_color(node);
                let transform = c
                    .transform
                    .trans(node.loc.0 as f64 * sq_dim, node.loc.1 as f64 * sq_dim);

                rectangle(color, square, transform, gl);
            }

            if let Some(loc) = start_loc {
                let (x, y) = ( (loc.0 as f64) * sq_dim, (loc.1 as f64) * sq_dim);
                let transform = c
                    .transform
                    .trans(x as f64, y as f64);
                rectangle(GREEN, square, transform, gl);
            }
            if let Some(loc) = dest_loc {
                let (x, y) = ( (loc.0 as f64) * sq_dim, (loc.1 as f64) * sq_dim);
                let transform = c
                    .transform
                    .trans(x as f64, y as f64);
                rectangle(RED, square, transform, gl);
            }
        });
    }
}