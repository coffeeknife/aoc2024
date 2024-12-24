use std::{fs::File, io::{BufRead, BufReader}, path::Path};

pub fn day21(input: String) {
    let codes: Vec<Vec<KeypadNum>> = parse_input(input);
    
    let mut converted: Vec<Vec<KeypadDir>> = Vec::new();

    for code in &codes {
        let mut extrapolated: Vec<KeypadDir> = Vec::new();
        for i in 0..code.len() - 1 {
            let cur = &code[i]; let next = &code[i+1];
            let mut steps = cur.extrapolate_steps(next);
            extrapolated.append(&mut steps);
        }
        converted.push(extrapolated);
    }

    for entry in converted {
        println!("{}", extrapolate(extrapolate(entry)).len())
    }
}

fn extrapolate(input: Vec<KeypadDir>) -> Vec<KeypadDir> {
    let mut extrapolated: Vec<KeypadDir> = Vec::new();
    for i in 0..input.len() - 1 {
        let cur = &input[i]; let next = &input[i+1];
        let mut steps = cur.extrapolate_steps(next);
        extrapolated.append(&mut steps);
    }
    extrapolated
}

fn parse_input(input: String) -> Vec<Vec<KeypadNum>> {
    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    let in_raw: Vec<Vec<char>> = BufReader::new(file).lines().flatten().map(|x:String| x.chars().collect()).collect();
    let mut codes: Vec<Vec<KeypadNum>> = Vec::new();

    for entry in in_raw {
        let mut code: Vec<KeypadNum> = Vec::new();
        code.push(KeypadNum::Activate); // start position
        for chr in entry {
            code.push(match chr {
                '0' => KeypadNum::Zero,
                '1' => KeypadNum::One,
                '2' => KeypadNum::Two,
                '3' => KeypadNum::Three,
                '4' => KeypadNum::Four,
                '5' => KeypadNum::Five,
                '6' => KeypadNum::Six,
                '7' => KeypadNum::Seven,
                '8' => KeypadNum::Eight,
                '9' => KeypadNum::Nine,
                'A' => KeypadNum::Activate,
                _ => panic!("Shouldn't happen")
            });
        }
        codes.push(code);
    }

    codes
}

trait Keypad {
    fn get_point(&self) -> (usize, usize);
    fn get_dist(&self, other: &impl Keypad) -> (i32, i32) {
        let other_pt: (usize, usize) = other.get_point(); let self_pt: (usize, usize) = self.get_point();
        (other_pt.0 as i32 - self_pt.0 as i32, other_pt.1 as i32 - self_pt.1 as i32)
    }

    fn extrapolate_steps(&self, other: &impl Keypad) -> Vec<KeypadDir> {
        let mut steps: Vec<KeypadDir> = Vec::new();
        let dist: (i32, i32) = self.get_dist(other);

        for _ in 0..dist.0.abs() {
            if dist.0 < 0 { steps.push(KeypadDir::Up) }
            else { steps.push(KeypadDir::Down) }
        }

        for _ in 0..dist.1.abs() {
            if dist.1 < 0 { steps.push(KeypadDir::Left) }
            else { steps.push(KeypadDir::Right) }
        }

        steps.push(KeypadDir::Activate);

        steps
    }
}

#[derive(Debug)]
enum KeypadNum { Seven, Eight, Nine, Four, Five, Six, One, Two, Three, Zero, Activate }
#[derive(Debug)]
enum KeypadDir { Up, Down, Left, Right, Activate }

impl Keypad for KeypadNum {
    fn get_point(&self) -> (usize, usize) {
        match self {
            KeypadNum::Seven => (0, 0),
            KeypadNum::Eight => (0, 1),
            KeypadNum::Nine => (0, 2),
            KeypadNum::Four => (1, 0),
            KeypadNum::Five => (1, 1),
            KeypadNum::Six => (1, 2),
            KeypadNum::One => (2, 0),
            KeypadNum::Two => (2, 1),
            KeypadNum::Three => (2, 2),
            KeypadNum::Zero => (3, 1),
            KeypadNum::Activate => (3, 2),
        }
    }
}

impl Keypad for KeypadDir {
    fn get_point(&self) -> (usize, usize) {
        match self {
            KeypadDir::Up => (0, 1),
            KeypadDir::Down => (1, 1),
            KeypadDir::Left => (1, 0),
            KeypadDir::Right => (1, 2),
            KeypadDir::Activate => (0, 2),
        }
    }
}