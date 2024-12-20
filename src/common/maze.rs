use std::collections::HashSet;

use super::map::Point;

#[derive(Debug, Clone)]
pub struct MazeMap {
    pub spaces: HashSet<Point>,
    pub start: Point,
    pub end: Point
}

impl MazeMap {
    pub fn new(in_map: Vec<Vec<char>>) -> Self {
        let mut spaces: HashSet<Point> = HashSet::new();
        let mut start: Point = Point { x: 0, y: 0 };
        let mut end: Point = Point { x: 0, y: 0 };

        for y in 0..in_map.len() {
            for x in 0..in_map[0].len() {
                if in_map[y][x] == 'S' {
                    start = Point { x, y };
                } else if in_map[y][x] == 'E' {
                    end = Point { x, y };
                }
                
                if in_map[y][x] != '#' {
                    spaces.insert(Point { x, y });
                }
            }
        }

        Self { spaces, start, end }
    }

    pub fn adjacent(&self, pt: Point) -> Vec<Point> {
        pt.adjacent().into_iter().filter(|p| self.spaces.contains(p)).collect()
    }
}