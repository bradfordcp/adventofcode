#[derive(Debug, Eq, PartialEq)]
pub enum Instruction {
  GoUp,
  GoDown
}

/// Follows the instructions supplied, returning the resulting floor number
pub fn climb_stairs(instructions: &Vec<Instruction>) -> i64 {
  instructions.into_iter().fold(0, |floor, instruction| {
    match instruction {
        Instruction::GoUp => floor + 1,
        Instruction::GoDown => floor - 1
    }
  })
}

/// Follows the instructions supplied, returning the position where we enter the basement
pub fn steps_to_basement(instructions: &Vec<Instruction>) -> i64 {
  let s = instructions.into_iter().fold((0, 1, None), |(floor, steps, first_time_to_basement_step), instruction| {
    match first_time_to_basement_step {
      Some(step) => (-1, -1, Some(step)),
      None => {
        let floor = match instruction {
          Instruction::GoUp => floor + 1,
          Instruction::GoDown => floor - 1
        };
        
        if floor == -1 {
          (-1, -1, Some(steps))
        } else {
          (floor, steps + 1, None)
        }
      }
    }
  });
  
  s.2.expect("Unexpected error, never entered basement")
}

/// Parses a string into a collection of instructions
pub fn parse(instructions: &str) -> Vec<Instruction> {
  instructions.trim().chars().into_iter().map(|e| {
    match e {
      '(' => Instruction::GoUp,
      ')' => Instruction::GoDown,
      _ => panic!("Unexpected element {e:?}")
    }
  }).collect()
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_parse() {
    let left = parse("(");
    let right = vec![Instruction::GoUp];

    assert_eq!(left, right);

    let left = parse(")");
    let right = vec![Instruction::GoDown];

    assert_eq!(left, right);

    let left = parse("()");
    let right = vec![Instruction::GoUp, Instruction::GoDown];

    assert_eq!(left, right);
  }

  fn validate_climb_stairs(input: &str, expected: i64) {
    let instructions = parse(input);
    let pos = climb_stairs(&instructions);

    assert_eq!(expected, pos);
  }

  #[test]
  fn case_one() {
    validate_climb_stairs("(())", 0);
    validate_climb_stairs("()()", 0);
  }

  #[test]
  fn case_two() {
    validate_climb_stairs("(((", 3);
    validate_climb_stairs("(()(()(", 3);
  }

  #[test]
  fn case_three() {
    validate_climb_stairs("))(((((", 3);
  }

  #[test]
  fn case_four() {
    validate_climb_stairs("())", -1);
    validate_climb_stairs("))(", -1);
  }

  #[test]
  fn case_five() {
    validate_climb_stairs(")))", -3);
    validate_climb_stairs(")())())", -3);
  }

  #[test]
  fn case_six() {
    let instructions = parse(")");
    let real = steps_to_basement(&instructions);

    assert_eq!(1, real);
  }

  #[test]
  fn case_seven() {
    let instructions = parse("()())");
    let real = steps_to_basement(&instructions);

    assert_eq!(5, real);
  }
}
