extern crate find_folder;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;

use opengl_graphics::OpenGL;
use piston::window::WindowSettings;
use piston::{Button, Event, Key, PressEvent};
use piston_window::{Glyphs, PistonWindow as Window};

pub mod levels;

pub struct App {
    window: Window,
    board: Vec<Vec<Hole>>,
    grabbing: bool,
    game_over: bool,
    won: bool,
    selected: (u8, u8),
    glyphs: Glyphs,
    current_level: usize,
    levels: Vec<Box<dyn levels::Level>>,
}

const GREEN: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
const YELLOW: [f32; 4] = [0.4, 0.4, 0.4, 1.0];
const RED: [f32; 4] = [0.3, 0.3, 0.3, 1.0];
const NOTIF: [f32; 4] = [0.2, 0.2, 0.2, 0.7];

const SQUARE_SIZE: f64 = 50.0;
const GUTTER_SIZE: f64 = 4.0;
const BORDER_SIZE: f64 = 8.0;

impl App {
    fn render(&mut self, e: &Event) {
        use graphics::*;

        let square = rectangle::square(0.0, 0.0, 50.0);
        let selected_square = rectangle::square(0.0, 0.0, 42.0);
        let selected_circle = ellipse::circle(0.0, 0.0, 20.0);
        let circle = ellipse::circle(0.0, 0.0, 16.0);

        let board = &self.board;
        let selected = self.selected;
        let grabbing = self.grabbing;
        let game_over = self.game_over;
        let won = self.won;

        let glyphs = &mut self.glyphs;

        self.window.draw_2d(e, |c, gl, device| {
            // Clear the screen.
            clear(GREEN, gl);

            for col in board {
                for hole in col {
                    if !hole.is_hole {
                        continue;
                    }

                    let transform = c.transform.trans(
                        BORDER_SIZE + hole.x as f64 * (SQUARE_SIZE + GUTTER_SIZE),
                        BORDER_SIZE + hole.y as f64 * (SQUARE_SIZE + GUTTER_SIZE),
                    );

                    rectangle(RED, square, transform, gl);

                    let transform2 = transform.trans(25.0, 25.0);

                    if hole.x == selected.0 && hole.y == selected.1 {
                        if grabbing {
                            rectangle(YELLOW, square, transform, gl);
                        } else {
                            let transform1 = transform.trans(4.0, 4.0);
                            rectangle(YELLOW, selected_square, transform1, gl);
                            ellipse(RED, selected_circle, transform2, gl)
                        }
                    }

                    if hole.filled {
                        ellipse(GREEN, circle, transform2, gl)
                    }
                }
            }

            if game_over {
                let transform = c.transform.trans(0.0, 0.0);

                let size = GUTTER_SIZE * 6.0 + SQUARE_SIZE * 7.0 + BORDER_SIZE * 2.0;

                let notif = rectangle::rectangle_by_corners(
                    (size / 2.0) - 180.0,
                    (size / 2.0) - 140.0,
                    (size / 2.0) + 180.0,
                    (size / 2.0) + 140.0,
                );
                rectangle(NOTIF, notif, transform, gl);

                let transform1 = c.transform.trans((size / 2.0) - 84.0, (size / 2.0) - 12.0);

                let mut text = "Game Over!";

                if won {
                    text = "You Won!"
                }

                text::Text::new_color([0.9, 0.9, 0.9, 1.0], 24)
                    .draw(text, glyphs, &c.draw_state, transform1, gl)
                    .unwrap();

                let transform2 = c.transform.trans((size / 2.0) - 84.0, (size / 2.0) + 12.0);

                text::Text::new_color([0.9, 0.9, 0.9, 1.0], 14)
                    .draw(
                        "Press R to restart level.",
                        glyphs,
                        &c.draw_state,
                        transform2,
                        gl,
                    )
                    .unwrap();

                if won {
                    let transform3 = c.transform.trans((size / 2.0) - 84.0, (size / 2.0) + 32.0);
                    text::Text::new_color([0.9, 0.9, 0.9, 1.0], 14)
                        .draw(
                            "Press SPACE to go to next level.",
                            glyphs,
                            &c.draw_state,
                            transform3,
                            gl,
                        )
                        .unwrap();
                }

                // Update glyphs before rendering.
                glyphs.factory.encoder.flush(device);
            }
        });
    }

    fn jump(&mut self, dicrection: (i8, i8)) {
        let (sxr, syr) = self.selected;
        let sx = sxr as i8;
        let sy = syr as i8;
        let (dx, dy) = dicrection;

        if self.is_valid_jump(dicrection) && self.grabbing {
            self.board[usize::from(sxr)][usize::from(syr)].filled = false;
            self.board[usize::from((sx + dx) as u8)][usize::from((sy + dy) as u8)].filled = true;
            self.board[usize::from((sx + 2 * dx) as u8)][usize::from((sy + 2 * dy) as u8)].filled =
                true;
            self.selected = ((sx + 2 * dx) as u8, (sy + 2 * dy) as u8);
        } else {
            if self.board[(sx + dx) as usize][(sy + dy) as usize].is_hole {
                self.selected = ((sx + dx) as u8, (sy + dy) as u8);
            }
        }

        if self.is_game_over() {
            self.game_over = true;

            if self.did_win() {
                self.won = true;
            }
        }
    }

    fn is_valid_jump(&self, dicrection: (i8, i8)) -> bool {
        let (sx, sy) = self.selected;
        return self.is_valid_jump_from((sx, sy), dicrection);
    }

