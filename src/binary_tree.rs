#[derive(Debug)]
pub struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, value: T)
    where
        T: PartialOrd,
    {
        if value < self.value {
            if let Some(ref mut left) = self.left {
                left.insert(value);
            } else {
                self.left = Some(Box::new(TreeNode::new(value)));
            }
        } else {
            if let Some(ref mut right) = self.right {
                right.insert(value);
            } else {
                self.right = Some(Box::new(TreeNode::new(value)));
            }
        }
    }

    pub fn in_order_iter(&self) -> InOrderIterator<T> {
      InOrderIterator::new(&self)

    }
}

pub struct InOrderIterator<'a, T> {
  current: Option<&'a TreeNode<T>>,
  stack: Vec<&'a TreeNode<T>>,
}

impl<'a, T> InOrderIterator<'a, T> {
  fn new(root: &'a TreeNode<T>) -> Self {
      let stack = Vec::new();
      
      InOrderIterator { 
         current: Some(root),
         stack
       }
  }
}

impl<'a, T> Iterator for InOrderIterator<'a, T> {
  type Item = &'a T;

  fn next(&mut self) -> Option<Self::Item> {
    while let Some(node) = self.current {
      // Push all left children onto the stack
      self.stack.push(node);
      self.current = node.left.as_ref().map(|n| n.as_ref());
  }

  if let Some(node) = self.stack.pop() {
      // Move to the right child after visiting the current node
      self.current = node.right.as_ref().map(|n| n.as_ref());
      return Some(&node.value);
  }

  None
  }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn in_order_iterator_should_be_increasing() {
      let mut values_iter = vec![5, 3, 4, 1, 3].into_iter();

      let mut root = TreeNode::new(values_iter.next().unwrap());

      for value in values_iter {
        root.insert(value);
        dbg!(&root);
      }
      
      let in_order_iter = root.in_order_iter();

      assert_eq!(in_order_iter.collect::<Vec<_>>(), vec![&1, &3, &3, &4, &5]);
    }
}
