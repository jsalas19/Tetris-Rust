use macroquad::prelude::*;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 20;
const CELL_SIZE: f32 = 30.0;
const BOARD_X: f32 = 100.0;
const BOARD_Y: f32 = 50.0;
const BOARD_PIXEL_WIDTH: f32 = BOARD_WIDTH as f32 * CELL_SIZE;
const BOARD_PIXEL_HEIGHT: f32 = BOARD_HEIGHT as f32 * CELL_SIZE;

type Cell = (i32, i32);
type Shape = [Cell; 4];

const SHAPES: [Shape; 7] = [
    [(0, 1), (1, 1), (2, 1), (3, 1)], // I
    [(1, 0), (2, 0), (1, 1), (2, 1)], // O
    [(1, 0), (0, 1), (1, 1), (2, 1)], // T
    [(1, 0), (2, 0), (0, 1), (1, 1)], // S
    [(0, 0), (1, 0), (1, 1), (2, 1)], // Z
    [(0, 0), (0, 1), (1, 1), (2, 1)], // J
    [(2, 0), (0, 1), (1, 1), (2, 1)], // L
];

const COLORS: [Color; 7] = [
    Color::new(0.22, 0.85, 0.91, 1.0),
    Color::new(0.97, 0.83, 0.27, 1.0),
    Color::new(0.70, 0.40, 0.96, 1.0),
    Color::new(0.38, 0.84, 0.48, 1.0),
    Color::new(0.94, 0.34, 0.39, 1.0),
    Color::new(0.34, 0.53, 0.95, 1.0),
    Color::new(0.98, 0.58, 0.28, 1.0),
];

#[derive(Clone, Copy)]
struct Piece {
    kind: usize,
    x: i32,
    y: i32,
    rotation: u8,
}

struct Game {
    board: [[Option<usize>; BOARD_WIDTH]; BOARD_HEIGHT],
    current: Piece,
    next_kind: usize,
    bag: Vec<usize>,
    score: u32,
    lines: u32,
    level: u32,
    fall_timer: f32,
    paused: bool,
    game_over: bool,
}

impl Game {
    fn new() -> Self {
        let mut game = Self {
            board: [[None; BOARD_WIDTH]; BOARD_HEIGHT],
            current: Piece {
                kind: 0,
                x: 3,
                y: 0,
                rotation: 0,
            },
            next_kind: 0,
            bag: Vec::new(),
            score: 0,
            lines: 0,
            level: 1,
            fall_timer: 0.0,
            paused: false,
            game_over: false,
        };
        game.current.kind = game.take_next_kind();
        game.next_kind = game.take_next_kind();
        game
    }

    fn take_next_kind(&mut self) -> usize {
        if self.bag.is_empty() {
            self.bag = (0..SHAPES.len()).collect();
            for i in (1..self.bag.len()).rev() {
                let j = macroquad::rand::gen_range(0, (i + 1) as i32) as usize;
                self.bag.swap(i, j);
            }
        }
        self.bag.pop().expect("the piece bag is refilled when empty")
    }

    fn collides(&self, piece: Piece) -> bool {
        shape_for(piece).iter().any(|&(dx, dy)| {
            let x = piece.x + dx;
            let y = piece.y + dy;
            x < 0
                || x >= BOARD_WIDTH as i32
                || y >= BOARD_HEIGHT as i32
                || (y >= 0 && self.board[y as usize][x as usize].is_some())
        })
    }

    fn try_move(&mut self, dx: i32, dy: i32) -> bool {
        let moved = Piece {
            x: self.current.x + dx,
            y: self.current.y + dy,
            ..self.current
        };
        if self.collides(moved) {
            false
        } else {
            self.current = moved;
            true
        }
    }

    fn rotate(&mut self, clockwise: bool) {
        if self.current.kind == 1 {
            return;
        }
        let mut candidate = self.current;
        candidate.rotation = if clockwise {
            (candidate.rotation + 1) % 4
        } else {
            (candidate.rotation + 3) % 4
        };
        if !self.collides(candidate) {
            self.current = candidate;
        }
    }

    fn lock_piece(&mut self) {
        let cells = shape_for(self.current);
        for &(dx, dy) in &cells {
            let x = self.current.x + dx;
            let y = self.current.y + dy;
            if y < 0 {
                self.game_over = true;
                return;
            }
            self.board[y as usize][x as usize] = Some(self.current.kind);
        }
        self.clear_lines();
        self.spawn_piece();
    }

