use std::time::Instant;
fn main () {
  let _start = Instant::now();
  let contents = std::fs::read_to_string("input/2015/8").unwrap();
  let result = match_sticks::code_minus_mem_chars(&contents);
  println!("Result: {result:?}");
}

mod match_sticks {

  pub fn code_minus_mem_chars(input: &str) -> usize {
    let mut counter: usize = 0;
    for line in input.lines(){
      let (mem, code) = self::count_line(line);
      counter += code - mem;
    }
    counter
  }

  pub fn count_line(line: &str) -> (usize, usize) {
    let mut mem_count: usize = 0;
    let mut code_count: usize = 0;
    let mut escape: u8 = 0;
    for char in line.chars(){
      if escape > 0 {
        match char {
          '\\' => {
            escape=0;
          },
          '"' => {
            escape=0;
          },
          'x' => {
            escape-=1;
          },
          '0'..='9' => {
            escape-=1;
          },
          'a'..='f' => {
            escape-=1;
          }
          c => {
            println!("Caso INvalido {c:?}");
            panic!("Caso inálido!!!");
          },
        }
      } else {
        match char {
          '\\' => {
            escape = 3;
            mem_count+=1;
          },
          '"' => {
          },
          c => {
            mem_count += 1;
          }
        }
      }
      code_count += 1;
    }
    (mem_count, code_count)
  }

}

#[cfg(test)]
mod test {
  use super::*;
  use indoc::indoc;

  #[test]
  fn test_simple_double_quotes() {
    let (mem, code) = match_sticks::count_line(r#""""#);
    assert_eq!(mem,0);
    assert_eq!(code,2);
    let mem_code = match_sticks::count_line(r#""abc""#);
    assert_eq!(mem_code, (3,5));
  }

  #[test]
  fn test_escape_sequences() {
    let mem_code = match_sticks::count_line(r#""aaa\"aaa""#);
    assert_eq!(mem_code, (7,10));
    let mem_code = match_sticks::count_line(r#""\x27""#);
    assert_eq!(mem_code, (1,6));
    let mem_code = match_sticks::count_line(r#""\xbf""#);
    assert_eq!(mem_code, (1,6));
  }

  #[test]
  fn test_code_minus_memory_characters() {
    let santas_list = indoc! {r#"
    ""
    "abc"
    "aaa\"aaa"
    "\x27""#
    };
    // let matchsticks = match_sticks::(santas_list);
    let result = match_sticks::code_minus_mem_chars(santas_list);
    assert_eq!(result, 12);
  }
}