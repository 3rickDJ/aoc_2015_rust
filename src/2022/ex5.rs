use std::fs;
use std::time::Instant;

fn main() {
  let input = fs::read_to_string("input/2022/5").unwrap();
  let start = Instant::now();
  let supply = SupplyStack::new(input.as_str());
  let tops = supply.tops();
  println!("Tops: {tops:?}");
  println!("Elapsed time: {:.2?}", start.elapsed());
}

#[derive(Debug)]
struct SupplyStack {
  stacks: Vec<Vec<char>>
}

#[derive(Debug)]
struct Movement {
  qty: u16,
  from: usize,
  to: usize
}

impl Movement {
  pub fn new(input: &str) -> Self {
    let parts = input.split_whitespace().collect::<Vec<&str>>();
    let qty = parts[1].parse::<u16>().unwrap();
    let from = parts[3].parse::<usize>().unwrap();
    let to = parts[5].parse::<usize>().unwrap();
    Self { qty, from, to }
  }
}

impl SupplyStack {
  pub fn new(input: &str) -> Self {
    let (grid, movements) = input.split_once("\n\n").unwrap();
    // drop last line
    let mut lines = grid.lines().collect::<Vec<&str>>();
    lines.pop();
    lines.reverse();
    let stack_number = (lines[0].len() + 1)/4;
    let mut stacks = vec![vec![]; stack_number];

    for line in lines {
      for (i, c) in line.chars().enumerate() {
        if i % 4 == 1 {
          match c {
            ' ' => continue,
            c => stacks[i/4].push(c),
          }
        }
      }
    }
    let movements = movements.lines().map(|line| Movement::new(line)).collect::<Vec<Movement>>();
    for movement in movements {
      Self::move_stack(&mut stacks, movement);
    }

    Self {
      stacks
    }
  }

  fn move_stack(stacks: &mut Vec<Vec<char>>, movement: Movement) {
    for _ in 0..movement.qty {
      let val = stacks[movement.from - 1].pop().unwrap();
      stacks[movement.to - 1].push(val);
    }
  }

  fn tops(&self) -> String {
    let mut tops = String::new();
    for stack in &self.stacks {
      match stack.last() {
        Some(c) => tops.push(*c),
        None => {},
      };
    }
    tops
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use indoc::indoc;

  #[test]
  fn test_parse_grid_to_stacks(){
    let input = indoc!{"
        [D]    
    [N] [C]    
    [Z] [M] [P]
     1   2   3 
    
    move 1 from 2 to 1
    move 3 from 1 to 3
    move 2 from 2 to 1
    move 1 from 1 to 2"
    };

    let supply = SupplyStack::new(input);
    let stacks = &supply.stacks;
    let expected = vec![
      vec!['C'],
      vec!['M'],
      vec!['P', 'D', 'N', 'Z']
      ];

    assert_eq!(stacks, &expected);

    let top = supply.tops();
    assert_eq!(top, "CMZ".to_string());

  }
}