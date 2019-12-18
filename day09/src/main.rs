use std::collections::VecDeque;
use std::convert::TryFrom;

fn main() {
  let mut x = Intcode::new([104, 23, 99].to_vec());
  println!("{:?}", x.run([].to_vec()));
}

#[derive(Debug, PartialEq)]
enum Status<T> {
  AwaitingInput(T),
  Halted(T),
}
impl<T> Status<T> {
  fn unwrap(&self) -> &T {
    match self {
      Status::AwaitingInput(out) => out,
      Status::Halted(out) => out,
    }
  }
}

struct Intcode {
  memory: Vec<i64>,
  codelen: usize,
  status: Status<Vec<i64>>,
  instruction_pointer: usize,
  relative_base: usize,
  runs: i64,
  tag: String,
}
impl Intcode {
  fn new(mut memory: Vec<i64>) -> Self {
    let codelen = memory.len();
    // start with 2KiB memory
    memory.resize(2 * 1024, 0);

    Intcode {
      memory: memory,
      codelen: codelen,
      status: Status::AwaitingInput([].to_vec()),
      instruction_pointer: 0,
      relative_base: 0,
      runs: 0,
      tag: "".to_string(),
    }
  }
  fn run(&mut self, input: Vec<i64>) -> &Status<Vec<i64>> {
    self.runs += 1;
    println!(
      "{} Run {} with {:?} from {}",
      self.tag, self.runs, input, self.instruction_pointer
    );
    self.status = self.calculate(input);
    println!(
      "Ending at {}: {:?} \n",
      self.instruction_pointer, self.status
    );
    return &self.status;
  }

  fn tag(&mut self, name: &str) {
    self.tag += name;
  }

  fn dump(&self) -> Vec<i64> {
    return self.memory[0..self.codelen].to_vec().clone();
  }

