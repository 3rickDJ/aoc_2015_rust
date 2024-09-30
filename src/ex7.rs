use std::collections::HashMap; use std::fs; use regex::Regex; use once_cell::sync::Lazy; use std::time::Instant;
fn main() {
  let input = fs::read_to_string("input/7").unwrap();
  let start = Instant::now();
  let circuit = Circuit::new(input);
  println!("{}", circuit.get_value("t"));
  println!("Elapsed time: {:.2?}", start.elapsed());
}

struct Circuit {
  instructions: HashMap<String, Operation>,
  mem: HashMap<String, u16>
}

impl Circuit {
  fn new(input: String) -> Self {
    let mut mp = HashMap::new();

    for line in input.lines() {
      let (operation, bind) = parse_line(line);
      mp.insert(bind, operation);
    }

    Circuit {
      instructions: mp,
      mem: HashMap::new()
    }
  }

  fn get_value(&self, key: &str) -> u16 {
    println!("key: {}", key);
    match key.parse::<u16>() {
      Ok(v) => v,
      Err(_) => {
        let operation = self.instructions.get(key).unwrap();
        match operation {
          Operation::Wire(wire) => self.get_value(wire),
          Operation::And(left, right) => self.get_value(left) & self.get_value(right),
          Operation::Or(left, right) => self.get_value(left) | self.get_value(right),
          Operation::Lshift(left, right) => self.get_value(left) << right,
          Operation::Rshift(left, right) => self.get_value(left) >> right,
          Operation::Not(right) => !self.get_value(right),
        }
      }
    }
  }

}



#[derive(Debug)]
enum Operation {
  Wire(String),
  And(String, String),
  Or(String, String),
  Lshift(String, u16),
  Rshift(String, u16),
  Not(String),
}

fn parse_line(line: &str) -> (Operation, String) {
    static RE_WIRE: Lazy<Regex> = Lazy::new(|| {
        Regex::new(r"^(?<op1>\w+) -> (?<bind>\w+)$").unwrap()
        });
    static RE_BITWISE: Lazy<Regex> = Lazy::new( || {
      Regex::new(r"^(?<left>\w+) (?<op>AND|OR|LSHIFT|RSHIFT) (?<right>\w+) -> (?<bind>\w+)$").unwrap()
    });
    static RE_NOT: Lazy<Regex> = Lazy::new( || {
      Regex::new(r"^(?<op>NOT) (?<right>\w+) -> (?<bind>\w+)$").unwrap()
    });

    if let Some(caps) = RE_WIRE.captures(line){
      let op1 = caps["op1"].to_string();
      let bind = caps["bind"].to_string();
      return (Operation::Wire(op1), bind);
    } else if let Some(caps) = RE_BITWISE.captures(line){
      let left = caps["left"].to_string();
      let op = caps["op"].to_string();
      let right = caps["right"].to_string();
      let bind = caps["bind"].to_string();
      return match op.as_str(){
        "AND" => (Operation::And(left, right), bind),
        "OR" => (Operation::Or(left, right), bind),
        "LSHIFT" => {
            let right = right.parse::<u16>().unwrap();
            (Operation::Lshift(left, right), bind)
          },
        "RSHIFT" => {
            let right = right.parse::<u16>().unwrap();
            (Operation::Rshift(left, right), bind)
        },
        _ => panic!("Sin match")
      }
    } else if let Some(caps) = RE_NOT.captures(line){
      let right = caps["right"].to_string();
      let bind = caps["bind"].to_string();
      return (Operation::Not(right), bind);
    } else {
      panic!("Sin match")
    }
  }

#[cfg(test)] mod test {
  use super::*;
  use indoc::indoc;

  #[test]
  fn test_simple_circuit(){
    let input = indoc!{"
      123 -> x
      456 -> y
      x AND y -> d
      x OR y -> e
      x LSHIFT 2 -> f
      y RSHIFT 2 -> g
      NOT x -> h
      NOT y -> i"
      }.to_string();
      let circuit = Circuit::new(input);
      let i_val = circuit.get_value("i");
      assert_eq!(i_val, 65079);
  }
}