use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}, fs::File, io::{BufRead, BufReader}, path::Path};

use strum::IntoEnumIterator;

use crate::common::{map::{Direction, Point}, maze::MazeMap};

#[derive(Debug, Eq, PartialEq, Hash)]
struct Step { pos: Point, dir: Direction, score: usize, path: Vec<Point> }
impl Step { fn new(pos: Point, dir: Direction, score: usize, path: Vec<Point>) -> Self { Self { pos, dir, score, path } } }
impl Ord for Step { fn cmp(&self, other: &Self) -> Ordering { other.score.cmp(&self.score) } }
impl PartialOrd for Step { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) } }

impl Point {
    pub fn adjacent_no_opposite(&self, dir: Direction) -> Vec<(Point, Direction, usize)> {
        Direction::iter().filter(|d: &Direction| *d != dir.opposite()).map(|d: Direction| {
            let cost: usize = if dir != d { 1001 } else { 1 };
            (d.step(self).unwrap(), d, cost)
        }).collect()
    }
}

impl MazeMap {
    fn adjacent_no_opposite(&self, pt: Point, dir: Direction) -> Vec<(Point, Direction, usize)> {
        pt.adjacent_no_opposite(dir).into_iter().filter(|(p, _, _)| self.spaces.contains(p)).collect()
    }

    fn shortest_paths(&self) -> (usize, Vec<Point>) {
        let mut shortest: usize = usize::MAX;
        let mut unique_tiles: Vec<Point> = Vec::new();
        let mut cache: HashMap<(Point, Direction), usize> = HashMap::new();

        // thank you internet for teaching me how to do binary searches
        let mut queue: BinaryHeap<Step> = BinaryHeap::new();
        queue.push(Step::new(self.start, Direction::Right, 0, vec![self.start]));

        while let Some(Step { pos, dir, score , path}) = queue.pop() {
            if let Some(&prev_score) = cache.get(&(pos, dir)) {
                if score > prev_score { continue }
            } else { cache.insert((pos, dir), score); }

            if pos == self.end && score <= shortest { 
                for step in &path {
                    if !unique_tiles.contains(step) {
                        unique_tiles.push(step.clone())
                    }
                }
                shortest = score; 
            }
            
            for (adj, dir_new, move_cost) in self.adjacent_no_opposite(pos, dir) {
                let mut path_new = path.clone();
                path_new.push(adj);
                queue.push(Step::new(adj, dir_new, score + move_cost, path_new));
            }
        }

        (shortest, unique_tiles)
    }
}

pub fn day16(input: String) {
    let in_map: MazeMap = MazeMap::new(parse_input(input));

    let (shortest, tiles) = in_map.shortest_paths();
    println!("Part 1: {}", shortest);
    println!("Part 2: {}", tiles.len());
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    BufReader::new(file).lines().flatten().map(|x:String| x.chars().collect()).collect()
}