use std::{fs::File, io::{BufRead, BufReader}, path::Path};

pub fn day2(input: String) {
    println!("[RUNNING DAY 2]");
    let reports = parse_input(input);
    let mut part1: u64 = 0;
    let mut part2: u64 = 0;
    for report in reports {
        let report_good: bool = check_report(&report);
        if report_good { part1 += 1; part2 += 1 }
        else {
            for i in 0..report.len() {
                let mut subreport = report.clone();
                subreport.remove(i);
                if check_report(&subreport) {
                    part2 += 1;
                    break;
                }
            }
        }
    }
    println!("Part 1 Result: {part1}");
    println!("Part 2 Result: {part2}");
}

fn check_report(report: &Vec<u64>) -> bool {
    let mut direction = 0; // 1 = ascending, 2 - descending
    for i in 0..(report.len() - 1) {
        let (a, b) = (report.get(i).unwrap(), report.get(i + 1).unwrap());
        if a == b { return false; }
        else if ((*a as i64) - (*b as i64)).abs() > 3 { return false }
        else if a < b {
            if direction == 2 { return false } // direction change: bad
            else { direction = 1 } // set direction to ascending
        } else if direction == 1 { return false } // direction must be descending
        else { direction = 2 } // set direction to descending
    }
    true
}

fn parse_input(input: String) -> Vec<Vec<u64>> {
    let mut reports: Vec<Vec<u64>> = Vec::new();

    let file: File = File::open(Path::new(&input)).expect("Err opening file");
    let lines = BufReader::new(file).lines().flatten();

    for line in lines {
        let report: Vec<u64> = line.split_whitespace().map(|s| s.parse::<u64>()).map(|x| x.unwrap()).collect();
        reports.push(report);
    }

    reports
}