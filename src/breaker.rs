use std::io::Write;

use crossterm::event::KeyCode;
use crossterm::style;

use crate::drawer::Drawer;
use crate::phys::{PhysicsEntity, PlayerEntity, Size, Square, StaticEntity, Vec2d};

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
const MOVE_UP: KeyCode = KeyCode::Up;
const MOVE_DOWN: KeyCode = KeyCode::Down;

impl Game {
    pub fn new(screen_size: (u16, u16)) -> Game {
        Game {
            ball: PhysicsEntity {
                collider: Square {
                    x: 0,
                    y: 0,
                    w: 1,
                    h: 1,
                },
                speed: Vec2d { x: 1, y: 1 },
                alive: true,
            },
            paddle: PlayerEntity {
                collider: Square {
                    x: 15,
                    y: 15,
                    w: 5,
                    h: 1,
                },
                speed: Vec2d { x: 0, y: 0 },
                alive: true,
            },
            blocks: vec![],
            screen_size: Size {
                w: screen_size.0 - 1,
                h: screen_size.1 - 1,
            },
        }
    }

    /// Updates the current game state
    /// Returns true if the game should continue, false if not
    pub fn update(&mut self, pressed_key: Option<KeyCode>) -> bool {
        // Move our paddle
        match pressed_key {
            Some(key) => match key {
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
        // Bounce our ball
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
        true
    }

    pub fn draw<W>(&self, draw: &mut Drawer<W>)
    where
        W: Write,
    {
        draw.draw_square(self.paddle.collider.clone(), style::Color::Cyan);
        draw.draw_square(self.ball.collider.clone(), style::Color::Red);
    }
}