    fn is_valid_jump_from(&self, hole: (u8, u8), dicrection: (i8, i8)) -> bool {
        let (sxr, syr) = hole;
        let sx = sxr as i8;
        let sy = syr as i8;
        let (dx, dy) = dicrection;
        let mut can_jump = false;

        if sy + 2 * dy > -1 && sy + 2 * dy < 7 && sx + 2 * dx > -1 && sx + 2 * dx < 7 {
            let hole = &self.board[usize::from(sxr)][usize::from(syr)];
            if hole.filled {
                let hole_next =
                    &self.board[usize::from((sx + dx) as u8)][usize::from((sy + dy) as u8)];
                if !hole_next.filled && hole_next.is_hole {
                    let hole_next_next = &self.board[usize::from((sx + 2 * dx) as u8)]
                        [usize::from((sy + 2 * dy) as u8)];
                    {
                        if !hole_next_next.filled && hole_next_next.is_hole {
                            can_jump = true;
                        }
                    }
                }
            }
        }

        return can_jump;
    }

    fn is_game_over(&self) -> bool {
        for x in 0..7 {
            for y in 0..7 {
                let hole = &self.board[x][y];
                if hole.is_hole && hole.filled {
                    let has_move = self.is_valid_jump_from((hole.x, hole.y), (0, -1))
                        || self.is_valid_jump_from((hole.x, hole.y), (0, 1))
                        || self.is_valid_jump_from((hole.x, hole.y), (-1, 0))
                        || self.is_valid_jump_from((hole.x, hole.y), (1, 0));

                    if has_move {
                        return false;
                    }
                }
            }
        }

        return true;
    }

    fn go_up(&mut self) {
        let (_sx, sy) = self.selected;
        if sy == 0 {
            return;
        }

        self.jump((0, -1));
        self.grabbing = false;
    }

    fn go_down(&mut self) {
        let (_sx, sy) = self.selected;
        if sy == 6 {
            return;
        }

        self.jump((0, 1));
        self.grabbing = false;
    }

    fn go_left(&mut self) {
        let (sx, _sy) = self.selected;
        if sx == 0 {
            return;
        }

        self.jump((-1, 0));
        self.grabbing = false;
    }

    fn go_right(&mut self) {
        let (sx, _sy) = self.selected;
        if sx == 6 {
            return;
        }

        self.jump((1, 0));
        self.grabbing = false;
    }

    fn grab(&mut self) {
        self.grabbing = !self.grabbing
    }

    fn did_win(&self) -> bool {
        let current_level = &self.levels[self.current_level];
        let max = current_level.get_max_pegs();

        let mut total_pegs = 0;
        for x in 0..7 {
            for y in 0..7 {
                if self.board[x][y].filled {
                    total_pegs += 1;
                }
            }
        }

        return total_pegs >= max;
    }

    fn restart(&mut self) {
        self.game_over = false;
        self.won = false;

        let current_level = &self.levels[self.current_level];
        current_level.init_board(&mut self.board);
        self.selected = current_level.get_selected();
    }

    fn start_next_level(&mut self) {
        if self.current_level + 1 < self.levels.len() {
            self.current_level += 1
        }

        self.game_over = false;
        self.won = false;

        let next_level = &self.levels[self.current_level];
        next_level.init_board(&mut self.board);
        self.selected = next_level.get_selected();
    }

    // fn update(&mut self, args: &UpdateArgs) {
    //     // Rotate 2 radians per second.
    //     //self.rotation += 2.0 * args.dt;
    // }

    fn handle_key(&mut self, key: Key) {
        match key {
            Key::Up => self.go_up(),
            Key::Down => self.go_down(),
            Key::Left => self.go_left(),
            Key::Right => self.go_right(),
            Key::Space => {
                if self.won && self.game_over {
                    self.start_next_level()
                } else {
                    self.grab()
                }
            }
            Key::R => self.restart(),
            Key::N => self.start_next_level(),
            _ => {}
        }
    }

    fn start(&mut self) {
        let first_level = &self.levels[self.current_level];
        first_level.init_board(&mut self.board);
        self.selected = first_level.get_selected();

        while let Some(e) = self.window.next() {
            self.render(&e);

            if let Some(Button::Keyboard(key)) = e.press_args() {
                self.handle_key(key)
            }

            // if let Some(args) = e.update_args() {
            //     app.update(&args);
            // }
        }
    }
}

pub struct Hole {
    is_hole: bool,
    filled: bool,
    x: u8,
    y: u8,
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    let size = GUTTER_SIZE * 6.0 + SQUARE_SIZE * 7.0 + BORDER_SIZE * 2.0;

    // Create a Glutin window.
    let mut window: Window = WindowSettings::new("grow your pegs", [size, size])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut board = Vec::new();
    for x in 0..7 {
        let col = Vec::new();
        board.push(col);
        for y in 0..7 {
            board[x].push(Hole {
                is_hole: false,
                filled: false,
                x: x as u8,
                y: y as u8,
            });
        }
    }

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets")
        .unwrap();
    println!("{:?}", assets);
    let glyphs: Glyphs = window
        .load_font(assets.join("SourceSansPro-Regular.ttf"))
        .unwrap();

    let levels: Vec<Box<dyn levels::Level>> = vec![
        Box::new(levels::intro::Intro {}),
        Box::new(levels::rectangle::Rect {}),
        Box::new(levels::home::Home {}),
        Box::new(levels::classic::Classic {}),
        Box::new(levels::euro::Euro {}),
    ];

    // Create a new game and run it.
    let mut app = App {
        window,
        board,
        grabbing: false,
        selected: (3, 3),
        game_over: false,
        won: false,
        glyphs: glyphs,
        current_level: 0,
        levels,
    };

    app.start();
}
