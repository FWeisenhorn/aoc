use super::direction::Direction;

#[derive(PartialOrd, Ord, PartialEq, Eq, Hash, Copy, Clone, Default, Debug)]
pub struct Pos {
    pub x: usize,
    pub y: usize,
}

impl Pos {
    pub fn steps(&self, dir: Direction, steps: usize) -> Option<Self> {
        // up, down, left, right
        match dir {
            Direction::Up => Some(Self {
                x: self.x.checked_sub(steps)?,
                y: self.y,
            }),
            Direction::Down => Some(Self {
                x: self.x.checked_add(steps)?,
                y: self.y,
            }),
            Direction::Left => Some(Self {
                x: self.x,
                y: self.y.checked_sub(steps)?,
            }),
            Direction::Right => Some(Self {
                x: self.x,
                y: self.y.checked_add(steps)?,
            }),
        }
    }

    pub fn steps_checked(
        &self,
        dir: Direction,
        steps: usize,
        max_x: usize,
        max_y: usize,
    ) -> Option<Self> {
        match self.steps(dir, steps) {
            Some(p) if p.x < max_x && p.y < max_y => Some(p),
            _ => None,
        }
    }

    pub const fn distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}
