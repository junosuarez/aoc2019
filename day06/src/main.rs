use indextree::Arena;
use indextree::Node;
use std::collections::HashMap;
use std::num::NonZeroUsize;

fn main() {
  let arena: &Arena<String> = &mut Arena::new();
}

// #[derive(Clone, Debug)]
// struct N<'n> {
//   name: String,
//   children: Vec<&'n N<'n>>,
// }

// impl<'n> N<'n> {
//   fn of(name: &str) -> N<'n> {
//     N {
//       name: name.to_string(),
//       children: Vec::new(),
//     }
//   }
// }

// #[derive(Clone, Debug)]
// struct Node {
//   name: String,
//   children: Vec<Node>,
// }

// impl Node {
//   fn of(name: &str) -> Node {
//     Node {
//       name: name.to_string(),
//       children: Vec::new(),
//     }
//   }
// }

// fn find(from: Node, name: &String) -> Option<Node> {
//   if from.name.eq(name) {
//     return Some(from);
//   }
//   // depth-first
//   for child in from.children {
//     if child.name.eq(name) {
//       return Some(child);
//     } else {
//       let found = find(child, name);
//       if found.is_some() {
//         return found;
//       }
//     }
//   }
//   return None;
// }
//
// fn find<'a, 'b>(from: &'a mut N<'a>, name: &'b String) -> Option<&'a mut N<'a>> {
//   if from.name.eq(name) {
//     return Some(from);
//   }
//   // depth-first
//   for child in from.children.iter_mut() {
//     if child.name.eq(name) {
//       return Some(child);
//     } else {
//       let found = find(child, name);
//       if found.is_some() {
//         return found;
//       }
//     }
//   }
//   return None;
// }

// fn parse(data: Vec<&str>) -> Node {
//   let mut root: Option<String> = None;
//   let mut tree = Node::of("__root");
//   let mut nodes: HashMap<String, Node> = HashMap::new();

//   data.iter().for_each(|edge| {
//     let s = edge.to_string().clone();
//     let parts: Vec<&str> = s.split(")").collect();
//     // println!("{:?} p", parts);

//     let from = parts[0].to_string();
//     let to = parts[1].to_string();

//     let to_n = Node::of(&to);
//     tree.children.push(to_n);
//     // nodes.entry(to).or_insert_with(|| Node::of(&to)).to_owned();
//     // nodes.insert(to.clone(), Node::of(&to));

//     // let from_n = nodes.entry(from.clone()).or_insert_with(|| Node::of(&from));
//     // let from_n: &mut Node;
//     // if nodes.contains_key(&from) {
//     //   from_n = nodes.get_mut(&from).expect("");
//     // } else {
//     //   nodes.insert(from.clone(), Node::of(&from));
//     //   from_n = nodes.get_mut(&from).expect("");
//     // }

//     let mut from_n = find(tree, &from)
//       .or_else(|| {
//         let n = Node::of(&from);
//         tree.children.push(n);
//         return Some(n);
//       })
//       .unwrap();

//     from_n.children.push(to_n);

//     if root.is_none() {
//       root = Some(from.clone());
//     }

//     // println!("x {:?}", nodes);
//     // println!("f: {:?}", from_n);
//   });

//   println!("x {:?} {:?}", nodes, root);
//   return nodes.get(&root.expect("root")).expect("r2").to_owned();
// }

// fn checksum(tree: Node) -> i32 {
//   0
// }

fn parse(data: Vec<&str>, arena: &mut Arena<String>) {
  // let mut map = HashMap::new();
  // let arena = &mut Arena::new();

  // data.iter().for_each(|edge| {
  //   let s = edge.to_string().clone();

  //   // parse parts from string
  //   let parts: Vec<&str> = s.split(")").collect();
  //   let from = parts[0].to_string();
  //   let to = parts[1].to_string();

  //   // a very naive way of checking if the node exists. improvement would be to maintain a map
  //   // from node name to indexed nodeid
  //   let from_id_opt = map.get(&from);
  //   let mut from_id;
  //   if map.contains_key(&from) {
  //     from_id = map.get(&from).expect("");
  //   } else {
  //     let n = arena.new_node(from);
  //     from_id = &n;
  //     map.insert(&from, n);
  //   }

  //   // assumption: to is always new. otherwise, perform the same search or else as above
  //   let to_id = arena.new_node(to);
  //   map.insert(&to, to_id);
  //   from_id.append(to_id, arena);
  // });

  // println!("x {:?}, {:?}", arena, map);
}

// fn checksum(tree: &N) -> i32 {
//   0
// }

#[cfg(test)]
mod tests {
  use super::*;
  use indextree::NodeId;
  #[test]
  fn ex_1() {
    let arena: &mut Arena<String> = &mut Arena::new();
    let data = [
      "COM)B", "B)C", "C)D", "D)E", "E)F", "B)G", "G)H", "D)I", "E)J", "J)K", "K)L",
    ]
    .to_vec();
    println!("{:?}", parse(data.clone(), arena));
    // println!("{:?}", parse(&mut arena, data.clone()));
    // assert_eq!(42, checksum(parse(data.clone())));
  }
  #[test]
  fn ex_2() {
    let arena: &mut Arena<String> = &mut Arena::new();
    let n1 = arena.new_node("n1".to_string());
    let n2 = arena.new_node("n2".to_string());
    let n3 = arena.new_node("n3".to_string());
    n1.append(n2, arena);
    n2.append(n3, arena);

    println!("{:?}", arena);
    let x = arena.iter().position(|n| n.get().eq(&"n2".to_string()));
    x.wrapping_add(1);
    // .map(|i| i + 1);
    println!("{:?}", x);
    // let n = NodeId {
    //   index1: x.and_then(|i| NonZeroUsize::new(i + 1)).unwrap(),
    // };
    let z = NodeId::from_non_zero_usize(x.unwrap());
    println!("{:?} {:?}", z, z.ancestors(arena).next());
  }
  // #[test]
  // fn tfind() {
  //   let mut tree = N::of("a");
  //   let mut b = N::of("b");
  //   b.children.push(&N::of("d"));
  //   tree.children.push(&b);
  //   tree.children.push(&N::of("c"));
  //   assert_eq!(
  //     Some("d".to_string()),
  //     find(&mut tree, &"d".to_string()).map(|n| n.to_owned().name)
  //   );
  // }
}
