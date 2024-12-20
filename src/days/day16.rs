use std::{cmp::Ordering, collections::{BinaryHeap, HashMap, HashSet}, fs::File, io::{BufRead, BufReader}, path::Path};

use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, EnumIter)]
enum Direction { Up, Down, Left, Right }
impl Direction {
    fn step(&self, pt: &Point) -> Point {
        match self {
            Direction::Up => Point::new(pt.x, pt.y - 1),
            Direction::Left => Point::new(pt.x - 1, pt.y),
            Direction::Down => Point::new(pt.x, pt.y + 1),
            Direction::Right => Point::new(pt.x + 1, pt.y)
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Step { pos: Point, dir: Direction, score: usize }
impl Step { fn new(pos: Point, dir: Direction, score: usize) -> Self { Self { pos, dir, score } } }
impl Ord for Step { fn cmp(&self, other: &Self) -> Ordering { other.score.cmp(&self.score) } }
impl PartialOrd for Step { fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) } }

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct Point { x: usize, y: usize }
impl Point { 
    fn new(x: usize, y: usize ) -> Self { Self { x, y } } 

    fn adjacent(&self, dir: Direction) -> Vec<(Point, Direction, usize)> {
        Direction::iter().filter(|d: &Direction| *d != dir.opposite()).map(|d: Direction| {
            let cost: usize = if dir != d { 1001 } else { 1 };
            (d.step(self), d, cost)
        }).collect()
    }
}

#[derive(Debug, Clone)]
struct Map {
    spaces: HashSet<Point>,
    start: Point,
    end: Point
}

impl Map {
    fn new(in_map: Vec<Vec<char>>) -> Self {
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

    fn adjacent(&self, pt: Point, dir: Direction) -> Vec<(Point, Direction, usize)> {
        pt.adjacent(dir).into_iter().filter(|(p, _, _)| self.spaces.contains(p)).collect()
    }

    fn shortest_path(&self) -> usize {
        let mut shortest: usize = usize::MAX;
        let mut cache: HashMap<(Point, Direction), usize> = HashMap::new();

        // thank you internet for teaching me how to do binary searches
        let mut queue: BinaryHeap<Step> = BinaryHeap::new();
        queue.push(Step::new(self.start, Direction::Right, 0));

        while let Some(Step { pos, dir, score }) = queue.pop() {
            if let Some(&prev_score) = cache.get(&(pos, dir)) {
                if score > prev_score { continue }
            } else { cache.insert((pos, dir), score); }

            if pos == self.end && score < shortest { 
                shortest = score; 
            }
            
            for (adj, dir_new, move_cost) in self.adjacent(pos, dir) {
                queue.push(Step::new(adj, dir_new, score + move_cost));
            }
        }

        shortest
    }
}

pub fn day16(input: String) {
    let in_map: Map = Map::new(parse_input(input));
    println!("Part 1: {}", in_map.shortest_path());
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    BufReader::new(file).lines().flatten().map(|x:String| x.chars().collect()).collect()
}