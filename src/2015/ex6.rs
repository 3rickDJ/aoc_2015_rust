use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::time::Instant;
use once_cell::sync::Lazy;

#[derive(Debug)]
struct Point (u16,u16);
impl Point {
    pub fn new(p1: u16, p2: u16) -> Point {
        Point(p1, p2)
    }
    pub fn from_str(p1: &str, p2: &str) -> Point {
        let p1 = p1.parse::<u16>().unwrap();
        let p2 = p2.parse::<u16>().unwrap();
        Point::new(p1,p2)
    }
}
#[derive(Debug)]
enum Instruction {
    ON(Point, Point),
    OFF(Point, Point),
    TOGGLE(Point, Point),
}

impl Instruction {
    pub fn new(line: &str) -> Instruction {
        static RE: Lazy<Regex> = Lazy::new(|| {
            Regex::new(r"(?<type>turn off|turn on|toggle) (?<p1>\d+),(?<p2>\d+) through (?<p3>\d+),(?<p4>\d+)").unwrap()
        });
        let caps = RE.captures(&line).unwrap();
        let a = Point::from_str(&caps["p1"], &caps["p2"]);
        let b = Point::from_str(&caps["p3"], &caps["p4"]);
        match &caps["type"] {
            "turn on" => Instruction::ON(a,b),
            "turn off" => Instruction::OFF(a,b),
            "toggle" => Instruction::TOGGLE(a,b),
            _ => panic!("Sin match")
        }
    }
}

fn main() {
    let total_time = Instant::now();
    let f = File::open("input/2015/6").unwrap();
    let reader = BufReader::new(f);
    let mut grid: Vec<Vec<i128>> = vec![vec![0;1000]; 1000];

    let start = Instant::now();
    let mut instructions: Vec<Instruction> = vec![];
    for (_, line) in reader.lines().enumerate(){
        let line =  match line { Ok(line) => line, Err(e) => panic!("{e}") };
        let inst = Instruction::new(&line);
        instructions.push(inst);
    }
    println!("INstructions parsing: {:?}", start.elapsed());

    let start = Instant::now();
    for ins in instructions {
        match ins {
            Instruction::ON(Point(a,b),Point(c,d))=> {
                for i in a..=c{
                    for j in b..=d {
                        grid[i as usize][j as usize] += 1;
                    }
                }
            },
            Instruction::OFF(Point(a,b),Point(c,d))=> {
                for i in a..=c{
                    for j in b..=d {
                        if grid[i as usize][j as usize] > 0 {
                            grid[i as usize][j as usize] -= 1;
                        }
                    }
                }
            },
            Instruction::TOGGLE(Point(a,b),Point(c,d))=> {
                for i in a..=c{
                    for j in b..=d {
                        grid[i as usize][j as usize] += 2;
                    }
                }
            },
        }
    }
    println!("Instruction execution: {:?}", start.elapsed());

    let start = Instant::now();
    let mut counter = 0;
    for row in grid {
        for e in row{
            counter += e;
        }
    }
    println!("Addition: {:?}", start.elapsed());
    println!("Counter={:?}", counter);
    println!("Total_time={:?}", total_time.elapsed());
}
