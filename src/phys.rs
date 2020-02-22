use std::ops::Add;

/// An entity that can be moved by the player
pub struct PlayerEntity {
    pub collider: Square,
    pub speed: Vec2d,
    pub alive: bool,
}
impl PlayerEntity {
    pub fn apply_speed(&mut self, bounds: Size) {
        self.collider.move_x(self.speed.x);
        self.collider.move_y(self.speed.y);
        self.collider.keep_inside(bounds);
    }
}

/// An entity that won't move ever
pub struct StaticEntity {
    pub collider: Square,
    pub alive: bool,
}

/// An entity that moves, but on it's own accord
pub struct PhysicsEntity {
    pub collider: Square,
    pub speed: Vec2d,
    pub alive: bool,
}

impl PhysicsEntity {
    pub fn apply_speed(&mut self, bounds: Size) {
        self.collider.move_x(self.speed.x);
        self.collider.move_y(self.speed.y);
        self.collider.keep_inside(bounds);
    }
}

#[derive(Clone, Copy)]
/// A vector in 2D space
pub struct Vec2d {
    pub x: i16,
    pub y: i16,
}

#[derive(Copy, Clone)]
/// A size of a given rectangle
pub struct Size {
    pub w: u16,
    pub h: u16,
}

#[derive(Clone, Copy)]
/// A square with a top left point and width + height
pub struct Square {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

impl Square {
    pub fn move_y(&mut self, y: i16) {
        if y == 0 {
            return;
        } else if y < 0 {
            self.y = match self.y.checked_sub((y * -1) as u16) {
                Some(res) => res,
                None => 0,
            }
        } else {
            self.y = match self.y.checked_add(y as u16) {
                Some(res) => res,
                None => u16::max_value(),
            }
        }
    }

    pub fn move_x(&mut self, x: i16) {
        if x == 0 {
            return;
        } else if x < 0 {
            self.x = match self.x.checked_sub((x * -1) as u16) {
                Some(res) => res,
                None => 0,
            }
        } else {
            self.x = match self.x.checked_add(x as u16) {
                Some(res) => res,
                None => u16::max_value(),
            }
        }
    }

    /// Keeps the given square inside the screen
    pub fn keep_inside(&mut self, screen_size: Size) {
        if screen_size.w < self.x + self.w {
            self.x = screen_size.w - self.w;
        }

        if screen_size.h < self.y + self.h {
            self.y = screen_size.h - self.h;
        }
    }
}
