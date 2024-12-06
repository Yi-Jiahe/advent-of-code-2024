use std::collections::HashMap;

use crate::binary_tree::TreeNode;

pub fn part_1(input: String) -> String {
  let mut list_one: Option<TreeNode<i32>> = None;
  let mut list_two: Option<TreeNode<i32>> = None;

  input.lines().for_each(|line| {
    let values = line.split_whitespace().collect::<Vec<&str>>();
    let value_one = values[0].parse::<i32>().unwrap();
    let value_two = values[1].parse::<i32>().unwrap();

    if let Some(node) = list_one.as_mut() {
      node.insert(value_one);
    } else {
      list_one = Some(TreeNode::new(value_one));
    }
    if let Some(node) = list_two.as_mut() {
      node.insert(value_two);
    } else {
      list_two = Some(TreeNode::new(value_two));
    }
  });

  let mut total_distance = 0;

  let iter_one = list_one.as_ref().unwrap().in_order_iter();
  let iter_two = list_two.as_ref().unwrap().in_order_iter();

  for (one, two) in iter_one.zip(iter_two) {
    let distance = i32::abs(one - two);

    total_distance += distance;
  }

  total_distance.to_string()
}