    fn clear_lines(&mut self) {
        let mut cleared = 0;
        let mut y = BOARD_HEIGHT as i32 - 1;
        while y >= 0 {
            if self.board[y as usize].iter().all(Option::is_some) {
                for row in (1..=y as usize).rev() {
                    self.board[row] = self.board[row - 1];
                }
                self.board[0] = [None; BOARD_WIDTH];
                cleared += 1;
            } else {
                y -= 1;
            }
        }
        if cleared > 0 {
            let points = [0, 100, 300, 500, 800][cleared];
            self.score += points * self.level;
            self.lines += cleared as u32;
            self.level = self.lines / 10 + 1;
        }
    }

    fn spawn_piece(&mut self) {
        self.current = Piece {
            kind: self.next_kind,
            x: 3,
            y: 0,
            rotation: 0,
        };
        self.next_kind = self.take_next_kind();
        self.fall_timer = 0.0;
        if self.collides(self.current) {
            self.game_over = true;
        }
    }

    fn update(&mut self, dt: f32) {
        if is_key_pressed(KeyCode::R) {
            *self = Self::new();
            return;
        }
        if is_key_pressed(KeyCode::P) && !self.game_over {
            self.paused = !self.paused;
        }
        if self.paused || self.game_over {
            return;
        }

        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
            self.try_move(-1, 0);
        }
        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
            self.try_move(1, 0);
        }
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::X) {
            self.rotate(true);
        }
        if is_key_pressed(KeyCode::Z) {
            self.rotate(false);
        }
        if is_key_pressed(KeyCode::Space) {
            while self.try_move(0, 1) {}
            self.lock_piece();
            return;
        }

        let soft_drop = is_key_down(KeyCode::Down) || is_key_down(KeyCode::S);
        let interval = if soft_drop {
            0.045
        } else {
            (0.65 * 0.82_f32.powi(self.level as i32 - 1)).max(0.08)
        };
        self.fall_timer += dt;
        if self.fall_timer >= interval {
            self.fall_timer = 0.0;
            if !self.try_move(0, 1) {
                self.lock_piece();
            } else if soft_drop {
                self.score += 1;
            }
        }
    }

    fn draw(&self) {
        clear_background(Color::new(0.035, 0.047, 0.085, 1.0));

        draw_text("TETRIS", BOARD_X, 34.0, 26.0, Color::new(0.35, 0.85, 0.91, 1.0));
        draw_rectangle(
            BOARD_X - 2.0,
            BOARD_Y - 2.0,
            BOARD_PIXEL_WIDTH + 4.0,
            BOARD_PIXEL_HEIGHT + 4.0,
            Color::new(0.12, 0.16, 0.24, 1.0),
        );
        draw_rectangle(
            BOARD_X,
            BOARD_Y,
            BOARD_PIXEL_WIDTH,
            BOARD_PIXEL_HEIGHT,
            Color::new(0.055, 0.07, 0.12, 1.0),
        );

        for row in 0..BOARD_HEIGHT {
            for col in 0..BOARD_WIDTH {
                if let Some(kind) = self.board[row][col] {
                    draw_cell(col as i32, row as i32, COLORS[kind], 1.0);
                } else {
                    draw_rectangle_lines(
                        BOARD_X + col as f32 * CELL_SIZE,
                        BOARD_Y + row as f32 * CELL_SIZE,
                        CELL_SIZE,
                        CELL_SIZE,
                        1.0,
                        Color::new(0.12, 0.15, 0.22, 1.0),
                    );
                }
            }
        }

        if !self.game_over {
            let shape = shape_for(self.current);
            let mut ghost = self.current;
            while !self.collides(Piece { y: ghost.y + 1, ..ghost }) {
                ghost.y += 1;
            }
            for &(dx, dy) in &shape {
                draw_cell(
                    ghost.x + dx,
                    ghost.y + dy,
                    Color::new(
                        COLORS[self.current.kind].r,
                        COLORS[self.current.kind].g,
                        COLORS[self.current.kind].b,
                        1.0,
                    ),
                    0.35,
                );
            }
            for &(dx, dy) in &shape {
                draw_cell(
                    self.current.x + dx,
                    self.current.y + dy,
                    COLORS[self.current.kind],
                    1.0,
                );
            }
        }

        draw_panel(self);
        if self.paused || self.game_over {
            let title = if self.game_over { "GAME OVER" } else { "PAUSED" };
            let width = measure_text(title, None, 36, 1.0).width;
            draw_rectangle(
                BOARD_X + 25.0,
                BOARD_Y + BOARD_PIXEL_HEIGHT / 2.0 - 55.0,
                BOARD_PIXEL_WIDTH - 50.0,
                110.0,
                Color::new(0.025, 0.035, 0.065, 0.92),
            );
            draw_text(
                title,
                BOARD_X + (BOARD_PIXEL_WIDTH - width) / 2.0,
                BOARD_Y + BOARD_PIXEL_HEIGHT / 2.0 - 3.0,
                36.0,
                WHITE,
            );
            let prompt = if self.game_over {
                "Press R to play again"
            } else {
                "Press P to resume"
            };
            let prompt_width = measure_text(prompt, None, 16, 1.0).width;
            draw_text(
                prompt,
                BOARD_X + (BOARD_PIXEL_WIDTH - prompt_width) / 2.0,
                BOARD_Y + BOARD_PIXEL_HEIGHT / 2.0 + 28.0,
                16.0,
                LIGHTGRAY,
            );
        }
    }

}

