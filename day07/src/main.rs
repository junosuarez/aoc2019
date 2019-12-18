use std::convert::TryFrom;

fn main() {}

struct Intcode {
  memory: Vec<i32>,
}
impl Intcode {
  fn new(mut memory: Vec<i32>) -> Self {
    Intcode { memory: memory }
  }
  fn start(&mut self, input: Vec<i32>) -> Vec<i32> {
    let (_, output) = calculate(&mut self.memory, input);
    return output;
  }
}

fn parse(expr: String) -> Vec<i32> {
  let memory: Vec<i32> = expr
    .split(",")
    .map(|s| s.parse::<i32>().expect("x"))
    .collect();
  return memory;
}

fn render(memory: &Vec<i32>) -> String {
  return memory
    .iter()
    .map(|s| s.to_string())
    .collect::<Vec<String>>()
    .join(",");
}

#[derive(Eq, PartialEq, Debug)]
enum ParameterMode {
  Position = 0,
  Immediate = 1,
}
fn parse_param_mode(c: char) -> ParameterMode {
  match c {
    '0' => ParameterMode::Position,
    '1' => ParameterMode::Immediate,
    _ => panic!("invalid param mode"),
  }
}

#[derive(Eq, PartialEq, Debug)]
struct Instruction {
  opcode: i8,
  mode1: ParameterMode,
  mode2: ParameterMode,
  mode3: ParameterMode,
}

// enum Parameter {
//   Immediate(i32),
//   Position(usize),
// }

impl Instruction {
  fn parse(code: i32) -> Instruction {
    // todo: putting this to a string and back is horrible and slow. bit twiddling would be an optimization.alloc
    let cstr = code.to_string();
    let mut rev = cstr.chars().rev();
    let a = rev.next().unwrap_or('0').to_digit(10).expect("");
    let b = rev.next().unwrap_or('0').to_digit(10).expect("");
    let c = rev
      .next()
      .map(parse_param_mode)
      .unwrap_or(ParameterMode::Position);
    let d = rev
      .next()
      .map(parse_param_mode)
      .unwrap_or(ParameterMode::Position);
    let e = rev
      .next()
      .map(parse_param_mode)
      .unwrap_or(ParameterMode::Position);
    // println!("{:?} {:?} {:?} {:?} {:?} {:?} ", a, b, c, d, e, code);
    Instruction {
      opcode: i8::try_from(10 * b + a).expect(""),
      mode1: c,
      mode2: d,
      mode3: e,
    }
  }
}

