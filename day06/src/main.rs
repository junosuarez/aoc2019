use std::collections::HashMap;

fn main() {
  println!("Hello, world!");
}

#[derive(Clone, Debug)]
struct Node {
  name: String,
  children: Vec<Node>,
}

impl Node {
  fn of(name: &str) -> Node {
    Node {
      name: name.to_string(),
      children: Vec::new(),
    }
  }
}

fn find(from: Node, name: &String) -> Option<Node> {
  if from.name.eq(name) {
    return Some(from);
  }
  // depth-first
  for child in from.children {
    if child.name.eq(name) {
      return Some(child);
    } else {
      let found = find(child, name);
      if found.is_some() {
        return found;
      }
    }
  }
  return None;
}

fn parse(data: Vec<&str>) -> Node {
  let mut nodes = HashMap::new();
  let mut root: Option<Node> = None;

  data.iter().for_each(|e| {
    let s = e.to_string().clone();
    let parts: Vec<&str> = s.split(")").collect();
    // println!("{:?} p", parts);

    let from = parts[0].to_string();
    let to = parts[1].to_string();

    let to_n = nodes
      .entry(to.clone())
      .or_insert_with(|| Node::of(&to))
      .to_owned();
    let from_n = nodes.entry(from.clone()).or_insert_with(|| Node::of(&from));

    from_n.children.push(to_n);

    if root.is_none() {
      root = Some(from_n.to_owned());
    }

    println!("x {:?}", nodes);
  });

  return root.expect("root");
}

fn checksum(tree: Node) -> i32 {
  0
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn ex_1() {
    let data = [
      "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
    ]
    .to_vec();
    println!("{:?}", parse(data.clone()));
    // assert_eq!(42, checksum(parse(data.clone())));
  }
  #[test]
  fn ex_2() {
    unimplemented!();
  }
  #[test]
  fn tfind() {
    let mut tree = Node::of("a");
    let mut b = Node::of("b");
    b.children.push(Node::of("d"));
    tree.children.push(b);
    tree.children.push(Node::of("c"));
    assert_eq!(
      Some("d".to_string()),
      find(tree, &"d".to_string()).map(|n| n.to_owned().name)
    );
  }
}
