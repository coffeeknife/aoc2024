use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumIter)]
pub enum Direction { Up, Down, Left, Right }
impl Direction {
    pub fn step(&self, pt: &Point) -> Option<Point> {
        match self {
            Direction::Up => if (pt.y > 0) { Some(Point::new(pt.x, pt.y - 1)) } else { None },
            Direction::Left => if (pt.x > 0) { Some(Point::new(pt.x - 1, pt.y)) } else { None },
            Direction::Down => Some(Point::new(pt.x, pt.y + 1)),
            Direction::Right => Some(Point::new(pt.x + 1, pt.y))
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Point { pub x: usize, pub y: usize }
impl Point { 
    pub fn new(x: usize, y: usize ) -> Self { Self { x, y } } 

    pub fn adjacent(&self) -> Vec<Point> {
        Direction::iter().map(|d: Direction| { d.step(self) }).flatten().collect()
    }
}