fn calculate(memory: &mut Vec<i32>, input: Vec<i32>) -> (&mut Vec<i32>, Vec<i32>) {
  let mut instruction_pointer = 0;
  let mut ins = input.into_iter();
  let mut output: Vec<i32> = Vec::new();

  loop {
    let instruction = Instruction::parse(memory[instruction_pointer]);
    if instruction.opcode == 99 {
      // halt immediately
      break;
    }
    // println!("op {:?}", instruction);
    match instruction.opcode {
      1 => {
        // add
        let left_op = memory[instruction_pointer + 1];
        let right_op = memory[instruction_pointer + 2];
        let dest = memory[instruction_pointer + 3] as usize;
        // println!("{:?} ADD {} {} to {}", instruction, left_op, right_op, dest);
        // println!("before: {}: {}", dest, memory[dest]);
        let left = match instruction.mode1 {
          ParameterMode::Position => memory[left_op as usize],
          ParameterMode::Immediate => left_op,
        };
        let right = match instruction.mode2 {
          ParameterMode::Position => memory[right_op as usize],
          ParameterMode::Immediate => right_op,
        };
        memory[dest] = left + right;
        // println!("after: {}: {}", dest, memory[dest]);
        instruction_pointer += 4;
      }
      2 => {
        // multiply
        let left_op = memory[instruction_pointer + 1];
        let right_op = memory[instruction_pointer + 2];
        let dest = memory[instruction_pointer + 3] as usize;
        // println!("{:?} MUL {} {} to {}", instruction, left_op, right_op, dest);
        // println!("before: {}: {}", dest, memory[dest]);
        let left = match instruction.mode1 {
          ParameterMode::Position => memory[left_op as usize],
          ParameterMode::Immediate => left_op,
        };
        let right = match instruction.mode2 {
          ParameterMode::Position => memory[right_op as usize],
          ParameterMode::Immediate => right_op,
        };
        memory[dest] = left * right;
        // println!("after: {}: {}", dest, memory[dest]);
        instruction_pointer += 4;
      }
      3 => {
        // input
        let dest = memory[instruction_pointer + 1] as usize;
        // for now, inputs are faked as `1`
        let input = ins.next().expect("missing ingput");
        println!("IN: {:?}", input);
        memory[dest] = input;
        instruction_pointer += 2;
      }
      4 => {
        // output
        let src = memory[instruction_pointer + 1];
        let out = match instruction.mode1 {
          ParameterMode::Position => memory[src as usize],
          ParameterMode::Immediate => src,
        };
        println!("OUT: {:?}", out);
        output.push(out);

        instruction_pointer += 2;
        // return (memory, output);
      }
      5 => {
        // jump-if-true: if the first parameter is non-zero,
        // it sets the instruction pointer to the value from the second parameter.
        // Otherwise, it does nothing.
        let flag = match instruction.mode1 {
          ParameterMode::Position => memory[memory[instruction_pointer + 1] as usize],
          ParameterMode::Immediate => memory[instruction_pointer + 1],
        };
        let address = match instruction.mode2 {
          ParameterMode::Position => memory[memory[instruction_pointer + 2] as usize],
          ParameterMode::Immediate => memory[instruction_pointer + 2],
        };
        if flag != 0 {
          instruction_pointer = address as usize;
        } else {
          instruction_pointer += 3;
        }
      }
      6 => {
        // jump-if-false: if the first parameter is zero,
        // it sets the instruction pointer to the value from the second parameter.
        // Otherwise, it does nothing
        let flag = match instruction.mode1 {
          ParameterMode::Position => memory[memory[instruction_pointer + 1] as usize],
          ParameterMode::Immediate => memory[instruction_pointer + 1],
        };
        let address = match instruction.mode2 {
          ParameterMode::Position => memory[memory[instruction_pointer + 2] as usize],
          ParameterMode::Immediate => memory[instruction_pointer + 2],
        };
        if flag == 0 {
          instruction_pointer = address as usize;
        } else {
          instruction_pointer += 3;
        }
      }
      7 => {
        // less than: if the first parameter is less than the second parameter,
        // it stores 1 in the position given by the third parameter.
        // Otherwise, it stores 0
        let left_op = memory[instruction_pointer + 1];
        let right_op = memory[instruction_pointer + 2];
        let dest = memory[instruction_pointer + 3] as usize;

        let left = match instruction.mode1 {
          ParameterMode::Position => memory[left_op as usize],
          ParameterMode::Immediate => left_op,
        };
        let right = match instruction.mode2 {
          ParameterMode::Position => memory[right_op as usize],
          ParameterMode::Immediate => right_op,
        };
        memory[dest] = match (left, right) {
          (left, right) if left < right => 1,
          _ => 0,
        };
        instruction_pointer += 4;
      }
      8 => {
        // equals: if the first parameter is equal to the second parameter,
        // it stores 1 in the position given by the third parameter.
        // Otherwise, it stores 0
        let left_op = memory[instruction_pointer + 1];
        let right_op = memory[instruction_pointer + 2];
        let dest = memory[instruction_pointer + 3] as usize;

        let left = match instruction.mode1 {
          ParameterMode::Position => memory[left_op as usize],
          ParameterMode::Immediate => left_op,
        };
        let right = match instruction.mode2 {
          ParameterMode::Position => memory[right_op as usize],
          ParameterMode::Immediate => right_op,
        };
        memory[dest] = match (left, right) {
          (left, right) if left == right => 1,
          _ => 0,
        };
        instruction_pointer += 4;
      }
      _ => {
        // error
        panic!(
          "invalid opcode {} at instruction_pointer {}",
          memory[instruction_pointer], instruction_pointer
        );
      }
    }
  }

  return (memory, output);
}

