use std::io::{self, prelude::*};

fn main() {
  let stdin = io::stdin();
  let input_modules = stdin
    .lock()
    .lines()
    .map(|l| l.unwrap().parse::<i32>().ok().expect(""));

  let mut sum: i32 = 0;

  for n in input_modules {
    sum += get_launch_fuel(n);
    println!("{:?} {:?}", n, sum);
  }
}

fn get_required_fuel(mass: i32) -> i32 {
  // we depend on i32 rounding, which rounds the division down toward 0
  return (mass / 3) - 2;
}

fn get_marginal_fuel(fuel: i32) -> i32 {
  if fuel <= 0 {
    return 0;
  } else {
    return fuel + get_marginal_fuel(get_required_fuel(fuel));
  }
}

// account for the fuel for the mass plus its fuel, recursively
fn get_launch_fuel(mass: i32) -> i32 {
  let fuel = get_required_fuel(mass);
  return get_marginal_fuel(fuel);
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn ex_1() {
    assert_eq!(2, get_launch_fuel(14));
  }

  #[test]
  fn ex_2() {
    assert_eq!(966, get_launch_fuel(1969));
  }

  #[test]
  fn ex_3() {
    assert_eq!(50346, get_launch_fuel(100756));
  }
}
