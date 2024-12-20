use std::collections::HashSet;

use super::map::Point;

#[derive(Debug, Clone)]
pub struct Map {
    pub spaces: HashSet<Point>,
    pub start: Point,
    pub end: Point
}