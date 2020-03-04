use crossterm::style;

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
#[derive(Debug)]
pub struct StaticEntity {
    pub collider: Square,
    pub alive: bool,
    pub color: style::Color,
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

#[derive(Clone, Copy, Debug)]
/// A square with a top left point and width + height
pub struct Square {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

pub enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}

impl Square {
    pub fn collides_with(&self, other: &Square) -> bool {
        return self.x < other.x + other.w
            && self.x + self.w > other.x
            && self.y < other.y + other.h
            && self.y + self.h > other.y;
    }

    pub fn get_center(&self) -> Vec2d {
        Vec2d {
            x: (self.x + self.w / 2) as i16,
            y: (self.y + self.y / 2) as i16,
        }
    }

    pub fn get_collision_direction(&self, other: &Square) -> Direction {
        let self_center = self.get_center();
        let other_center = other.get_center();
        let dy = (self_center.y - other_center.y) as f64;
        let dx = (self_center.x - other_center.x) as f64;
        // add pi/8 onto our angle, that way our comparisons below are on even angles rather than partial angles
        // This turns our comparisons on 45deg angles into comparisons on 90deg angles (but in radians)
        let angle = dy.atan2(dx) + std::f64::consts::FRAC_PI_8;
        if angle < std::f64::consts::FRAC_PI_4 {
            return Direction::RIGHT;
        } else if angle < std::f64::consts::FRAC_PI_2 {
            return Direction::UP;
        } else if angle < std::f64::consts::FRAC_PI_4 * 3.0 {
            return Direction::LEFT;
        } else {
            return Direction::DOWN;
        }
    }

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
