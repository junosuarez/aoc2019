fn main() {
  let mut memory = parse("1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,5,19,23,1,13,23,27,1,6,27,31,2,31,13,35,1,9,35,39,2,39,13,43,1,43,10,47,1,47,13,51,2,13,51,55,1,55,9,59,1,59,5,63,1,6,63,67,1,13,67,71,2,71,10,75,1,6,75,79,1,79,10,83,1,5,83,87,2,10,87,91,1,6,91,95,1,9,95,99,1,99,9,103,2,103,10,107,1,5,107,111,1,9,111,115,2,13,115,119,1,119,10,123,1,123,10,127,2,127,10,131,1,5,131,135,1,10,135,139,1,139,2,143,1,6,143,0,99,2,14,0,0".to_string());
  // before running the program, replace position 1 with the value 12 and replace position 2 with the value 2
  memory[1] = 12;
  memory[2] = 2;
  memory = calculate(memory);

  println!("{}", render(memory));
}

fn parse(expr: String) -> Vec<i32> {
  let memory: Vec<i32> = expr
    .split(",")
    .map(|s| s.parse::<i32>().expect("x"))
    .collect();
  return memory;
}

fn render(memory: Vec<i32>) -> String {
  return memory
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>()
    .join(",");
}

fn calculate(mut memory: Vec<i32>) -> Vec<i32> {
  let mut instruction_pointer = 0;

  loop {
    let opcode = memory[instruction_pointer];
    if opcode == 99 {
      // halt immediately
      break;
    }
    let left_op = memory[instruction_pointer + 1] as usize;
    let right_op = memory[instruction_pointer + 2] as usize;
    let dest = memory[instruction_pointer + 3] as usize;
    println!("op {}", opcode);
    match opcode {
      1 => {
        // add
        println!("add {} {} to {}", left_op, right_op, dest);
        memory[dest] = memory[left_op] + memory[right_op];
        instruction_pointer += 4;
      }
      2 => {
        // multiply
        println!("mul {} {} to {}", left_op, right_op, dest);
        memory[dest] = memory[left_op] * memory[right_op];
        instruction_pointer += 4;
      }
      _ => {
        // error
        panic!("error at instruction_pointer {}", instruction_pointer);
      }
    }
  }

  return memory;
}

fn calc_str(expr: String) -> String {
  return render(calculate(parse(expr)));
}

fn calculate_str(expr: String) -> String {
  let mut memory: Vec<i32> = expr
    .split(",")
    .map(|s| s.parse::<i32>().expect("x"))
    .collect();

  let mut instruction_pointer = 0;

  loop {
    let opcode = memory[instruction_pointer];
    if opcode == 99 {
      // halt immediately
      break;
    }
    let left_op = memory[instruction_pointer + 1] as usize;
    let right_op = memory[instruction_pointer + 2] as usize;
    let dest = memory[instruction_pointer + 3] as usize;
    println!("op {}", opcode);
    match opcode {
      1 => {
        // add
        println!("add {} {} to {}", left_op, right_op, dest);
        memory[dest] = memory[left_op] + memory[right_op];
      }
      2 => {
        // multiply
        println!("mul {} {} to {}", left_op, right_op, dest);
        memory[dest] = memory[left_op] * memory[right_op];
      }
      _ => {
        // error
        panic!("error at instruction_pointer {}", instruction_pointer);
      }
    }

    instruction_pointer += 4;
  }

  return memory
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>()
    .join(",");
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn ex_1() {
    assert_eq!("2,0,0,0,99", calc_str("1,0,0,0,99".to_string()));
  }

  #[test]
  fn ex_2() {
    assert_eq!("2,3,0,6,99", calc_str("2,3,0,3,99".to_string()));
  }

  #[test]
  fn ex_3() {
    assert_eq!("2,4,4,5,99,9801", calc_str("2,4,4,5,99,0".to_string()));
  }

  #[test]
  fn ex_4() {
    assert_eq!(
      "30,1,1,4,2,5,6,0,99",
      calc_str("1,1,1,4,99,5,6,0,99".to_string())
    );
  }
}