fn calc_str(expr: String, ins: Vec<i32>) -> String {
  let m = &mut parse(expr);
  let (mem, _out) = &calculate(m, ins);
  println!("out: {:?}", _out);
  return render(mem);
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn ex_1() {
    assert_eq!(
      Instruction {
        opcode: 2,
        mode1: ParameterMode::Position,
        mode2: ParameterMode::Immediate,
        mode3: ParameterMode::Position
      },
      Instruction::parse(1002)
    )
  }
  #[test]
  fn ex_2() {
    assert_eq!(
      "1101,100,-1,4,99",
      calc_str("1101,100,-1,4,0".to_string(), [].to_vec())
    );
  }

  #[test]
  fn input() {
    // read input to position 3 and then halt
    let mut mem = [3, 3, 99, 0].to_vec();
    let (rmem, rout) = calculate(&mut mem, [22].to_vec());
    println!("f, {:?}", rmem);
    assert_eq!([3, 3, 99, 22].to_vec(), *rmem);
  }

  #[test]
  fn output() {
    // write position 3 to output and then halt
    let mut mem = [4, 3, 99, 55].to_vec();
    let (rmem, routput) = calculate(&mut mem, [].to_vec());
    println!("o, {:?} {:?}", rmem, routput);
    assert_eq!([55].to_vec(), routput);
  }

  #[test]
  fn io() {
    // read input to position 5, output position 5, and then halt
    let mut mem = [3, 5, 4, 5, 99, 0].to_vec();
    let (rmem, rout) = calculate(&mut mem, [22].to_vec());
    println!("f, {:?} {:?}", rmem, rout);
    assert_eq!([3, 5, 4, 5, 99, 22].to_vec(), *rmem);
    assert_eq!([22].to_vec(), rout);
  }

  #[test]
  fn e2_1() {
    let (_m, out) = calculate(
      &mut [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(),
      [8].to_vec(),
    );
    assert_eq!([1].to_vec(), out);
    let (_, out) = calculate(
      &mut [3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec(),
      [0].to_vec(),
    );
    assert_eq!([0].to_vec(), out);
  }

  #[test]
  fn e2_2() {
    // this program uses an input instruction to ask for a single number.
    // The program will then output 999 if the input value is below 8,
    // output 1000 if the input value is equal to 8,
    // or output 1001 if the input value is greater than 8.

    let (_, out) = calculate(
      &mut [
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
      ]
      .to_vec(),
      [7].to_vec(),
    );
    assert_eq!([999].to_vec(), out);

    let (_, out) = calculate(
      &mut [
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
      ]
      .to_vec(),
      [8].to_vec(),
    );
    assert_eq!([1000].to_vec(), out);

    let (_, out) = calculate(
      &mut [
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
      ]
      .to_vec(),
      [9].to_vec(),
    );
    assert_eq!([1001].to_vec(), out);
  }

  fn day07_series(mem: Vec<i32>, phase_settings: Vec<i32>) -> i32 {
    let mut signal = 0;
    for setting in phase_settings {
      let mut amp = Intcode::new(mem.clone());
      let out = *amp
        .start([setting, signal].to_vec())
        .first()
        .expect("no output");
      println!("[ {}, {} ] {}", setting, signal, out);
      signal = out;
    }

    return signal;
  }

  // returns signal, phase settings
  fn day07_solver() -> (i32, Vec<i32>) {
    let mem = [
      3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 30, 47, 60, 81, 102, 183, 264, 345, 426, 99999, 3, 9,
      1002, 9, 5, 9, 4, 9, 99, 3, 9, 1002, 9, 5, 9, 1001, 9, 4, 9, 1002, 9, 4, 9, 4, 9, 99, 3, 9,
      101, 2, 9, 9, 1002, 9, 4, 9, 4, 9, 99, 3, 9, 1001, 9, 3, 9, 1002, 9, 2, 9, 101, 5, 9, 9,
      1002, 9, 2, 9, 4, 9, 99, 3, 9, 102, 4, 9, 9, 101, 4, 9, 9, 1002, 9, 3, 9, 101, 2, 9, 9, 4, 9,
      99, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9,
      101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9,
      4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 99, 3,
      9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9,
      2, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9,
      3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9,
      101, 2, 9, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9,
      4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
      1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 99, 3, 9, 102, 2,
      9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3,
      9, 1001, 9, 1, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9,
      9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 1002, 9, 2, 9, 4, 9,
      3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2,
      9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 2, 9, 4, 9,
      3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 99,
    ]
    .to_vec();
    let mut high = 0;
    let mut high_settings = [0, 0, 0, 0, 0].to_vec();
    // heh...
    for a in 0..5 {
      for b in 0..5 {
        if b == a {
          continue;
        }
        for c in 0..5 {
          if c == b || c == a {
            continue;
          }
          for d in 0..5 {
            if d == c || d == b || d == a {
              continue;
            }
            for e in 0..5 {
              if e == d || e == c || e == b || e == a {
                continue;
              }
              let settings = [a, b, c, d, e].to_vec();
              let signal = day07_series(mem.clone(), settings.clone());
              if signal > high {
                high = signal;
                high_settings = settings;
              }
            }
          }
        }
      }
    }

    return (high, high_settings);
  }

  #[test]
  fn OO() {
    let mut ampA = Intcode::new(
      [
        3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
      ]
      .to_vec(),
    );

    let out = ampA.start([1, 0].to_vec());
    println!("{:?}", out);
  }

  #[test]
  fn day7_ex_1() {
    assert_eq!(
      43210,
      day07_series(
        [3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0].to_vec(),
        [4, 3, 2, 1, 0].to_vec()
      )
    );
    assert_eq!(
      54321,
      day07_series(
        [
          3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
          99, 0, 0
        ]
        .to_vec(),
        [0, 1, 2, 3, 4].to_vec()
      )
    );
    assert_eq!(
      65210,
      day07_series(
        [
          3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
          33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
        ]
        .to_vec(),
        [1, 0, 4, 3, 2].to_vec()
      )
    );
  }

  #[test]
  fn day07_part1() {
    assert_eq!((116680, [3, 2, 4, 1, 0].to_vec()), day07_solver());
  }
}
