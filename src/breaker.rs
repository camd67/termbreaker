use std::io::Write;

use crossterm::event::KeyCode;
use crossterm::style;

use crate::drawer::Drawer;
use crate::phys::{Direction, PhysicsEntity, PlayerEntity, Size, Square, StaticEntity, Vec2d};

/// Stores the current state of the breaker game
pub struct Game {
    ball: PhysicsEntity,
    paddle: PlayerEntity,
    blocks: Vec<StaticEntity>,

    /// Screen size in 0,0 based coordinates
    screen_size: Size,
}

const MOVE_RIGHT: KeyCode = KeyCode::Right;
const MOVE_LEFT: KeyCode = KeyCode::Left;
const QUIT_KEY: KeyCode = KeyCode::Char('q');
const BLOCK_HEIGHT: u16 = 2;
const BLOCK_WIDTH: u16 = 4;
const BLOCK_COLORS: [style::Color; 4] = [
    style::Color::Magenta,
    style::Color::Grey,
    style::Color::Yellow,
    style::Color::Green,
];

fn generate_blocks(screen_size: Size) -> Vec<StaticEntity> {
    let mut blocks = Vec::new();
    const BLOCK_ROW_START: u16 = 2;
    let block_row_end: u16 = screen_size.h / 2;
    // Generate an alternating series of blocks, each equal sized
    // ---- ---- ---- ----
    //   ---- ---- ----
    // ---- ---- ---- ----
    // Up to half the screen height
    let mut row = BLOCK_ROW_START;
    let mut color_index = 0;
    while row < block_row_end {
        // Get our offset, so our rows look a little more staggered
        let col_offset: u16 = if row % BLOCK_HEIGHT * 2 == 0 {
            0
        } else {
            BLOCK_WIDTH / 2
        };
        let mut col = col_offset;
        while col < screen_size.w - BLOCK_WIDTH {
            // place a block in this column
            let new_block = StaticEntity {
                collider: Square {
                    x: col,
                    y: row,
                    w: BLOCK_WIDTH,
                    h: BLOCK_HEIGHT,
                },
                alive: true,
                color: BLOCK_COLORS[color_index],
            };
            blocks.push(new_block);
            col += BLOCK_WIDTH + 1;
            color_index = if color_index % 2 == 0 {
                color_index + 1
            } else {
                color_index - 1
            };
        }
        color_index = if color_index < 2 { 2 } else { 0 };
        row += BLOCK_HEIGHT + 1;
    }
    return blocks;
}

impl Game {
    pub fn new(screen_size: (u16, u16)) -> Game {
        let screen_size = Size {
            w: screen_size.0 - 1,
            h: screen_size.1 - 1,
        };
        return Game {
            ball: PhysicsEntity {
                collider: Square {
                    x: screen_size.w / 2,
                    y: screen_size.h / 2 + 2,
                    w: 1,
                    h: 1,
                },
                speed: Vec2d { x: 1, y: 1 },
                alive: true,
            },
            paddle: PlayerEntity {
                collider: Square {
                    x: screen_size.w / 2 - 3,
                    y: screen_size.h - 3,
                    w: 6,
                    h: 1,
                },
                speed: Vec2d { x: 0, y: 0 },
                alive: true,
            },
            blocks: generate_blocks(screen_size),
            screen_size,
        };
    }

    /// Updates the current game state
    /// Returns true if the game should continue, false if not
    pub fn update(&mut self, pressed_key: Option<KeyCode>) -> bool {
        // Move our paddle
        match pressed_key {
            Some(key) => match key {
                QUIT_KEY => {
                    // Just bail out immediately if we're quitting
                    return false;
                }
                MOVE_LEFT => {
                    self.paddle.speed.x = -1;
                }
                MOVE_RIGHT => {
                    self.paddle.speed.x = 1;
                }
                _ => {
                    self.paddle.speed.x = 0;
                }
            },
            None => {
                self.paddle.speed.x = 0;
            }
        }
        // Move our paddle
        self.paddle.apply_speed(self.screen_size);
        // Move our ball
        self.ball.apply_speed(self.screen_size);
        // Bounce our ball off the outer walls
        if self.ball.collider.x + self.ball.collider.w == self.screen_size.w
            || self.ball.collider.x == 0
        {
            self.ball.speed.x *= -1
        }
        if self.ball.collider.y + self.ball.collider.h == self.screen_size.h
            || self.ball.collider.y == 0
        {
            self.ball.speed.y *= -1;
        }

        // Check for collisions against our paddle
        if self.ball.collider.collides_with(&self.paddle.collider) {
            self.ball.speed.y *= -1;
            // horizontal velocity is adjusted based on collision location
            // more than half determines the direction
            if self.ball.collider.x > self.paddle.collider.x + self.paddle.collider.w / 2 {
                self.ball.speed.x = -1;
            } else {
                self.ball.speed.x = 1;
            }
        }
        // Check for collisions against our blocks, and then destroy them
        for block in &mut self.blocks {
            if !block.alive {
                continue;
            }
            if block.collider.collides_with(&self.ball.collider) {
                block.alive = false;
                // Then bounce the ball
                match block.collider.get_collision_direction(&self.ball.collider) {
                    Direction::UP => {
                        self.ball.speed.y *= -1;
                    }
                    Direction::RIGHT => {
                        self.ball.speed.x *= -1;
                    }
                    Direction::DOWN => {
                        self.ball.speed.y *= -1;
                    }
                    Direction::LEFT => {
                        self.ball.speed.x *= -1;
                    }
                }
            }
        }

        return true;
    }

    pub fn draw<W>(&self, draw: &mut Drawer<W>)
    where
        W: Write,
    {
        for block in &self.blocks {
            if !block.alive {
                continue;
            }
            draw.draw_square(block.collider, block.color);
        }
        draw.draw_square(self.paddle.collider, style::Color::Cyan);
        draw.draw_square(self.ball.collider, style::Color::Red);
    }
}
