use std::collections::HashMap;

use crate::binary_tree::TreeNode;

pub fn part_1(input: &str) -> String {
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

pub fn part_2(input: &str) -> String {
  let mut list_one: Vec<i32> = Vec::new();
  let mut list_two_appearances: HashMap<i32, i32> = HashMap::new();

  input.lines().for_each(|line| {
    let values = line.split_whitespace().collect::<Vec<&str>>();
    let value_one = values[0].parse::<i32>().unwrap();
    let value_two = values[1].parse::<i32>().unwrap();

    list_one.push(value_one);
    *list_two_appearances.entry(value_two).or_insert(0) += 1;
  });

  let mut similarity_score = 0;

  list_one.iter().for_each(|value| {
    if let Some(appearances) = list_two_appearances.get(value) {
      similarity_score += value * appearances;
    }
  });

  similarity_score.to_string()
}
