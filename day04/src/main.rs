fn main() {
  let mut count = 0;
  for p in 236491..713787 {
    if is_candidate(p) {
      count = count + 1;
    }
  }
  println!("{}", count);
}

#[derive(Debug)]
struct Match {
  digit: char,
  len: i32,
}

fn is_candidate(pass: i32) -> bool {
  // It is a six-digit number.
  // skipped

  // The value is within the range given in your puzzle input.
  // skipped

  // Two adjacent digits are the same (like 22 in 122345).
  let mut last: Option<char> = None;

  let repeated = pass.to_string().chars().any(|c| {
    let repeat = last.map(|l| l.eq(&c)).unwrap_or(false);
    last = Some(c);
    // println!("rep {:?} {:?} {:?}", repeat, last, c);
    return repeat;
  });

  if !repeated {
    return false;
  }

  // Going from left to right, the digits never decrease; they only ever increase or stay the same
  // (like 111123 or 135679).
  last = None;
  let ascending = pass.to_string().chars().all(|c| {
    let asc = last.map(|l| l <= c).unwrap_or(true);
    last = Some(c);
    // println!("asc {:?} {:?} {:?}", asc, last, c);
    return asc;
  });

  if !ascending {
    return false;
  }

  // part 2: the two adjacent matching digits are not part of a larger group of matching digits.
  // [nb: i have no idea what this means, really]
  // i'm going to try splitting it into groups of repeated chars
  let mut matches = Vec::new();
  let mut last_char: Option<char> = None;
  let mut last_len = 0;
  for c in pass.to_string().chars() {
    // let mut last_group = groups.last();
    if last_char.eq(&Some(c)) {
      last_len = last_len + 1;
    } else {
      if (last_char.is_some()) {
        matches.push(Match {
          digit: last_char.expect("msg: &str"),
          len: last_len,
        })
      }
      last_char = Some(c);
      last_len = 1;
    }

    println!("l {:?} {:?} {:?} {:?}", last_char, last_len, c, matches);
  }
  if (last_char.is_some()) {
    matches.push(Match {
      digit: last_char.expect("msg: &str"),
      len: last_len,
    })
  }
  println!("m {:?} ", matches);

  // make sure at least one of the matches has exactly 2 len
  let adjacent_2_digits = matches.into_iter().any(|m| m.len == 2);
  if !adjacent_2_digits {
    return false;
  }

  return true;
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn ex_1() {
    assert!(is_candidate(111111));
    assert!(!is_candidate(223450));
    assert!(!is_candidate(123789));
  }
  #[test]
  fn ex_2() {
    assert!(is_candidate(112233));
    assert!(!is_candidate(123444));
    assert!(is_candidate(111122));
  }
}
