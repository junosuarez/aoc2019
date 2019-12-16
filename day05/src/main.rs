use std::convert::TryFrom;

fn main() {
  let o = calc_str(
    "3,225,1,225,6,6,1100,1,238,225,104,0,1102,68,5,225,1101,71,12,225,1,117,166,224,1001,224,-100,224,4,224,102,8,223,223,101,2,224,224,1,223,224,223,1001,66,36,224,101,-87,224,224,4,224,102,8,223,223,101,2,224,224,1,223,224,223,1101,26,51,225,1102,11,61,224,1001,224,-671,224,4,224,1002,223,8,223,1001,224,5,224,1,223,224,223,1101,59,77,224,101,-136,224,224,4,224,1002,223,8,223,1001,224,1,224,1,223,224,223,1101,11,36,225,1102,31,16,225,102,24,217,224,1001,224,-1656,224,4,224,102,8,223,223,1001,224,1,224,1,224,223,223,101,60,169,224,1001,224,-147,224,4,224,102,8,223,223,101,2,224,224,1,223,224,223,1102,38,69,225,1101,87,42,225,2,17,14,224,101,-355,224,224,4,224,102,8,223,223,1001,224,2,224,1,224,223,223,1002,113,89,224,101,-979,224,224,4,224,1002,223,8,223,1001,224,7,224,1,224,223,223,1102,69,59,225,4,223,99,0,0,0,677,0,0,0,0,0,0,0,0,0,0,0,1105,0,99999,1105,227,247,1105,1,99999,1005,227,99999,1005,0,256,1105,1,99999,1106,227,99999,1106,0,265,1105,1,99999,1006,0,99999,1006,227,274,1105,1,99999,1105,1,280,1105,1,99999,1,225,225,225,1101,294,0,0,105,1,0,1105,1,99999,1106,0,300,1105,1,99999,1,225,225,225,1101,314,0,0,106,0,0,1105,1,99999,7,677,677,224,1002,223,2,223,1006,224,329,1001,223,1,223,1007,226,226,224,1002,223,2,223,1006,224,344,1001,223,1,223,1108,226,677,224,102,2,223,223,1005,224,359,1001,223,1,223,1107,226,677,224,1002,223,2,223,1006,224,374,101,1,223,223,1107,677,226,224,1002,223,2,223,1006,224,389,101,1,223,223,7,226,677,224,1002,223,2,223,1005,224,404,101,1,223,223,1008,677,226,224,102,2,223,223,1005,224,419,101,1,223,223,1008,226,226,224,102,2,223,223,1006,224,434,101,1,223,223,107,226,226,224,1002,223,2,223,1005,224,449,1001,223,1,223,108,226,677,224,102,2,223,223,1005,224,464,101,1,223,223,1108,677,226,224,102,2,223,223,1005,224,479,101,1,223,223,1007,226,677,224,102,2,223,223,1006,224,494,101,1,223,223,107,677,677,224,102,2,223,223,1005,224,509,101,1,223,223,108,677,677,224,102,2,223,223,1006,224,524,1001,223,1,223,8,226,677,224,102,2,223,223,1005,224,539,101,1,223,223,107,677,226,224,102,2,223,223,1005,224,554,1001,223,1,223,8,226,226,224,102,2,223,223,1006,224,569,1001,223,1,223,7,677,226,224,1002,223,2,223,1005,224,584,1001,223,1,223,1108,226,226,224,102,2,223,223,1005,224,599,1001,223,1,223,1107,677,677,224,1002,223,2,223,1006,224,614,1001,223,1,223,1007,677,677,224,1002,223,2,223,1006,224,629,1001,223,1,223,108,226,226,224,102,2,223,223,1005,224,644,1001,223,1,223,8,677,226,224,1002,223,2,223,1005,224,659,1001,223,1,223,1008,677,677,224,1002,223,2,223,1006,224,674,1001,223,1,223,4,223,99,226".to_string(),
    [1].to_vec());
  println!("{}", o);
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

fn calculate(mut memory: Vec<i32>, input: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
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
  let (mem, _out) = &calculate(parse(expr), ins);
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
    let (rmem, rout) = calculate(mem, [22].to_vec());
    println!("f, {:?}", rmem);
    assert_eq!([3, 3, 99, 22].to_vec(), rmem);
  }

  #[test]
  fn output() {
    // write position 3 to output and then halt
    let mut mem = [4, 3, 99, 55].to_vec();
    let (rmem, routput) = calculate(mem, [].to_vec());
    println!("o, {:?} {:?}", rmem, routput);
    assert_eq!([55].to_vec(), routput);
  }

  #[test]
  fn io() {
    // read input to position 5, output position 5, and then halt
    let mem = [3, 5, 4, 5, 99, 0].to_vec();
    let (rmem, rout) = calculate(mem, [22].to_vec());
    println!("f, {:?} {:?}", rmem, rout);
    assert_eq!([3, 5, 4, 5, 99, 22].to_vec(), rmem);
    assert_eq!([22].to_vec(), rout);
  }
}
