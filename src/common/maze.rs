use std::collections::{HashMap, HashSet};

use super::{map::Point, misc::safe_add};

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

// a maze map that calculates a grid of how far each point is from the end.
pub struct DistMap {
    pub map: MazeMap,
    pub distances: HashMap<Point, usize>
}

impl DistMap {
    pub fn new(in_map: Vec<Vec<char>>) -> Self {
        let map: MazeMap = MazeMap::new(in_map);
        let mut distances: HashMap<Point, usize> = HashMap::new();
        distances.insert(map.end, 0);


        for point in map.adjacent(map.end) {
            cascade_update(&point, &map, &mut distances, &map.end);
        }

        map.adjacent(map.end).iter().map(|p| { cascade_update(&p, &map, &mut distances, &map.end) });

        Self { map, distances }
    }
}

fn cascade_update(pt: &Point, map: &MazeMap, dist_map: &mut HashMap<Point, usize>, trigger: &Point) {
    println!("trying to cascade {:?}", pt);
    if pt == trigger { return } // don't overwrite the point we're trying to cascade off of
    println!("Made it past the trigger check");

    let cur_val: usize = *dist_map.get(pt).unwrap_or_else(|| &usize::MAX);
    let min_adj: usize = safe_add(1, map.adjacent(*pt).iter().map(|p: &Point| -> usize {
        *dist_map.get(p).unwrap_or_else(|| &usize::MAX)
    }).min().unwrap_or(usize::MAX));
    
    dist_map.entry(*pt).and_modify(|x: &mut usize| *x = min_adj).or_insert(min_adj);
    
    if cur_val != min_adj {
        for point in map.adjacent(*pt) {
            cascade_update(pt, map, dist_map, trigger);
        }
    }
}