use std::collections::HashMap;
use std::fs;
use std::time::Instant;
fn main() {
    let input = fs::read_to_string("input/7").unwrap();
    let mut circuit = Circuit::new(input);
    let start = Instant::now();
    let a_val = circuit.get_value("a");
    println!("{}", a_val);
    println!("Elapsed time: {:.2?}", start.elapsed());
    circuit.reset();
    circuit.set("b", a_val);
    let a_val = circuit.get_value("a");
    println!("{}", a_val);
    println!("Elapsed time: {:.2?}", start.elapsed());
}

struct Circuit {
    instructions: HashMap<String, Operation>,
    mem: HashMap<String, u16>,
}

impl Circuit {
    fn new(input: String) -> Self {
        let mut mp = HashMap::new();

        let start = Instant::now();
        for line in input.lines() {
            let (operation, bind) = parse_line(line);
            mp.insert(bind, operation);
        }
        println!("Elapsed pasing time: {:.2?}", start.elapsed());
        Circuit {
            instructions: mp,
            mem: HashMap::new(),
        }
    }

    fn get_value(&mut self, key: &str) -> u16 {
        //cached
        if let Some(&num) = self.mem.get(key) {
            return num;
        }
        //base case
        if let Ok(num) = key.parse::<u16>() {
            return num;
        }
        //recursion
        let operation = self.instructions.get(key).unwrap().clone();
        let result = match operation {
            Operation::Wire(wire) => self.get_value(&wire),
            Operation::And(left, right) => self.get_value(&left) & self.get_value(&right),
            Operation::Or(left, right) => self.get_value(&left) | self.get_value(&right),
            Operation::Lshift(left, right) => self.get_value(&left) << right,
            Operation::Rshift(left, right) => self.get_value(&left) >> right,
            Operation::Not(right) => !self.get_value(&right),
        };
        self.mem.insert(key.to_string(), result);
        result
    }

    fn reset(&mut self) { self.mem.clear() }

    fn set(&mut self, key: &str, value: u16) { self.mem.insert(key.to_string(), value);}
}

#[derive(Debug, Clone)]
enum Operation {
    Wire(String),
    And(String, String),
    Or(String, String),
    Lshift(String, u16),
    Rshift(String, u16),
    Not(String),
}

fn parse_line(line: &str) -> (Operation, String) {
  let parts: Vec<&str> = line.split_whitespace().collect();
  match parts.len() {
      3 => {
          // Handles operations like "123 -> x"
          let bind = parts[2].to_string();
          let op1 = parts[0].to_string();
          (Operation::Wire(op1), bind)
      }
      4 => {
          // Handles operations like "NOT x -> h"
          let bind = parts[3].to_string();
          let right = parts[1].to_string();
          (Operation::Not(right), bind)
      }
      5 => {
          // Handles operations like "x AND y -> d", "x OR y -> e", etc.
          let left = parts[0].to_string();
          let op = parts[1];
          let right = parts[2].to_string();
          let bind = parts[4].to_string();
          
          match op {
              "AND" => (Operation::And(left, right), bind),
              "OR" => (Operation::Or(left, right), bind),
              "LSHIFT" => {
                  let right_val = right.parse::<u16>().unwrap();
                  (Operation::Lshift(left, right_val), bind)
              }
              "RSHIFT" => {
                  let right_val = right.parse::<u16>().unwrap();
                  (Operation::Rshift(left, right_val), bind)
              }
              _ => panic!("Unrecognized operation"),
          }
      }
      _ => panic!("Invalid instruction format"),
  }
}

#[cfg(test)]
mod test {
    use super::*;
    use indoc::indoc;

    #[test]
    fn test_simple_circuit() {
        let input = indoc! {"
      123 -> x
      456 -> y
      x AND y -> d
      x OR y -> e
      x LSHIFT 2 -> f
      y RSHIFT 2 -> g
      NOT x -> h
      NOT y -> i"
        }
        .to_string();
        let mut circuit = Circuit::new(input);
        let i_val = circuit.get_value("i");
        assert_eq!(i_val, 65079);
    }
}