  fn calculate(&mut self, input: Vec<i64>) -> Status<Vec<i64>> {
    let mut ins = input.into_iter();
    let mut output: Vec<i64> = Vec::new();

    loop {
      let instruction = Instruction::parse(self.memory[self.instruction_pointer]);
      if instruction.opcode == 99 {
        // halt immediately
        break;
      }
      // println!("op {:?}", instruction);
      match instruction.opcode {
        1 => {
          // add
          let left_op = self.memory[self.instruction_pointer + 1];
          let right_op = self.memory[self.instruction_pointer + 2];
          let dest = self.memory[self.instruction_pointer + 3] as usize;
          // println!("{:?} ADD {} {} to {}", instruction, left_op, right_op, dest);
          // println!("before: {}: {}", dest, memory[dest]);
          let left = match instruction.mode1 {
            ParameterMode::Position => self.memory[left_op as usize],
            ParameterMode::Immediate => left_op,
            ParameterMode::Relative => self.memory[left_op as usize + self.relative_base],
          };
          let right = match instruction.mode2 {
            ParameterMode::Position => self.memory[right_op as usize],
            ParameterMode::Immediate => right_op,
            ParameterMode::Relative => self.memory[right_op as usize + self.relative_base],
          };
          self.memory[dest] = left + right;
          // println!("after: {}: {}", dest, memory[dest]);
          self.instruction_pointer += 4;
        }
        2 => {
          // multiply
          let left_op = self.memory[self.instruction_pointer + 1];
          let right_op = self.memory[self.instruction_pointer + 2];
          let dest = self.memory[self.instruction_pointer + 3] as usize;
          // println!("{:?} MUL {} {} to {}", instruction, left_op, right_op, dest);
          // println!("before: {}: {}", dest, memory[dest]);
          let left = match instruction.mode1 {
            ParameterMode::Position => self.memory[left_op as usize],
            ParameterMode::Immediate => left_op,
            ParameterMode::Relative => self.memory[left_op as usize + self.relative_base],
          };
          let right = match instruction.mode2 {
            ParameterMode::Position => self.memory[right_op as usize],
            ParameterMode::Immediate => right_op,
            ParameterMode::Relative => self.memory[right_op as usize + self.relative_base],
          };
          self.memory[dest] = left * right;
          // println!("after: {}: {}", dest, memory[dest]);
          self.instruction_pointer += 4;
        }
        3 => {
          // input
          let op = self.memory[self.instruction_pointer + 1] as usize;
          let dest = match instruction.mode1 {
            ParameterMode::Position => self.memory[op as usize] as usize,
            ParameterMode::Immediate => op,
            ParameterMode::Relative => self.memory[op as usize + self.relative_base] as usize,
          };

          // for now, inputs are faked as `1`
          let input = ins.next();

          if input.is_none() {
            // exhausted supplied input, yield awaiting more
            // don't advance the instruction pointer, so that when we re-enter,
            // it will process this instruction again and attempt to set the destination address
            return Status::AwaitingInput(output);
          } else {
            println!("IN: {:?}", input);
            self.memory[dest] = input.expect("couldnt read input");
            self.instruction_pointer += 2;
          }
        }
        4 => {
          // output
          let src = self.memory[self.instruction_pointer + 1];
          let out = match instruction.mode1 {
            ParameterMode::Position => self.memory[src as usize],
            ParameterMode::Immediate => src,
            ParameterMode::Relative => {
              println!(
                "{}, {}, {}",
                src,
                self.relative_base,
                (src + self.relative_base as i64) as usize
              );
              // TODO: factor out parameter mode dereferencing to one place, and fix the overflow math
              self.memory[(src + self.relative_base as i64) as usize]
            }
          };
          println!("OUT: {:?}", out);
          output.push(out);

          self.instruction_pointer += 2;
          // return (memory, output);
        }
        5 => {
          // jump-if-true: if the first parameter is non-zero,
          // it sets the instruction pointer to the value from the second parameter.
          // Otherwise, it does nothing.
          let flag = match instruction.mode1 {
            ParameterMode::Position => {
              self.memory[self.memory[self.instruction_pointer + 1] as usize]
            }
            ParameterMode::Immediate => self.memory[self.instruction_pointer + 1],
            ParameterMode::Relative => {
              self.memory[self.memory[self.instruction_pointer + 1] as usize + self.relative_base]
            }
          };
          let address = match instruction.mode2 {
            ParameterMode::Position => {
              self.memory[self.memory[self.instruction_pointer + 2] as usize]
            }
            ParameterMode::Immediate => self.memory[self.instruction_pointer + 2],
            ParameterMode::Relative => {
              self.memory[self.memory[self.instruction_pointer + 2] as usize + self.relative_base]
            }
          };
          if flag != 0 {
            self.instruction_pointer = address as usize;
          } else {
            self.instruction_pointer += 3;
          }
        }
        6 => {
          // jump-if-false: if the first parameter is zero,
          // it sets the instruction pointer to the value from the second parameter.
          // Otherwise, it does nothing
          let flag = match instruction.mode1 {
            ParameterMode::Position => {
              self.memory[self.memory[self.instruction_pointer + 1] as usize]
            }
            ParameterMode::Immediate => self.memory[self.instruction_pointer + 1],
            ParameterMode::Relative => {
              self.memory[self.memory[self.instruction_pointer + 1] as usize + self.relative_base]
            }
          };
          let address = match instruction.mode2 {
            ParameterMode::Position => {
              self.memory[self.memory[self.instruction_pointer + 2] as usize]
            }
            ParameterMode::Immediate => self.memory[self.instruction_pointer + 2],
            ParameterMode::Relative => {
              self.memory[self.memory[self.instruction_pointer + 2] as usize + self.relative_base]
            }
          };
          if flag == 0 {
            self.instruction_pointer = address as usize;
          } else {
            self.instruction_pointer += 3;
          }
        }
        7 => {
          // less than: if the first parameter is less than the second parameter,
          // it stores 1 in the position given by the third parameter.
          // Otherwise, it stores 0
          let left_op = self.memory[self.instruction_pointer + 1];
          let right_op = self.memory[self.instruction_pointer + 2];
          let dest = self.memory[self.instruction_pointer + 3] as usize;

          let left = match instruction.mode1 {
            ParameterMode::Position => self.memory[left_op as usize],
            ParameterMode::Immediate => left_op,
            ParameterMode::Relative => self.memory[left_op as usize + self.relative_base],
          };
          let right = match instruction.mode2 {
            ParameterMode::Position => self.memory[right_op as usize],
            ParameterMode::Immediate => right_op,
            ParameterMode::Relative => self.memory[right_op as usize + self.relative_base],
          };
          self.memory[dest] = match (left, right) {
            (left, right) if left < right => 1,
            _ => 0,
          };
          self.instruction_pointer += 4;
        }
        8 => {
          // equals: if the first parameter is equal to the second parameter,
          // it stores 1 in the position given by the third parameter.
          // Otherwise, it stores 0
          let left_op = self.memory[self.instruction_pointer + 1];
          let right_op = self.memory[self.instruction_pointer + 2];
          let dest = self.memory[self.instruction_pointer + 3] as usize;

          let left = match instruction.mode1 {
            ParameterMode::Position => self.memory[left_op as usize],
            ParameterMode::Immediate => left_op,
            ParameterMode::Relative => self.memory[left_op as usize + self.relative_base],
          };
          let right = match instruction.mode2 {
            ParameterMode::Position => self.memory[right_op as usize],
            ParameterMode::Immediate => right_op,
            ParameterMode::Relative => self.memory[right_op as usize + self.relative_base],
          };
          self.memory[dest] = match (left, right) {
            (left, right) if left == right => 1,
            _ => 0,
          };
          self.instruction_pointer += 4;
        }
        9 => {
          // shift relative base
          let op = self.memory[self.instruction_pointer + 1];
          let shift = match instruction.mode1 {
            ParameterMode::Position => self.memory[op as usize],
            ParameterMode::Immediate => op,
            ParameterMode::Relative => self.memory[op as usize + self.relative_base],
          };

          self.relative_base += shift as usize;

          println!("Shifted relative base {} to {}", shift, self.relative_base);

          self.instruction_pointer += 2;
        }
        _ => {
          // error
          panic!(
            "invalid opcode {} at instruction_pointer {}",
            self.memory[self.instruction_pointer], self.instruction_pointer
          );
        }
      }
    }

    return Status::Halted(output);
  }
}

