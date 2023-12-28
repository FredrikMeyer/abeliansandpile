#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn up(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }
    pub fn down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    pub fn left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    pub fn right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
}