fn shape_for(piece: Piece) -> Shape {
    let mut shape = SHAPES[piece.kind];
    for _ in 0..piece.rotation {
        shape = shape.map(|(x, y)| (3 - y, x));
    }
    shape
}

fn draw_cell(x: i32, y: i32, color: Color, alpha: f32) {
    if y < 0 {
        return;
    }
    let inset = 2.0;
    let mut color = color;
    color.a *= alpha;
    draw_rectangle(
        BOARD_X + x as f32 * CELL_SIZE + inset,
        BOARD_Y + y as f32 * CELL_SIZE + inset,
        CELL_SIZE - inset * 2.0,
        CELL_SIZE - inset * 2.0,
        color,
    );
    if alpha > 0.9 {
        draw_rectangle(
            BOARD_X + x as f32 * CELL_SIZE + inset,
            BOARD_Y + y as f32 * CELL_SIZE + inset,
            CELL_SIZE - inset * 2.0,
            3.0,
            Color::new(1.0, 1.0, 1.0, 0.16),
        );
    }
}

fn draw_panel(game: &Game) {
    let x = BOARD_X + BOARD_PIXEL_WIDTH + 34.0;
    draw_text("NEXT", x, 76.0, 19.0, LIGHTGRAY);
    draw_rectangle(x - 8.0, 88.0, 150.0, 115.0, Color::new(0.08, 0.10, 0.16, 1.0));
    let preview_cell = 23.0;
    for &(dx, dy) in &SHAPES[game.next_kind] {
        let preview_x = x + 24.0 + dx as f32 * preview_cell;
        let preview_y = 103.0 + dy as f32 * preview_cell;
        draw_rectangle(
            preview_x + 1.0,
            preview_y + 1.0,
            preview_cell - 2.0,
            preview_cell - 2.0,
            COLORS[game.next_kind],
        );
    }

    let stats_y = 244.0;
    draw_stat(x, stats_y, "SCORE", &game.score.to_string());
    draw_stat(x, stats_y + 68.0, "LINES", &game.lines.to_string());
    draw_stat(x, stats_y + 136.0, "LEVEL", &game.level.to_string());

    draw_text("CONTROLS", x, 497.0, 17.0, LIGHTGRAY);
    draw_text("Move     Left/Right  A/D", x, 525.0, 14.0, GRAY);
    draw_text("Rotate   Up/X  or  Z", x, 549.0, 14.0, GRAY);
    draw_text("Soft drop Down/S", x, 573.0, 14.0, GRAY);
    draw_text("Hard drop Space", x, 597.0, 14.0, GRAY);
    draw_text("Pause    P    Restart R", x, 621.0, 14.0, GRAY);
}

fn draw_stat(x: f32, y: f32, label: &str, value: &str) {
    draw_text(label, x, y, 15.0, GRAY);
    draw_text(value, x, y + 31.0, 25.0, WHITE);
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Tetris - Rust".to_owned(),
        window_width: 620,
        window_height: 700,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new();
    loop {
        game.update(get_frame_time());
        game.draw();
        next_frame().await;
    }
}
