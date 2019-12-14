use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();

  let arg1 = args.get(1).expect("Missing input");
  let input: i32 = arg1.parse().unwrap();

  println!("{}", get_required_fuel(input));
}

fn get_required_fuel(mass: i32) -> i32 {
  // we depend on i32 rounding, which rounds the division down toward 0
  return (mass / 3) - 2;
}