fn parse(expr: String) -> Vec<i64> {
  let memory: Vec<i64> = expr
    .split(",")
    .map(|s| s.parse::<i64>().expect("x"))
    .collect();
  return memory;
}

fn render(memory: &Vec<i64>) -> String {
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
  Relative = 2,
}
fn parse_param_mode(c: char) -> ParameterMode {
  match c {
    '0' => ParameterMode::Position,
    '1' => ParameterMode::Immediate,
    '2' => ParameterMode::Relative,
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
//   Immediate(i64),
//   Position(usize),
// }

impl Instruction {
  fn parse(code: i64) -> Instruction {
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

fn calc_str(expr: String, ins: Vec<i64>) -> String {
  let m = parse(expr);
  let mut c = Intcode::new(m);
  let out = c.run(ins);
  println!("out: {:?}", out);
  return render(&c.dump());
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
    let mut c = Intcode::new([3, 3, 99, 0].to_vec());
    c.run([22].to_vec());
    assert_eq!([3, 3, 99, 22].to_vec(), c.dump());
  }

  #[test]
  fn output() {
    // write position 3 to output and then halt
    let mut c = Intcode::new([4, 3, 99, 55].to_vec());
    let out = c.run([].to_vec());
    assert_eq!(&Status::Halted([55].to_vec()), out);
  }

  #[test]
  fn io() {
    // read input to position 5, output position 5, and then halt
    let mut c = Intcode::new([3, 5, 4, 5, 99, 0].to_vec());
    let out = c.run([22].to_vec());
    assert_eq!(&Status::Halted([22].to_vec()), out);
    assert_eq!([3, 5, 4, 5, 99, 22].to_vec(), c.dump());
  }

  #[test]
  fn e2_1() {
    let mut c = Intcode::new([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec());
    let out = c.run([8].to_vec());
    assert_eq!(&Status::Halted([1].to_vec()), out);

    let mut c = Intcode::new([3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8].to_vec());
    let out = c.run([0].to_vec());
    assert_eq!(&Status::Halted([0].to_vec()), out);
  }

  #[test]
  fn e2_2() {
    // this program uses an input instruction to ask for a single number.
    // The program will then output 999 if the input value is below 8,
    // output 1000 if the input value is equal to 8,
    // or output 1001 if the input value is greater than 8.

    let mut c = Intcode::new(
      [
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
      ]
      .to_vec(),
    );
    let out = c.run([7].to_vec());

    assert_eq!(&Status::Halted([999].to_vec()), out);

    let mut c = Intcode::new(
      [
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
      ]
      .to_vec(),
    );
    let out = c.run([8].to_vec());
    assert_eq!(&Status::Halted([1000].to_vec()), out);

    let mut c = Intcode::new(
      [
        3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0, 0,
        1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4, 20,
        1105, 1, 46, 98, 99,
      ]
      .to_vec(),
    );
    let out = c.run([9].to_vec());
    assert_eq!(&Status::Halted([1001].to_vec()), out);
  }

  fn day07_series(mem: Vec<i64>, phase_settings: Vec<i64>) -> i64 {
    let mut signal = 0;
    for setting in phase_settings {
      let mut amp = Intcode::new(mem.clone());
      let out = *amp
        .run([setting, signal].to_vec())
        .unwrap()
        .first()
        .expect("no output");
      println!("[ {}, {} ] {}", setting, signal, out);
      signal = out;
    }

    return signal;
  }

  fn day07_feedback_loop(mem: Vec<i64>, phase_settings: Vec<i64>) -> i64 {
    let mut signal = 0;
    let mut amps = VecDeque::new();
    let mut name = 0;

    for setting in phase_settings {
      let mut amp = Intcode::new(mem.clone());
      amp.tag(&format!("{}) ", name));
      name += 1;
      // initialize with setting, status will be AwaitingInput
      amp.run([setting].to_vec());
      amps.push_back(amp);
    }

    while let Some(mut amp) = amps.pop_front() {
      match amp.run([signal].to_vec()) {
        Status::AwaitingInput(out) => {
          signal = *out.first().expect("no output");
          amps.push_back(amp);
        }
        Status::Halted(out) => {
          signal = *out.first().expect("no output");
        }
      }
    }

    return signal;
  }

  // returns signal, phase settings
  fn day07_solver() -> (i64, Vec<i64>) {
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

  // returns signal, phase settings
  fn day07_feedback_solver() -> (i64, Vec<i64>) {
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
    for a in 5..10 {
      for b in 5..10 {
        if b == a {
          continue;
        }
        for c in 5..10 {
          if c == b || c == a {
            continue;
          }
          for d in 5..10 {
            if d == c || d == b || d == a {
              continue;
            }
            for e in 5..10 {
              if e == d || e == c || e == b || e == a {
                continue;
              }
              let settings = [a, b, c, d, e].to_vec();
              let signal = day07_feedback_loop(mem.clone(), settings.clone());
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

    let out = ampA.run([1, 0].to_vec());
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

  #[test]
  fn reentrant() {
    let mut c = Intcode::new(
      [
        // reads a number, prints it, reads a number, prints it, halts
        3, 10, 4, 10, 3, 10, 4, 10, 99, 0, -1,
      ]
      .to_vec(),
    );

    // start with no input
    let out1 = c.run([].to_vec());
    assert_eq!(&Status::AwaitingInput([].to_vec()), out1);

    // resume with 100, it echos it and awaits again
    let out2 = c.run([100].to_vec());
    assert_eq!(&Status::AwaitingInput([100].to_vec()), out2);

    // resume with 23, it echos it and halts
    let out3 = c.run([23].to_vec());
    assert_eq!(&Status::Halted([23].to_vec()), out3);
  }

  #[test]
  fn day07_feedback_ex1() {
    assert_eq!(
      139629729,
      day07_feedback_loop(
        [
          3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1, 28,
          1005, 28, 6, 99, 0, 0, 5
        ]
        .to_vec(),
        [9, 8, 7, 6, 5].to_vec()
      )
    )
  }

  #[test]
  fn day07_feedback_part2() {
    assert_eq!(
      (89603079, [7, 6, 5, 8, 9].to_vec()),
      day07_feedback_solver()
    );
  }

  #[test]
  fn day9_ex1() {
    let code = [
      109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
    ]
    .to_vec();
    let mut c = Intcode::new(code.clone());
    let out = c.run([].to_vec());
    // it's a quine!
    assert_eq!(&Status::Halted(code), out);
  }

  #[test]
  fn day9_ex2() {
    let code = [1102, 34915192, 34915192, 7, 4, 7, 99, 0].to_vec();
    let mut c = Intcode::new(code.clone());
    let out = c.run([].to_vec());
    assert_eq!(&Status::Halted([1219070632396864].to_vec()), out);
  }
  #[test]
  fn day9_ex3() {
    let code = [104, 1125899906842624, 99].to_vec();
    let mut c = Intcode::new(code.clone());
    let out = c.run([].to_vec());
    assert_eq!(&Status::Halted([1125899906842624].to_vec()), out);
  }
  #[test]
  fn day9_part1() {
    let code = [
      1102, 34463338, 34463338, 63, 1007, 63, 34463338, 63, 1005, 63, 53, 1102, 3, 1, 1000, 109,
      988, 209, 12, 9, 1000, 209, 6, 209, 3, 203, 0, 1008, 1000, 1, 63, 1005, 63, 65, 1008, 1000,
      2, 63, 1005, 63, 904, 1008, 1000, 0, 63, 1005, 63, 58, 4, 25, 104, 0, 99, 4, 0, 104, 0, 99,
      4, 17, 104, 0, 99, 0, 0, 1101, 0, 34, 1006, 1101, 0, 689, 1022, 1102, 27, 1, 1018, 1102, 1,
      38, 1010, 1102, 1, 31, 1012, 1101, 20, 0, 1015, 1102, 1, 791, 1026, 1102, 0, 1, 1020, 1101,
      24, 0, 1000, 1101, 0, 682, 1023, 1101, 788, 0, 1027, 1101, 0, 37, 1005, 1102, 21, 1, 1011,
      1102, 1, 28, 1002, 1101, 0, 529, 1024, 1101, 39, 0, 1017, 1102, 30, 1, 1013, 1101, 0, 23,
      1003, 1102, 524, 1, 1025, 1101, 32, 0, 1007, 1102, 25, 1, 1008, 1101, 29, 0, 1001, 1101, 33,
      0, 1016, 1101, 410, 0, 1029, 1101, 419, 0, 1028, 1101, 22, 0, 1014, 1102, 26, 1, 1019, 1102,
      1, 35, 1009, 1102, 36, 1, 1004, 1102, 1, 1, 1021, 109, 11, 2107, 22, -8, 63, 1005, 63, 199,
      4, 187, 1106, 0, 203, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 2, 21108, 40, 40, -2, 1005,
      1011, 221, 4, 209, 1106, 0, 225, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 13, 21102, 41, 1, -7,
      1008, 1019, 41, 63, 1005, 63, 251, 4, 231, 1001, 64, 1, 64, 1106, 0, 251, 1002, 64, 2, 64,
      109, -19, 1202, 1, 1, 63, 1008, 63, 26, 63, 1005, 63, 271, 1105, 1, 277, 4, 257, 1001, 64, 1,
      64, 1002, 64, 2, 64, 109, 7, 2101, 0, -6, 63, 1008, 63, 24, 63, 1005, 63, 297, 1106, 0, 303,
      4, 283, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 7, 1205, -1, 315, 1105, 1, 321, 4, 309, 1001,
      64, 1, 64, 1002, 64, 2, 64, 109, -11, 21107, 42, 41, 0, 1005, 1010, 341, 1001, 64, 1, 64,
      1106, 0, 343, 4, 327, 1002, 64, 2, 64, 109, -8, 1207, 6, 24, 63, 1005, 63, 363, 1001, 64, 1,
      64, 1106, 0, 365, 4, 349, 1002, 64, 2, 64, 109, 11, 1206, 8, 381, 1001, 64, 1, 64, 1106, 0,
      383, 4, 371, 1002, 64, 2, 64, 109, 4, 1205, 4, 401, 4, 389, 1001, 64, 1, 64, 1105, 1, 401,
      1002, 64, 2, 64, 109, 14, 2106, 0, -3, 4, 407, 1001, 64, 1, 64, 1106, 0, 419, 1002, 64, 2,
      64, 109, -33, 1202, 3, 1, 63, 1008, 63, 29, 63, 1005, 63, 445, 4, 425, 1001, 64, 1, 64, 1105,
      1, 445, 1002, 64, 2, 64, 109, -5, 2102, 1, 7, 63, 1008, 63, 25, 63, 1005, 63, 465, 1105, 1,
      471, 4, 451, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 11, 21107, 43, 44, 7, 1005, 1011, 489, 4,
      477, 1105, 1, 493, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -3, 1208, 8, 35, 63, 1005, 63, 511,
      4, 499, 1105, 1, 515, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 25, 2105, 1, -2, 4, 521, 1106,
      0, 533, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -8, 21108, 44, 47, -8, 1005, 1010, 549, 1106,
      0, 555, 4, 539, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -19, 1207, 7, 35, 63, 1005, 63, 577,
      4, 561, 1001, 64, 1, 64, 1106, 0, 577, 1002, 64, 2, 64, 109, 2, 2108, 32, 0, 63, 1005, 63,
      597, 1001, 64, 1, 64, 1106, 0, 599, 4, 583, 1002, 64, 2, 64, 109, 13, 2101, 0, -7, 63, 1008,
      63, 32, 63, 1005, 63, 625, 4, 605, 1001, 64, 1, 64, 1105, 1, 625, 1002, 64, 2, 64, 109, -13,
      2107, 24, 2, 63, 1005, 63, 645, 1001, 64, 1, 64, 1106, 0, 647, 4, 631, 1002, 64, 2, 64, 109,
      18, 21101, 45, 0, -4, 1008, 1015, 43, 63, 1005, 63, 671, 1001, 64, 1, 64, 1105, 1, 673, 4,
      653, 1002, 64, 2, 64, 109, -6, 2105, 1, 10, 1001, 64, 1, 64, 1105, 1, 691, 4, 679, 1002, 64,
      2, 64, 109, 1, 1208, -6, 23, 63, 1005, 63, 707, 1105, 1, 713, 4, 697, 1001, 64, 1, 64, 1002,
      64, 2, 64, 109, -2, 1206, 8, 731, 4, 719, 1001, 64, 1, 64, 1106, 0, 731, 1002, 64, 2, 64,
      109, -7, 21102, 46, 1, 5, 1008, 1010, 43, 63, 1005, 63, 751, 1106, 0, 757, 4, 737, 1001, 64,
      1, 64, 1002, 64, 2, 64, 109, -9, 2108, 24, 4, 63, 1005, 63, 779, 4, 763, 1001, 64, 1, 64,
      1106, 0, 779, 1002, 64, 2, 64, 109, 38, 2106, 0, -7, 1106, 0, 797, 4, 785, 1001, 64, 1, 64,
      1002, 64, 2, 64, 109, -27, 2102, 1, -6, 63, 1008, 63, 29, 63, 1005, 63, 819, 4, 803, 1105, 1,
      823, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 1, 21101, 47, 0, 7, 1008, 1015, 47, 63, 1005, 63,
      845, 4, 829, 1105, 1, 849, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -11, 1201, 5, 0, 63, 1008,
      63, 31, 63, 1005, 63, 869, 1106, 0, 875, 4, 855, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 5,
      1201, 4, 0, 63, 1008, 63, 34, 63, 1005, 63, 901, 4, 881, 1001, 64, 1, 64, 1105, 1, 901, 4,
      64, 99, 21102, 27, 1, 1, 21101, 915, 0, 0, 1105, 1, 922, 21201, 1, 58905, 1, 204, 1, 99, 109,
      3, 1207, -2, 3, 63, 1005, 63, 964, 21201, -2, -1, 1, 21101, 0, 942, 0, 1106, 0, 922, 22101,
      0, 1, -1, 21201, -2, -3, 1, 21102, 1, 957, 0, 1106, 0, 922, 22201, 1, -1, -2, 1106, 0, 968,
      22102, 1, -2, -2, 109, -3, 2106, 0, 0,
    ]
    .to_vec();
    let mut c = Intcode::new(code);
    let out = c.run([1].to_vec());
    assert_eq!(&Status::Halted([].to_vec()), out);
  }
}
